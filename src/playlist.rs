extern crate libc;
extern crate projectm_sys as ffi;

use rand::Rng;
use std::ffi::CString;

pub struct Playlist {
    playlist: *mut ffi::projectm_playlist,
    rng: rand::rngs::ThreadRng,
}

pub type ProjectMHandle = *mut ffi::projectm;

impl Playlist {
    /// Create a new playlist for [Projectm](ProjectMHandle)
    pub fn create(projectm: ProjectMHandle) -> Playlist {
        let playlist;
        unsafe {
            playlist = ffi::projectm_playlist_create(projectm);
        }
        Playlist {
            playlist,
            rng: rand::thread_rng(),
        }
    }

    pub fn len(&self) -> u32 {
        unsafe { ffi::projectm_playlist_size(self.playlist) }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Scan and add a directory of presets to the playlist.
    pub fn add_path(&self, path: &str, recursive: bool) {
        unsafe {
            let c_path = CString::new(path).unwrap();
            ffi::projectm_playlist_add_path(self.playlist, c_path.as_ptr(), recursive, false);
        }
    }

    /// Go to the next preset in the playlist (hard cut).
    pub fn play_next(&mut self) {
        unsafe {
            ffi::projectm_playlist_play_next(self.playlist, true);
        }
    }

    /// Go to the previous preset in the playlist (hard cut).
    pub fn play_prev(&mut self) {
        unsafe {
            // FIXME THIS IS WRONG
            ffi::projectm_playlist_play_previous(self.playlist, true);
        }
    }

    /// Go to a random preset in the playlist (hard cut).
    pub fn play_random(&mut self) {
        let len = self.len();
        let index: u32 = self.rng.gen_range(0..len);
        unsafe {
            ffi::projectm_playlist_set_position(self.playlist, index, true);
        }
    }

    /// Set shuffle mode.
    pub fn set_shuffle(&self, shuffle: bool) {
        unsafe {
            ffi::projectm_playlist_set_shuffle(self.playlist, shuffle);
        }
    }

    /// Get shuffle mode.
    pub fn get_shuffle(&self) -> bool {
        unsafe { ffi::projectm_playlist_get_shuffle(self.playlist) }
    }
}
