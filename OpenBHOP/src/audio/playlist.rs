use macroquad::audio::{load_sound, play_sound, set_sound_volume, stop_sound, PlaySoundParams, Sound};
use std::{error::Error, fmt, fs, io, path::PathBuf};

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

#[derive(Debug)]
pub enum PlaylistLoadError {
    ReadTracksDir { source: io::Error },
    ReadTrackDirEntry { source: io::Error },
    EmptyTracksDir,
    InvalidTrackPath { path: PathBuf },
    ReadTrackFile { path: PathBuf, source: io::Error },
    InvalidWav { path: PathBuf },
    LoadSound { path: PathBuf, message: String },
}

impl fmt::Display for PlaylistLoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ReadTracksDir { source } => write!(f, "failed to read tracks directory: {source}"),
            Self::ReadTrackDirEntry { source } => write!(f, "failed to read a track directory entry: {source}"),
            Self::EmptyTracksDir => write!(f, "tracks directory is empty or contains no valid WAV files"),
            Self::InvalidTrackPath { path } => write!(f, "invalid track path: {}", path.display()),
            Self::ReadTrackFile { path, source } => write!(f, "failed to read track file {}: {source}", path.display()),
            Self::InvalidWav { path } => write!(f, "invalid or unsupported WAV file: {}", path.display()),
            Self::LoadSound { path, message } => write!(f, "failed to load sound {}: {message}", path.display()),
        }
    }
}

impl Error for PlaylistLoadError {}

impl Playlist {
    pub async fn load() -> Result<Self, PlaylistLoadError> {
        let mut paths = Vec::new();
        let read_dir = fs::read_dir("assets/audio/tracks")
            .map_err(|source| PlaylistLoadError::ReadTracksDir { source })?;

        for entry in read_dir {
            let path = entry
                .map_err(|source| PlaylistLoadError::ReadTrackDirEntry { source })?
                .path();

            if path.is_file() && path.extension().is_some_and(|ext| ext == "wav") {
                paths.push(path);
            }
        }

        paths.sort();

        if paths.is_empty() {
            return Err(PlaylistLoadError::EmptyTracksDir);
        }

        let mut tracks = Vec::with_capacity(paths.len());

        for path in paths {
            // Convert path to string; non-UTF8 paths are treated as errors
            let path_str = path
                .to_str()
                .ok_or_else(|| PlaylistLoadError::InvalidTrackPath { path: path.clone() })?;
            // Read file bytes and validate WAV format
            let bytes = fs::read(&path)
                .map_err(|source| PlaylistLoadError::ReadTrackFile { path: path.clone(), source })?;

            let duration = Self::parse_wav_duration(&bytes)
                .ok_or_else(|| PlaylistLoadError::InvalidWav { path: path.clone() })?;

            // Load sound via macroquad; if codec/file invalid, propagate error
            let sound = load_sound(path_str)
                .await
                .map_err(|message| PlaylistLoadError::LoadSound {
                    path: path.clone(),
                    message: message.to_string(),
                })?;

            let name = path
                .file_stem()
                .and_then(|stem| stem.to_str())
                .ok_or_else(|| PlaylistLoadError::InvalidTrackPath { path: path.clone() })?
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
        Ok(playlist)
    }

    fn parse_wav_duration(bytes: &[u8]) -> Option<f32> {
        // Validate WAV header: RIFF signature at start, WAVE signature at offset 8
        if bytes.len() < 12 || &bytes[0..4] != b"RIFF" || &bytes[8..12] != b"WAVE" {
            return None;
        }

        // Start after RIFF/WAVE headers; track format info and audio data size
        let mut offset = 12usize;
        let mut byte_rate = None;
        let mut data_size = None;

        while offset + 8 <= bytes.len() {
            let chunk_id = &bytes[offset..offset + 4];
            // Read chunk size safely; if slice missing or conversion fails, return None
            let chunk_size = u32::from_le_bytes(bytes.get(offset + 4..offset + 8)?.try_into().ok()?) as usize;
            let chunk_data_start = offset + 8;
            let chunk_data_end = chunk_data_start.checked_add(chunk_size)?;
            // Prevent integer overflow and validate chunk fits within buffer
            if chunk_data_end > bytes.len() {
                return None;
            }

            if chunk_id == b"fmt " {
                // Format chunk must have at least 16 bytes to contain byte_rate at offset 8
                if chunk_size < 16 {
                    return None;
                }
                // Extract byte_rate (bytes per second) from format chunk
                byte_rate = Some(u32::from_le_bytes(
                    bytes.get(chunk_data_start + 8..chunk_data_start + 12)?.try_into().ok()?,
                ));
            } else if chunk_id == b"data" {
                // Data chunk size is the audio sample size in bytes
                data_size = Some(chunk_size);
            }
            // Move to next chunk; align to even offset (chunk_size & 1 adds 1 if odd)
            offset = chunk_data_end + (chunk_size & 1);
        }

        // Duration = audio data size in bytes / bytes per second; fail if either is missing
        Some(data_size? as f32 / byte_rate? as f32)
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
