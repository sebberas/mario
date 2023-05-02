use std::collections::HashMap;
use std::fs::*;
use std::path::*;
use std::sync::{Arc, Mutex, RwLock};
use std::time::Duration;

use sdl2::audio::{AudioCVT, AudioCallback, AudioDevice, AudioSpecDesired, AudioSpecWAV};
use sdl2::AudioSubsystem;

struct Sound {
    data: Vec<u8>,
    position: usize,
}

impl AudioCallback for Sound {
    type Channel = u8;

    fn callback(&mut self, output: &mut [Self::Channel]) {
        let Self { data, position } = self;

        for item in output {
            // [0, 255] -> [-128, 127]
            let point = data.get(*position).map(|p| *p as f32).unwrap_or(0f32) - 128f32;
            let scaled = (point + 128f32) as u8;
            *item = scaled;
            *position += 1;
        }
    }
}

struct AudioManagerShared {
    system: AudioSubsystem,
    device: Mutex<Option<AudioDevice<Sound>>>,
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
            device: Mutex::new(None),
            files: RwLock::new(HashMap::new()),
        }))
    }

    /// Updates the audio manager.
    /// This function is called once pr. frame by the underlying "engine".
    pub fn update(&mut self) {
        let mut device = self.0.device.lock().unwrap();
        if let Some(device) = device.as_mut() {
            println!("yeet");
            device.resume();
        }
    }

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
        let mut files = self.0.files.write().unwrap();

        let (stream, _) = files.get_mut(path.as_ref()).unwrap();
        let stream = if let Some(stream) = stream.as_mut() {
            stream
        } else {
            let stream = AudioSpecWAV::load_wav(path).unwrap();
            files.insert((path.as_ref()).to_owned(), (Some(stream), false));
            files.get_mut(path.as_ref()).unwrap().0.as_mut().unwrap()
        };

        let spec = AudioSpecDesired {
            freq: Some(44_100),
            channels: Some(1),
            samples: None,
        };

        let device = self.0.system.open_playback(None, &spec, |spec| {
            let converter = AudioCVT::new(
                stream.format,
                stream.channels,
                stream.freq,
                spec.format,
                spec.channels,
                spec.freq,
            )
            .unwrap();
            let data = converter.convert(stream.buffer().to_vec());

            Sound { data, position: 0 }
        });

        *self.0.device.lock().unwrap() = Some(device.unwrap());
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
