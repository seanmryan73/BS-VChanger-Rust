use cpal::traits::{DeviceTrait, HostTrait};

fn host() -> cpal::Host {
    cpal::default_host()
}

pub fn list_input_devices() -> Vec<String> {
    host()
        .input_devices()
        .into_iter()
        .flatten()
        .filter_map(|d| d.name().ok())
        .collect()
}

pub fn list_output_devices() -> Vec<String> {
    host()
        .output_devices()
        .into_iter()
        .flatten()
        .filter_map(|d| d.name().ok())
        .collect()
}

pub fn find_input_device(name: &str) -> Option<cpal::Device> {
    host()
        .input_devices()
        .ok()?
        .find(|d| d.name().ok().as_deref() == Some(name))
}

pub fn find_output_device(name: &str) -> Option<cpal::Device> {
    host()
        .output_devices()
        .ok()?
        .find(|d| d.name().ok().as_deref() == Some(name))
}
