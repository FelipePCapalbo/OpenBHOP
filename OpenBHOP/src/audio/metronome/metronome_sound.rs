use macroquad::audio::{load_sound_from_bytes, play_sound, PlaySoundParams, Sound};
use crate::config::METRONOME_VOLUME;

const TICK_DURATION_S: f32 = 0.015;
const TICK_FREQ_MAIN: f32 = 1500.0;
const TICK_FREQ_SUB: f32 = 900.0;
const SAMPLE_RATE: u32 = 44_100;

/// Carrega e toca os dois sons do metrônomo (downbeat e subdivisão).
pub struct MetronomeSound {
    tick_main: Sound,
    tick_sub: Sound,
}

impl MetronomeSound {
    pub async fn load() -> Self {
        let tick_main = load_sound_from_bytes(&Self::build_wav(TICK_FREQ_MAIN)).await.unwrap();
        let tick_sub  = load_sound_from_bytes(&Self::build_wav(TICK_FREQ_SUB)).await.unwrap();

        Self { tick_main, tick_sub }
    }

    pub fn play(&self, is_downbeat: bool) {
        let sound = if is_downbeat { &self.tick_main } else { &self.tick_sub };
        play_sound(sound, PlaySoundParams { looped: false, volume: METRONOME_VOLUME });
    }

    /// Gera uma onda senoidal amortecida como WAV PCM 16-bit mono.
    fn build_wav(frequency: f32) -> Vec<u8> {
        let num_samples = (TICK_DURATION_S * SAMPLE_RATE as f32) as usize;
        let mut pcm: Vec<i16> = Vec::with_capacity(num_samples);

        for i in 0..num_samples {
            let t = i as f32 / SAMPLE_RATE as f32;
            let envelope = (-t / (TICK_DURATION_S * 0.3)).exp();
            let sample =
                (envelope * (2.0 * std::f32::consts::PI * frequency * t).sin()) * i16::MAX as f32;
            pcm.push(sample as i16);
        }

        Self::pcm_to_wav(&pcm, SAMPLE_RATE)
    }

    fn pcm_to_wav(samples: &[i16], sample_rate: u32) -> Vec<u8> {
        let num_channels: u16 = 1;
        let bits_per_sample: u16 = 16;
        let byte_rate = sample_rate * num_channels as u32 * bits_per_sample as u32 / 8;
        let block_align: u16 = num_channels * bits_per_sample / 8;
        let data_size = (samples.len() * 2) as u32;
        let chunk_size = 36 + data_size;

        let mut wav = Vec::with_capacity(44 + samples.len() * 2);

        wav.extend_from_slice(b"RIFF");
        wav.extend_from_slice(&chunk_size.to_le_bytes());
        wav.extend_from_slice(b"WAVE");
        wav.extend_from_slice(b"fmt ");
        wav.extend_from_slice(&16u32.to_le_bytes());
        wav.extend_from_slice(&1u16.to_le_bytes());
        wav.extend_from_slice(&num_channels.to_le_bytes());
        wav.extend_from_slice(&sample_rate.to_le_bytes());
        wav.extend_from_slice(&byte_rate.to_le_bytes());
        wav.extend_from_slice(&block_align.to_le_bytes());
        wav.extend_from_slice(&bits_per_sample.to_le_bytes());
        wav.extend_from_slice(b"data");
        wav.extend_from_slice(&data_size.to_le_bytes());
        for &s in samples {
            wav.extend_from_slice(&s.to_le_bytes());
        }

        wav
    }
}
