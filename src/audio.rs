use std::path::Path;

use sdl2::AudioSubsystem;

// This class handles the loading and playing of audio.
pub struct AudioManager {}

impl AudioManager {
    /// Creates a new audio manager.
    pub fn new(system: AudioSubsystem) -> Self {
        todo!()
    }

    /// Registers an entire directory for the manager.
    ///
    /// Setting `recurse` to true will also register all the nested directories.
    pub fn register_dir(&self, path: &impl AsRef<Path>, recurse: bool) {
        todo!()
    }

    /// Registers a piece of audio so that the audio manager knows where to find
    /// it.
    ///
    /// # Panics
    /// Panics if `path` isn't pointing to a file.
    pub fn register(&self, path: &impl AsRef<Path>) {
        todo!()
    }

    /// Returns whether the `path` has been registered with the audio manager.
    ///
    /// This can be both files and directories.
    pub fn registered(&self, path: &impl AsRef<Path>) -> bool {
        todo!()
    }

    /// Tells the audio manager that this piece of audio will soon be used and
    /// it should be loaded in the backgorund.
    ///
    /// # Panics
    ///
    /// Panics if `path` hasn't been registered with the audio manager.
    ///
    /// Panics if `path` isn't pointing to a file.
    pub fn preload(&self, path: &impl AsRef<Path>) {
        todo!()
    }

    /// Starts playing the audio in `path`
    ///
    /// # Panics
    /// - Panics if `path` hasn't been registered with the audio manager.
    /// - Panics if `path` isn't pointing to a file.
    pub fn start(&self, path: &impl AsRef<Path>) {}

    /// Stops playing the audio in `path`
    ///
    /// This function is a no-op if the audio is not playing.
    ///
    /// # Panics
    /// - Panics if `path` hasn't been registered with the audio manager.
    /// - Panics if `path` isn't pointing to a file.
    pub fn stop(&self, path: &impl AsRef<Path>) {}
}
