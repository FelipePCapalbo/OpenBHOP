use macroquad::audio::{load_sound, play_sound, stop_sound, set_sound_volume, PlaySoundParams, Sound};
use std::fs;

pub struct Track {
    pub name: String,
    pub sound: Sound,
    pub duration: f32,
}

pub struct Playlist {
    pub tracks: Vec<Track>,
    pub current_track_index: usize,
    pub playback_time: f32,
    pub current_volume: f32,
}

impl Playlist {
    pub async fn load() -> Self {
        let mut tracks = Vec::new();
        let mut paths: Vec<_> = fs::read_dir("assets/audio/tracks")
            .unwrap()
            .map(|e| e.unwrap().path())
            .filter(|p| p.is_file() && p.extension().map_or(false, |ext| ext == "wav"))
            .collect();
        paths.sort();

        for path in paths {
            let path_str = path.to_str().unwrap().to_string();
            let duration = Self::parse_wav_duration(&fs::read(&path).unwrap());
            let sound = load_sound(&path_str).await.unwrap();
            let name = path
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .strip_suffix(".wav")
                .unwrap()
                .to_string();
            tracks.push(Track { name, sound, duration });
        }

        let playlist = Self {
            tracks,
            current_track_index: 0,
            playback_time: 0.0,
            current_volume: crate::config::MUSIC_BASE_VOLUME,
        };

        playlist.play_current();
        playlist
    }

    fn parse_wav_duration(bytes: &[u8]) -> f32 {
        let mut offset = 12;
        let mut byte_rate = 176400;
        let mut data_size = 0;
        while offset + 8 <= bytes.len() {
            let chunk_id = &bytes[offset..offset + 4];
            let chunk_size = u32::from_le_bytes(bytes[offset + 4..offset + 8].try_into().unwrap()) as usize;
            if chunk_id == b"fmt " {
                byte_rate = u32::from_le_bytes(bytes[offset + 16..offset + 20].try_into().unwrap());
            } else if chunk_id == b"data" {
                data_size = chunk_size;
            }
            offset += 8 + chunk_size;
        }
        data_size as f32 / byte_rate as f32
    }

    fn play_current(&self) {
        let track = &self.tracks[self.current_track_index];
        play_sound(
            &track.sound,
            PlaySoundParams {
                looped: false,
                volume: 0.0,
            },
        );
    }

    pub fn update(&mut self, player_speed: f32, delta_time: f32) {
        self.playback_time += delta_time;

        let track = &self.tracks[self.current_track_index];
        let duration = track.duration;

        let fade_in = (self.playback_time / crate::config::MUSIC_FADE_DURATION).min(1.0);
        let fade_out = ((duration - self.playback_time) / crate::config::MUSIC_FADE_DURATION).clamp(0.0, 1.0);
        let fade_factor = fade_in.min(fade_out);

        let target_volume = if player_speed <= crate::config::MAX_SPEED {
            crate::config::MUSIC_BASE_VOLUME
        } else {
            let t = ((player_speed - crate::config::MAX_SPEED)
                / crate::config::MUSIC_MAX_VOLUME_SPEED)
                .clamp(0.0, 1.0);
            crate::config::MUSIC_BASE_VOLUME
                + (crate::config::MUSIC_MAX_VOLUME - crate::config::MUSIC_BASE_VOLUME) * t
        };

        self.current_volume += (target_volume - self.current_volume)
            * delta_time
            * crate::config::MUSIC_VOLUME_LERP_SPEED;

        set_sound_volume(&track.sound, self.current_volume * fade_factor);

        if self.playback_time >= duration {
            stop_sound(&track.sound);
            self.current_track_index = (self.current_track_index + 1) % self.tracks.len();
            self.playback_time = 0.0;
            self.play_current();
        }
    }

    pub fn current_track_name(&self) -> Option<&str> {
        Some(self.tracks[self.current_track_index].name.as_str())
    }
}
