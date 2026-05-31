use macroquad::audio::{load_sound_from_bytes, play_sound, PlaySoundParams, Sound};
use macroquad::file::load_file;
use std::f32::consts::FRAC_PI_2;

pub struct JumpAudio {
    sounds: Vec<Sound>,
    steps_per_octave: usize,
}

impl JumpAudio {
    pub async fn load() -> Self {
        let original_bytes = load_file("assets/audio/jump.wav").await.unwrap();
        let mut sounds = Vec::new();

        let steps_per_octave = 20;
        let total_steps = steps_per_octave * 2;

        for i in 0..=total_steps {
            let pitch = 0.5 * 2.0_f32.powf(i as f32 / steps_per_octave as f32);
            let modified_bytes = Self::modify_wav_pitch(&original_bytes, pitch);
            let sound = load_sound_from_bytes(&modified_bytes).await.unwrap();
            sounds.push(sound);
        }

        Self {
            sounds,
            steps_per_octave,
        }
    }

    pub fn play(&self, speed: f32) {
        let cycle_rate = 0.05;
        let phase = (speed.abs() * cycle_rate).fract();

        let sub_index = (phase * self.steps_per_octave as f32).round() as usize;
        let base_index = sub_index + self.steps_per_octave;

        let vol_sub = (phase * FRAC_PI_2).sin();
        let vol_base = (phase * FRAC_PI_2).cos();

        play_sound(
            &self.sounds[sub_index],
            PlaySoundParams {
                looped: false,
                volume: vol_sub,
            },
        );

        play_sound(
            &self.sounds[base_index],
            PlaySoundParams {
                looped: false,
                volume: vol_base,
            },
        );
    }

    fn modify_wav_pitch(wav_bytes: &[u8], pitch_multiplier: f32) -> Vec<u8> {
        let mut modified = wav_bytes.to_vec();

        let original_sample_rate = u32::from_le_bytes(modified[24..28].try_into().unwrap());
        let new_sample_rate = (original_sample_rate as f32 * pitch_multiplier) as u32;
        modified[24..28].copy_from_slice(&new_sample_rate.to_le_bytes());

        let original_byte_rate = u32::from_le_bytes(modified[28..32].try_into().unwrap());
        let bytes_per_sample = original_byte_rate as f32 / original_sample_rate as f32;
        let new_byte_rate = (new_sample_rate as f32 * bytes_per_sample) as u32;
        modified[28..32].copy_from_slice(&new_byte_rate.to_le_bytes());

        modified
    }
}
