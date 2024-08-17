use core::f32;
use hound;
use std::error::Error;
use std::f32::consts::PI;
use std::i16;

const SAMPLE_RATE: u32 = 44100;
const DURATION: f32 = 3.0;
const STEP_COUNT: u32 = 15;
const AMPLITUDE: f32 = i16::MAX as f32;

fn main() -> Result<(), Box<dyn Error>> {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create("sine.wav", spec).unwrap();

    let mut last_secs = 0.0;
    let mut last_frequency = f32::NAN;
    let mut last_period_progress = 0.0;
    let mut period_progress_offset = 0.0;
    for secs in (0..(SAMPLE_RATE as f32 * DURATION) as u32).map(|x| x as f32 / SAMPLE_RATE as f32) {
        let t = secs / DURATION;
        let step = (t * STEP_COUNT as f32) as usize;
        const DIATONIC_SCALE: [f32; 7] = [0.0, 2.0, 4.0, 5.0, 7.0, 9.0, 11.0];
        let note_index = 60.0 + DIATONIC_SCALE[step % 7] + (step / 7) as f32 * 12.0;

        let frequency = frequency(note_index as f32);
        if frequency != last_frequency {
            let last_period_progress_if_frequency_is_applied = (last_secs * frequency) % 1.0;
            let current_period_progress_without_offset = (secs * frequency) % 1.0;
            let delta = (current_period_progress_without_offset
                - last_period_progress_if_frequency_is_applied
                + 1.0)
                % 1.0;
            let target_period_progress = (last_period_progress + delta) % 1.0;
            period_progress_offset =
                (target_period_progress - current_period_progress_without_offset + 1.0) % 1.0;
        }
        last_frequency = frequency;
        last_secs = secs;

        let period_progress = (secs * frequency + period_progress_offset) % 1.0;
        last_period_progress = period_progress;

        let sample = sample(period_progress);
        writer.write_sample((sample * AMPLITUDE) as i16).unwrap();
    }

    Ok(())
}

fn sample(period_progress: f32) -> f32 {
    // ((0.9 * (period_progress * 2.0 * PI).sin() + 0.2 * (period_progress * 4.0 * PI).sin()) + 1.0)
    //     .powi(3)
    //     / 4.0
    //     - 1.0
    (period_progress * 2.0 * PI).sin()
}

fn frequency(note_index: f32) -> f32 {
    const A4_FREQUENCY: f32 = 440.0;
    const INDICES_PER_OCTAVE: f32 = 12.0;
    const A4_INDEX: f32 = 69.0;

    A4_FREQUENCY * 2.0f32.powf((note_index - A4_INDEX) / INDICES_PER_OCTAVE)
}
