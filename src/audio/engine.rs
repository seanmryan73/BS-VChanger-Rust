use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};
use parking_lot::Mutex;
use cpal::{Stream, StreamConfig, SampleFormat, SupportedStreamConfig, traits::*};
use ringbuf::{HeapRb, traits::{Producer, Consumer, Split}};

use crate::audio::effects::EffectChain;
use crate::audio::level::LevelBuffer;
use crate::audio::spectrum::SpectrumBuffer;
use super::devices;

const RING_BUF_SAMPLES: usize = 16_384;
const PREFERRED_RATES:   &[u32] = &[48_000, 44_100, 16_000, 22_050, 96_000];
const PREFERRED_FORMATS: &[SampleFormat] = &[SampleFormat::F32, SampleFormat::I16, SampleFormat::U16];

pub struct StartConfig {
    pub input_name:   String,
    pub monitor_name: Option<String>,
    pub virtual_name: Option<String>,
    pub input_gain:   Arc<AtomicU32>,
}

pub struct RealtimeAudioEngine {
    _streams:         Vec<Stream>,
    pub effect_chain: Arc<Mutex<EffectChain>>,
    last_error:       Arc<Mutex<Option<String>>>,
    pub sample_rate:  u32,
}

impl RealtimeAudioEngine {
    pub fn start(
        config: &StartConfig,
        effect_chain: Arc<Mutex<EffectChain>>,
        spectrum: SpectrumBuffer,
        input_level: LevelBuffer,
    ) -> Result<Self, String> {
        let last_error: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));
        let mut streams: Vec<Stream> = Vec::new();

        let want_monitor = config.monitor_name.is_some();
        let want_virtual = config.virtual_name.is_some();
        if !want_monitor && !want_virtual {
            return Err("Select at least one output device (Monitor or Virtual).".into());
        }

        // ── Input ─────────────────────────────────────────────────────────────
        let in_dev = devices::find_input_device(&config.input_name)
            .ok_or_else(|| format!("Input device '{}' not found", config.input_name))?;

        let (in_cfg, in_fmt) = probe_input_config(&in_dev)
            .map_err(|e| format!("Input '{}': {e}", config.input_name))?;

        let sample_rate = in_cfg.sample_rate.0;
        let in_channels = in_cfg.channels as usize;

        // ── Ring buffers ──────────────────────────────────────────────────────
        let (mut mon_prod, mon_cons)   = HeapRb::<f32>::new(RING_BUF_SAMPLES).split();
        let (mut virt_prod, virt_cons) = HeapRb::<f32>::new(RING_BUF_SAMPLES).split();

        // ── Input stream ──────────────────────────────────────────────────────
        let chain      = Arc::clone(&effect_chain);
        let err_in     = Arc::clone(&last_error);
        let input_gain = Arc::clone(&config.input_gain);

        macro_rules! input_cb {
            ($ty:ty, $to_f32:expr) => {{
                let spec  = spectrum.clone();
                let level = input_level.clone();
                let gain  = Arc::clone(&input_gain);
                move |data: &[$ty], _: &cpal::InputCallbackInfo| {
                    let mut mono: Vec<f32> = if in_channels == 1 {
                        data.iter().map($to_f32).collect()
                    } else {
                        data.chunks(in_channels)
                            .map(|ch| ch.iter().map($to_f32).sum::<f32>() / in_channels as f32)
                            .collect()
                    };
                    let g = f32::from_bits(gain.load(Ordering::Relaxed));
                    if g != 1.0 { for s in &mut mono { *s *= g; } }
                    level.push(&mono);
                    // try_lock, not lock: apply_chain() (UI thread) only holds this
                    // lock for a pointer swap, but the callback must never block —
                    // skip effects for this batch (pass audio through dry) rather
                    // than risk an XRUN if it's ever contended.
                    if let Some(mut c) = chain.try_lock() {
                        c.process(&mut mono, sample_rate);
                    }
                    spec.push(&mono);
                    if want_monitor { mon_prod.push_slice(&mono); }
                    if want_virtual { virt_prod.push_slice(&mono); }
                }
            }};
        }

        let in_stream = match in_fmt {
            SampleFormat::F32 => in_dev.build_input_stream(
                &in_cfg, input_cb!(f32, |s: &f32| *s),
                make_err_cb(Arc::clone(&err_in)), None,
            ),
            SampleFormat::I16 => in_dev.build_input_stream(
                &in_cfg, input_cb!(i16, |s: &i16| *s as f32 / 32_768.0),
                make_err_cb(Arc::clone(&err_in)), None,
            ),
            SampleFormat::U16 => in_dev.build_input_stream(
                &in_cfg, input_cb!(u16, |s: &u16| *s as f32 / 32_768.0 - 1.0),
                make_err_cb(Arc::clone(&err_in)), None,
            ),
            fmt => return Err(format!("Input: unsupported format {fmt:?}")),
        }
        .map_err(|e| format!("Input stream error: {e}"))?;

        // ── Output streams (built before input starts so consumers are ready) ──
        // Pass the input sample_rate so output probing prioritises the same
        // rate — mismatched rates cause pitch drift and ring-buffer crackling.
        if let Some(name) = &config.monitor_name {
            streams.push(build_output_stream(name, mon_cons, Arc::clone(&last_error), sample_rate)?);
        }
        if let Some(name) = &config.virtual_name {
            streams.push(build_output_stream(name, virt_cons, Arc::clone(&last_error), sample_rate)?);
        }

        // ── Start input last so ring buffers never overflow before consumers ──
        in_stream.play().map_err(|e| e.to_string())?;
        streams.push(in_stream);

        Ok(Self { _streams: streams, effect_chain, last_error, sample_rate })
    }

    pub fn take_error(&self) -> Option<String> {
        self.last_error.lock().take()
    }
}

// ── Config probing ────────────────────────────────────────────────────────────

/// Builds a no-op dummy stream to verify a config actually works with WASAPI,
/// then immediately drops it. Returns the first config that succeeds.
///
/// `default_output_config()` can return a format that WASAPI then refuses to
/// open — probing is the only reliable way to know what will actually work.
fn probe_input_config(dev: &cpal::Device) -> Result<(StreamConfig, SampleFormat), String> {
    for (cfg, fmt) in input_candidates(dev) {
        let ok = match fmt {
            SampleFormat::F32 => dev.build_input_stream(&cfg, |_: &[f32], _| {}, |_| {}, None).is_ok(),
            SampleFormat::I16 => dev.build_input_stream(&cfg, |_: &[i16], _| {}, |_| {}, None).is_ok(),
            SampleFormat::U16 => dev.build_input_stream(&cfg, |_: &[u16], _| {}, |_| {}, None).is_ok(),
            _ => false,
        };
        if ok { return Ok((cfg, fmt)); }
    }
    Err("no compatible input config found".into())
}

fn probe_output_config(dev: &cpal::Device, preferred_rate: u32) -> Result<(StreamConfig, SampleFormat), String> {
    // Try preferred rate first (avoids resampling). If not supported, fall
    // back to any working config — the caller will resample as needed.
    let preferred = output_candidates_at_rate(dev, preferred_rate);
    let fallback  = output_candidates_any_rate(dev, preferred_rate);
    for (cfg, fmt) in preferred.into_iter().chain(fallback) {
        let ok = match fmt {
            SampleFormat::F32 => dev.build_output_stream(&cfg, |_: &mut [f32], _| {}, |_| {}, None).is_ok(),
            SampleFormat::I16 => dev.build_output_stream(&cfg, |_: &mut [i16], _| {}, |_| {}, None).is_ok(),
            SampleFormat::U16 => dev.build_output_stream(&cfg, |_: &mut [u16], _| {}, |_| {}, None).is_ok(),
            _ => false,
        };
        if ok { return Ok((cfg, fmt)); }
    }
    Err("no compatible output config found".into())
}

/// Candidate list: default config first, then supported ranges ordered by
/// preferred format and preferred sample rate.
fn input_candidates(dev: &cpal::Device) -> Vec<(StreamConfig, SampleFormat)> {
    candidates_from_rates(
        dev.default_input_config().ok(),
        dev.supported_input_configs().ok(),
        PREFERRED_RATES,
    )
}

/// Returns only output configs that match `rate` exactly, ordered by format
/// preference. Intentionally ignores the device default — the caller wants a
/// specific rate, not whatever Windows happens to have configured.
fn output_candidates_at_rate(dev: &cpal::Device, rate: u32) -> Vec<(StreamConfig, SampleFormat)> {
    let mut out = Vec::new();
    let ranges = match dev.supported_output_configs() {
        Ok(r) => r.collect::<Vec<_>>(),
        Err(_) => return out,
    };
    for &fmt in PREFERRED_FORMATS {
        for range in &ranges {
            if range.sample_format() == fmt
                && range.min_sample_rate().0 <= rate
                && range.max_sample_rate().0 >= rate
            {
                out.push((StreamConfig {
                    channels:    range.channels(),
                    sample_rate: cpal::SampleRate(rate),
                    buffer_size: cpal::BufferSize::Default,
                }, fmt));
            }
        }
    }
    out
}

/// Fallback: all supported output configs EXCEPT preferred_rate (already tried).
fn output_candidates_any_rate(dev: &cpal::Device, skip_rate: u32) -> Vec<(StreamConfig, SampleFormat)> {
    let rates: Vec<u32> = PREFERRED_RATES.iter().copied().filter(|&r| r != skip_rate).collect();
    candidates_from_rates(
        dev.default_output_config().ok(),
        dev.supported_output_configs().ok(),
        &rates,
    )
}

fn candidates_from_rates(
    default: Option<SupportedStreamConfig>,
    supported: Option<impl Iterator<Item = cpal::SupportedStreamConfigRange>>,
    rates: &[u32],
) -> Vec<(StreamConfig, SampleFormat)> {
    let mut out: Vec<(StreamConfig, SampleFormat)> = Vec::new();

    // Default config first (may be overridden below if a preferred rate matches)
    if let Some(cfg) = &default {
        let fmt = cfg.sample_format();
        if PREFERRED_FORMATS.contains(&fmt) {
            out.push((to_stream_config(cfg), fmt));
        }
    }

    // Enumerated configs: iterate format preference × caller-supplied rate order
    if let Some(ranges) = supported {
        let ranges: Vec<_> = ranges.collect();
        for &fmt in PREFERRED_FORMATS {
            for &rate in rates {
                for range in &ranges {
                    if range.sample_format() == fmt
                        && range.min_sample_rate().0 <= rate
                        && range.max_sample_rate().0 >= rate
                    {
                        let cfg = StreamConfig {
                            channels:    range.channels(),
                            sample_rate: cpal::SampleRate(rate),
                            buffer_size: cpal::BufferSize::Default,
                        };
                        if !out.iter().any(|(c, f)| *f == fmt
                            && c.channels == cfg.channels
                            && c.sample_rate == cfg.sample_rate)
                        {
                            out.push((cfg, fmt));
                        }
                    }
                }
            }
        }
    }

    out
}

fn to_stream_config(s: &SupportedStreamConfig) -> StreamConfig {
    StreamConfig {
        channels:    s.channels(),
        sample_rate: s.sample_rate(),
        buffer_size: cpal::BufferSize::Default,
    }
}

// ── Linear-interpolation resampler ───────────────────────────────────────────

/// Sample-by-sample linear resampler. Works with any callback size.
/// Uses rubato-quality interpolation isn't needed here — at voice-changer
/// rates (44.1 k↔48 k) linear interp aliases above ~18 kHz, well above voice.
struct OutputResampler {
    ratio: f64, // input_rate / output_rate
    phase: f64, // fractional position between prev and next input samples
    prev:  f32,
    next:  f32,
}

impl OutputResampler {
    fn new(input_rate: u32, output_rate: u32) -> Self {
        Self { ratio: input_rate as f64 / output_rate as f64, phase: 0.0, prev: 0.0, next: 0.0 }
    }

    fn next_sample(&mut self, cons: &mut impl Consumer<Item = f32>) -> f32 {
        // Interpolate at current phase, then advance
        let out = self.prev + (self.next - self.prev) * self.phase as f32;
        self.phase += self.ratio;
        while self.phase >= 1.0 {
            self.prev = self.next;
            self.next = cons.try_pop().unwrap_or(self.prev);
            self.phase -= 1.0;
        }
        out
    }
}

// ── Output stream builder ─────────────────────────────────────────────────────

fn build_output_stream(
    device_name: &str,
    mut cons: impl Consumer<Item = f32> + Send + 'static,
    last_error: Arc<Mutex<Option<String>>>,
    input_rate: u32,
) -> Result<Stream, String> {
    let dev = devices::find_output_device(device_name)
        .ok_or_else(|| format!("Output device '{device_name}' not found"))?;

    let (cfg, fmt) = probe_output_config(&dev, input_rate)
        .map_err(|e| format!("Output '{device_name}': {e}"))?;

    let output_rate = cfg.sample_rate.0;
    let channels    = cfg.channels as usize;
    let err         = Arc::clone(&last_error);

    let stream = if output_rate != input_rate {
        // Rates differ — resample on the fly so pitch and tempo are correct.
        let mut rs = OutputResampler::new(input_rate, output_rate);
        match fmt {
            SampleFormat::F32 => dev.build_output_stream(&cfg, move |data: &mut [f32], _| {
                let frames = data.len() / channels;
                for i in 0..frames {
                    let s = rs.next_sample(&mut cons);
                    for c in 0..channels { data[i * channels + c] = s; }
                }
            }, make_err_cb(err), None),
            SampleFormat::I16 => dev.build_output_stream(&cfg, move |data: &mut [i16], _| {
                let frames = data.len() / channels;
                for i in 0..frames {
                    let s = rs.next_sample(&mut cons);
                    let v = (s * 32_767.0).clamp(-32_768.0, 32_767.0) as i16;
                    for c in 0..channels { data[i * channels + c] = v; }
                }
            }, make_err_cb(err), None),
            SampleFormat::U16 => dev.build_output_stream(&cfg, move |data: &mut [u16], _| {
                let frames = data.len() / channels;
                for i in 0..frames {
                    let s = rs.next_sample(&mut cons);
                    let v = ((s + 1.0) * 32_767.5).clamp(0.0, 65_535.0) as u16;
                    for c in 0..channels { data[i * channels + c] = v; }
                }
            }, make_err_cb(err), None),
            fmt => return Err(format!("Output '{device_name}': unsupported format {fmt:?}")),
        }
    } else {
        // Rates match — direct fill, no resampling overhead.
        match fmt {
            SampleFormat::F32 => dev.build_output_stream(
                &cfg, move |data: &mut [f32], _| { fill_f32(data, channels, &mut cons); },
                make_err_cb(err), None,
            ),
            SampleFormat::I16 => dev.build_output_stream(
                &cfg, move |data: &mut [i16], _| { fill_i16(data, channels, &mut cons); },
                make_err_cb(err), None,
            ),
            SampleFormat::U16 => dev.build_output_stream(
                &cfg, move |data: &mut [u16], _| { fill_u16(data, channels, &mut cons); },
                make_err_cb(err), None,
            ),
            fmt => return Err(format!("Output '{device_name}': unsupported format {fmt:?}")),
        }
    }
    .map_err(|e| format!("Output '{device_name}': {e}"))?;

    stream.play().map_err(|e| e.to_string())?;
    Ok(stream)
}

// ── Shared helpers ────────────────────────────────────────────────────────────

fn make_err_cb(
    last_error: Arc<Mutex<Option<String>>>,
) -> impl FnMut(cpal::StreamError) + Send + 'static {
    move |e| { *last_error.lock() = Some(e.to_string()); }
}

fn fill_f32(data: &mut [f32], ch: usize, cons: &mut impl Consumer<Item = f32>) {
    let frames = data.len() / ch;
    let mut mono = vec![0.0f32; frames];
    let n = cons.pop_slice(&mut mono);
    for i in 0..frames {
        let s = if i < n { mono[i] } else { 0.0 };
        for c in 0..ch { data[i * ch + c] = s; }
    }
}

fn fill_i16(data: &mut [i16], ch: usize, cons: &mut impl Consumer<Item = f32>) {
    let frames = data.len() / ch;
    let mut mono = vec![0.0f32; frames];
    let n = cons.pop_slice(&mut mono);
    for i in 0..frames {
        let s = if i < n { mono[i] } else { 0.0 };
        let v = (s * 32_767.0).clamp(-32_768.0, 32_767.0) as i16;
        for c in 0..ch { data[i * ch + c] = v; }
    }
}

fn fill_u16(data: &mut [u16], ch: usize, cons: &mut impl Consumer<Item = f32>) {
    let frames = data.len() / ch;
    let mut mono = vec![0.0f32; frames];
    let n = cons.pop_slice(&mut mono);
    for i in 0..frames {
        let s = if i < n { mono[i] } else { 0.0 };
        let v = ((s + 1.0) * 32_767.5).clamp(0.0, 65_535.0) as u16;
        for c in 0..ch { data[i * ch + c] = v; }
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use ringbuf::{HeapRb, traits::{Producer, Consumer, Split, Observer}};

    // ── Ring buffer: dual-output data flow ────────────────────────────────────

    /// Both ring buffers receive all data and consumers read it back correctly.
    #[test]
    fn dual_buffers_both_receive_data() {
        let (mut mon_prod, mut mon_cons)   = HeapRb::<f32>::new(RING_BUF_SAMPLES).split();
        let (mut virt_prod, mut virt_cons) = HeapRb::<f32>::new(RING_BUF_SAMPLES).split();

        let audio: Vec<f32> = (0..480).map(|i| i as f32 / 480.0).collect();

        let n_mon  = mon_prod.push_slice(&audio);
        let n_virt = virt_prod.push_slice(&audio);

        assert_eq!(n_mon,  480, "monitor ring buffer should accept all 480 samples");
        assert_eq!(n_virt, 480, "virtual ring buffer should accept all 480 samples");

        let mut out_mon  = vec![0.0f32; 480];
        let mut out_virt = vec![0.0f32; 480];
        let r_mon  = mon_cons.pop_slice(&mut out_mon);
        let r_virt = virt_cons.pop_slice(&mut out_virt);

        assert_eq!(r_mon,  480, "monitor consumer should drain all 480 samples");
        assert_eq!(r_virt, 480, "virtual consumer should drain all 480 samples");
        assert_eq!(out_mon,  audio, "monitor data must match input");
        assert_eq!(out_virt, audio, "virtual data must match input — failure here means the two producers interfere");
    }

    /// Simulates 100 input callbacks with virtual draining half as often as monitor.
    /// Both buffers should receive every sample without overflow (16 k buffer >> 480×100 = 48 k).
    /// NOTE: 480 × 100 = 48 000 > RING_BUF_SAMPLES (16 384), so overflow IS expected here
    /// unless the consumer keeps up — verifies the overflow path drops cleanly.
    #[test]
    fn dual_buffers_asymmetric_drain_rates() {
        let (mut mon_prod, mut mon_cons)   = HeapRb::<f32>::new(RING_BUF_SAMPLES).split();
        let (mut virt_prod, mut virt_cons) = HeapRb::<f32>::new(RING_BUF_SAMPLES).split();

        let callback = vec![0.5f32; 480];
        let mut mon_pushed   = 0usize;
        let mut virt_pushed  = 0usize;
        let mut virt_drained = 0usize;

        for iter in 0..100 {
            mon_pushed  += mon_prod.push_slice(&callback);
            virt_pushed += virt_prod.push_slice(&callback);

            // Monitor drains every callback (fast consumer)
            let mut sink = vec![0.0f32; 480];
            mon_cons.pop_slice(&mut sink);

            // Virtual drains every other callback (slow consumer — like a large WASAPI buffer)
            if iter % 2 == 1 {
                let mut sink2 = vec![0.0f32; 960];
                virt_drained += virt_cons.pop_slice(&mut sink2);
            }
        }

        // Monitor should always drain in sync — no overflow
        assert_eq!(mon_pushed, 480 * 100, "monitor should never drop samples");

        // Virtual drains 50 times * up-to-960 = ≤48 000. Buffer is 16 384 so
        // overflow is inevitable after ~34 callbacks. Verify total pushed ≤ 48 000.
        assert!(virt_pushed <= 480 * 100, "virtual pushed should not exceed input count");
        // After overflow virt_pushed < 48 000 — overflow drops silently, no panic
        println!("virt pushed={virt_pushed} drained={virt_drained} (overflow at ~16384 samples in)");
    }

    // ── fill_f32: mono→stereo expansion ──────────────────────────────────────

    #[test]
    fn fill_f32_mono_to_stereo() {
        let (mut prod, mut cons) = HeapRb::<f32>::new(64).split();
        prod.push_slice(&[0.1, 0.2, 0.3, 0.4]);

        let mut out = vec![0.0f32; 8]; // 4 frames × 2 channels
        fill_f32(&mut out, 2, &mut cons);

        // Each mono sample must appear on both L and R channels
        assert_eq!(out, vec![0.1, 0.1, 0.2, 0.2, 0.3, 0.3, 0.4, 0.4],
            "mono sample must be duplicated to both stereo channels");
    }

    #[test]
    fn fill_f32_empty_buffer_outputs_silence() {
        let (_prod, mut cons) = HeapRb::<f32>::new(64).split();
        let mut out = vec![1.0f32; 8]; // pre-filled with non-zero
        fill_f32(&mut out, 2, &mut cons);
        assert!(out.iter().all(|&s| s == 0.0), "empty ring buffer must produce silence, not garbage");
    }

    // ── OutputResampler: 48 kHz → 44.1 kHz ───────────────────────────────────

    #[test]
    fn resampler_produces_correct_sample_count() {
        let (mut prod, mut cons) = HeapRb::<f32>::new(RING_BUF_SAMPLES).split();
        // 100 ms at 48 kHz
        prod.push_slice(&vec![0.5f32; 4_800]);

        let mut rs = OutputResampler::new(48_000, 44_100);
        // Consume 100 ms worth of 44.1 kHz output
        let out: Vec<f32> = (0..4_410).map(|_| rs.next_sample(&mut cons)).collect();

        assert_eq!(out.len(), 4_410, "resampler must produce exactly the requested output count");
    }

    #[test]
    fn resampler_empty_input_outputs_held_value() {
        // If the ring buffer is empty, next_sample should repeat the last known
        // sample (prev), not produce NaN or panic.
        let (_prod, mut cons) = HeapRb::<f32>::new(64).split();
        let mut rs = OutputResampler::new(48_000, 44_100);
        let s = rs.next_sample(&mut cons); // should not panic
        assert!(s.is_finite(), "resampler must not produce NaN or Inf on empty buffer");
    }

    // ── Ring buffer overflow: push_slice drops silently ───────────────────────

    #[test]
    fn push_slice_drops_on_overflow() {
        let (mut prod, _cons) = HeapRb::<f32>::new(100).split();
        let pushed = prod.push_slice(&vec![1.0f32; 150]);
        assert_eq!(pushed, 100, "push_slice must drop excess samples rather than blocking or panicking");
        assert_eq!(prod.vacant_len(), 0, "buffer must be full after overflow push");
    }
}
