use std::collections::HashMap;
use std::fs::*;
use std::path::*;
use std::sync::{Arc, RwLock};
use std::time::Duration;

use sdl2::audio::AudioSpecWAV;
use sdl2::AudioSubsystem;

struct AudioManagerShared {
    system: AudioSubsystem,
    files: RwLock<HashMap<PathBuf, (Option<AudioSpecWAV>, bool)>>,
}

/// This class handles the loading and playing of audio.
// TODO Should probably use a channel.
pub struct AudioManager(Arc<AudioManagerShared>);

impl AudioManager {
    /// Creates a new audio manager.
    pub fn new(system: AudioSubsystem) -> Self {
        Self(Arc::new(AudioManagerShared {
            system,
            files: RwLock::new(HashMap::new()),
        }))
    }

    /// Updates the audio manager.
    /// This function is called once pr. frame by the underlying "engine".
    pub fn update(&mut self) {}

    /// Registers an entire directory for the manager.
    ///
    /// Setting `recurse` to true will also register all the files in nested
    /// directories.
    pub fn register_dir(&self, path: &impl AsRef<Path>, recurse: bool) {
        let entries = read_dir(path).unwrap();
        for entry in entries {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() && recurse {
                self.register_dir(&path, true);
            } else if path.is_file() {
                let mut files = self.0.files.write().unwrap();
                files.insert(path, (None, false));
            }
        }
    }

    /// Registers a piece of audio so that the audio manager knows where to find
    /// it.
    ///
    /// # Panics
    /// Panics if `path` isn't a path to a file.
    pub fn register(&self, path: &impl AsRef<Path>) {
        let path = path.as_ref().to_owned();
        assert!(path.is_file());

        let mut files = self.0.files.write().unwrap();
        files.insert(path, (None, false));
    }

    /// Returns whether the `path` has been registered with the audio manager.
    ///
    /// This can be both files and directories.
    pub fn registered(&self, path: &impl AsRef<Path>) -> bool {
        let files = self.0.files.read().unwrap();
        files.contains_key(path.as_ref())
    }

    /// Tells the audio manager that this piece of audio will soon be used and
    /// it should be loaded in the backgorund.
    ///
    /// Internally calls `preload_with_time(path, Duration::SECOND)`
    ///
    /// # Panics
    ///
    /// - Panics if `path` hasn't been registered with the audio manager.
    /// - Panics if `path` isn't a path to a file.
    pub fn preload(&self, path: &impl AsRef<Path>) {
        assert!(path.as_ref().is_file());

        let mut files = self.0.files.write().unwrap();
        let (stream, _) = files.get_mut(path.as_ref()).unwrap();
        if stream.is_none() {
            *stream = Some(sdl2::audio::AudioSpecWAV::load_wav(path).unwrap())
        }
    }

    pub fn preload_with_time(&self, path: &impl AsRef<Path>, time: Duration) {}

    /// Starts playing the audio in `path`
    ///
    /// # Panics
    /// - Panics if `path` hasn't been registered with the audio manager.
    /// - Panics if `path` isn't pointing to a file.
    pub fn start(&self, path: &impl AsRef<Path>) {
        let files = self.0.files.read();
    }

    /// Stops playing the audio in `path`
    ///
    /// This function is a no-op if the audio is not playing.
    ///
    /// # Panics
    /// - Panics if `path` hasn't been registered with the audio manager.
    /// - Panics if `path` isn't pointing to a file.
    pub fn stop(&self, path: &impl AsRef<Path>) {}
}
