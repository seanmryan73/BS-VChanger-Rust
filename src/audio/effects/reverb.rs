use super::AudioEffect;

const COMB_DELAYS: [usize; 4]   = [1557, 1617, 1491, 1422];
const ALLPASS_DELAYS: [usize; 2] = [225,  556];
const DAMP: f32 = 0.5;

struct CombFilter { buf: Vec<f32>, pos: usize, feedback: f32, damp_store: f32 }
struct AllpassFilter { buf: Vec<f32>, pos: usize }

impl CombFilter {
    fn new(delay: usize, feedback: f32) -> Self {
        Self { buf: vec![0.0; delay], pos: 0, feedback, damp_store: 0.0 }
    }
    fn process(&mut self, input: f32) -> f32 {
        let out = self.buf[self.pos];
        self.damp_store = out * (1.0 - DAMP) + self.damp_store * DAMP;
        self.buf[self.pos] = input + self.damp_store * self.feedback;
        self.pos = (self.pos + 1) % self.buf.len();
        out
    }
}

impl AllpassFilter {
    fn new(delay: usize) -> Self {
        Self { buf: vec![0.0; delay], pos: 0 }
    }
    fn process(&mut self, input: f32) -> f32 {
        let buf_out = self.buf[self.pos];
        let out = -input + buf_out;
        self.buf[self.pos] = input + buf_out * 0.5;
        self.pos = (self.pos + 1) % self.buf.len();
        out
    }
}

pub struct ReverbEffect {
    pub room_size: f32, // 0.0–1.0
    pub wet:       f32,
    combs:    [CombFilter; 4],
    allpasses: [AllpassFilter; 2],
}

impl ReverbEffect {
    pub fn new(room_size: f32, wet: f32) -> Self {
        let feedback = 0.84 + room_size * 0.15;
        Self {
            room_size, wet,
            combs: [
                CombFilter::new(COMB_DELAYS[0], feedback),
                CombFilter::new(COMB_DELAYS[1], feedback),
                CombFilter::new(COMB_DELAYS[2], feedback),
                CombFilter::new(COMB_DELAYS[3], feedback),
            ],
            allpasses: [
                AllpassFilter::new(ALLPASS_DELAYS[0]),
                AllpassFilter::new(ALLPASS_DELAYS[1]),
            ],
        }
    }
}

impl AudioEffect for ReverbEffect {
    fn process(&mut self, samples: &mut [f32], _sample_rate: u32) {
        for s in samples.iter_mut() {
            let input = *s * 0.015;
            let mut sum = 0.0;
            for c in &mut self.combs { sum += c.process(input); }
            let mut out = sum;
            for a in &mut self.allpasses { out = a.process(out); }
            *s = *s * (1.0 - self.wet) + out * self.wet;
        }
    }

    fn name(&self) -> &'static str { "Reverb" }

    fn reset(&mut self) {
        for c in &mut self.combs { c.buf.fill(0.0); c.pos = 0; c.damp_store = 0.0; }
        for a in &mut self.allpasses { a.buf.fill(0.0); a.pos = 0; }
    }
}
