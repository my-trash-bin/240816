use hound;
use std::error::Error;
use std::f32::consts::PI;
use std::i16;

const SAMPLE_RATE: u32 = 44100;
const DURATION: f32 = 3.0;
const STEP_COUNT: u32 = 15;

fn main() -> Result<(), Box<dyn Error>> {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create("sine.wav", spec).unwrap();

    for secs in (0..(SAMPLE_RATE as f32 * DURATION) as u32).map(|x| x as f32 / SAMPLE_RATE as f32) {
        let t = secs / DURATION;
        let step = (t * STEP_COUNT as f32) as usize;
        let hz = hz((60.0
            + [0.0, 2.0, 4.0, 5.0, 7.0, 9.0, 11.0][step % 7]
            + (step / 7) as f32 * 12.0) as f32);
        let sample = (secs * hz * 2.0 * PI).sin();
        let amplitude = i16::MAX as f32;
        writer.write_sample((sample * amplitude) as i16).unwrap();
    }

    Ok(())
}

fn hz(note_index: f32) -> f32 {
    const A4_HZ: f32 = 440.0;
    const INDICES_PER_OCTAVE: f32 = 12.0;
    const A4_INDEX: f32 = 69.0;

    A4_HZ * 2.0f32.powf((note_index - A4_INDEX) / INDICES_PER_OCTAVE)
}
