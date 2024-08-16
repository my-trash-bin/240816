use hound;
use std::error::Error;
use std::f32::consts::PI;
use std::i16;

fn main() -> Result<(), Box<dyn Error>> {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create("sine.wav", spec).unwrap();

    for t in (0..44100).map(|x| x as f32 / 44100.0) {
        let hz = 440.0;
        let step = (t * 8.0) as usize;
        let hz = hz
            * f32::powf(
                2.0,
                ([0.0, 2.0, 4.0, 5.0, 7.0, 9.0, 11.0][step % 7] + 12.0 * (step / 7) as f32) / 12.0,
            );
        let sample = (t * hz * 2.0 * PI).sin();
        let amplitude = i16::MAX as f32;
        writer.write_sample((sample * amplitude) as i16).unwrap();
    }

    Ok(())
}
