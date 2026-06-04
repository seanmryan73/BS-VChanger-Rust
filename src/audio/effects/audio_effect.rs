pub trait AudioEffect: Send {
    fn process(&mut self, samples: &mut [f32], sample_rate: u32);
    fn name(&self) -> &'static str;
    fn reset(&mut self);
}
