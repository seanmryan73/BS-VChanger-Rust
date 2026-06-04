use std::sync::Arc;
use parking_lot::Mutex;
use cpal::{Stream, StreamConfig, SampleFormat, SupportedStreamConfig, traits::*};
use ringbuf::{HeapRb, traits::{Producer, Consumer, Split}};

use crate::audio::effects::EffectChain;
use crate::audio::spectrum::SpectrumBuffer;
use super::devices;

const RING_BUF_SAMPLES: usize = 16_384;
const PREFERRED_RATES:   &[u32] = &[48_000, 44_100, 16_000, 22_050, 96_000];
const PREFERRED_FORMATS: &[SampleFormat] = &[SampleFormat::F32, SampleFormat::I16, SampleFormat::U16];

pub struct StartConfig {
    pub input_name:   String,
    pub monitor_name: Option<String>,
    pub virtual_name: Option<String>,
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
        let chain  = Arc::clone(&effect_chain);
        let err_in = Arc::clone(&last_error);

        macro_rules! input_cb {
            ($ty:ty, $to_f32:expr) => {{
                let spec = spectrum.clone();
                move |data: &[$ty], _: &cpal::InputCallbackInfo| {
                    let mut mono: Vec<f32> = if in_channels == 1 {
                        data.iter().map($to_f32).collect()
                    } else {
                        data.chunks(in_channels)
                            .map(|ch| ch.iter().map($to_f32).sum::<f32>() / in_channels as f32)
                            .collect()
                    };
                    chain.lock().process(&mut mono, sample_rate);
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

        in_stream.play().map_err(|e| e.to_string())?;
        streams.push(in_stream);

        // ── Output streams ────────────────────────────────────────────────────
        if let Some(name) = &config.monitor_name {
            streams.push(build_output_stream(name, mon_cons, Arc::clone(&last_error))?);
        }
        if let Some(name) = &config.virtual_name {
            streams.push(build_output_stream(name, virt_cons, Arc::clone(&last_error))?);
        }

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

fn probe_output_config(dev: &cpal::Device) -> Result<(StreamConfig, SampleFormat), String> {
    for (cfg, fmt) in output_candidates(dev) {
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
    candidates_from(
        dev.default_input_config().ok(),
        dev.supported_input_configs().ok(),
    )
}

fn output_candidates(dev: &cpal::Device) -> Vec<(StreamConfig, SampleFormat)> {
    candidates_from(
        dev.default_output_config().ok(),
        dev.supported_output_configs().ok(),
    )
}

fn candidates_from(
    default: Option<SupportedStreamConfig>,
    supported: Option<impl Iterator<Item = cpal::SupportedStreamConfigRange>>,
) -> Vec<(StreamConfig, SampleFormat)> {
    let mut out: Vec<(StreamConfig, SampleFormat)> = Vec::new();

    // Default config first
    if let Some(cfg) = default {
        let fmt = cfg.sample_format();
        if PREFERRED_FORMATS.contains(&fmt) {
            out.push((to_stream_config(&cfg), fmt));
        }
    }

    // Enumerated configs: iterate format preference × rate preference
    if let Some(ranges) = supported {
        let ranges: Vec<_> = ranges.collect();
        for &fmt in PREFERRED_FORMATS {
            for &rate in PREFERRED_RATES {
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
                        // Skip exact duplicates
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

// ── Output stream builder ─────────────────────────────────────────────────────

fn build_output_stream(
    device_name: &str,
    mut cons: impl Consumer<Item = f32> + Send + 'static,
    last_error: Arc<Mutex<Option<String>>>,
) -> Result<Stream, String> {
    let dev = devices::find_output_device(device_name)
        .ok_or_else(|| format!("Output device '{device_name}' not found"))?;

    // Probe finds the first config that WASAPI will actually open.
    let (cfg, fmt) = probe_output_config(&dev)
        .map_err(|e| format!("Output '{device_name}': {e}"))?;

    let channels = cfg.channels as usize;
    let err      = Arc::clone(&last_error);

    let stream = match fmt {
        SampleFormat::F32 => dev.build_output_stream(
            &cfg,
            move |data: &mut [f32], _| { fill_f32(data, channels, &mut cons); },
            make_err_cb(err), None,
        ),
        SampleFormat::I16 => dev.build_output_stream(
            &cfg,
            move |data: &mut [i16], _| { fill_i16(data, channels, &mut cons); },
            make_err_cb(err), None,
        ),
        SampleFormat::U16 => dev.build_output_stream(
            &cfg,
            move |data: &mut [u16], _| { fill_u16(data, channels, &mut cons); },
            make_err_cb(err), None,
        ),
        fmt => return Err(format!("Output '{device_name}': unsupported format {fmt:?}")),
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
