//! ProjectM for Rust
//!
//! This library contains bindings to libprojectm. Its purpose
//! is to read an audio input and to produce mesmerizing visuals
//! by detecting tempo, and rendering advanced equations into a
//! limitless array of user-contributed visualizations.
//!
//! # Example
//!
// ! use projectm_rs::core::*;
// !
// ! let ProjectMHandle = Projectm::create();
//!

extern crate libc;
extern crate projectm_sys as ffi;

use std::ffi::CString;

pub enum Projectm {}
pub type ProjectMHandle = *mut ffi::projectm;

pub type ProjectMChannels = u32;
pub const MONO: ProjectMChannels = 1;
pub const STEREO: ProjectMChannels = 2;

pub type ProjectMTouchType = u32;
pub const TOUCH_TYPE_RANDOM: ProjectMTouchType = 0;
pub const TOUCH_TYPE_CIRCLE: ProjectMTouchType = 1;
pub const TOUCH_TYPE_RADIAL_BLOB: ProjectMTouchType = 2;
pub const TOUCH_TYPE_BLOB2: ProjectMTouchType = 3;
pub const TOUCH_TYPE_BLOB3: ProjectMTouchType = 4;
pub const TOUCH_TYPE_DERIVATIVE_LINE: ProjectMTouchType = 5;
pub const TOUCH_TYPE_BLOB5: ProjectMTouchType = 6;
pub const TOUCH_TYPE_LINE: ProjectMTouchType = 7;
pub const TOUCH_TYPE_DOUBLE_LINE: ProjectMTouchType = 8;

impl Projectm {
    // -----------------
    // Core
    // -----------------

    pub fn create() -> *mut ffi::projectm {
        unsafe { ffi::projectm_create() }
    }

    pub fn destroy(instance: ProjectMHandle) {
        unsafe { ffi::projectm_destroy(instance) };
    }

    pub fn load_preset_file(instance: ProjectMHandle, filename: &str, smooth_transition: bool) {
        unsafe {
            ffi::projectm_load_preset_file(
                instance,
                filename.as_ptr() as *mut i8,
                smooth_transition,
            )
        };
    }

    pub fn load_preset_data(instance: ProjectMHandle, data: &str, smooth_transition: bool) {
        unsafe {
            ffi::projectm_load_preset_data(instance, data.as_ptr() as *mut i8, smooth_transition)
        };
    }

    pub fn reset_textures(instance: ProjectMHandle) {
        unsafe { ffi::projectm_reset_textures(instance) };
    }

    pub fn get_version_components() -> (i32, i32, i32) {
        #[derive(Debug, Default, Copy, Clone)]
        #[repr(C, packed)]
        struct Version {
            major: i32,
            minor: i32,
            patch: i32,
        }

        let mut version = Version::default();

        unsafe {
            ffi::projectm_get_version_components(
                std::ptr::addr_of_mut!(version.major),
                std::ptr::addr_of_mut!(version.minor),
                std::ptr::addr_of_mut!(version.patch),
            );
        }

        (version.major, version.minor, version.patch)
    }

    pub fn get_version_string() -> String {
        let get_version = unsafe { ffi::projectm_get_version_string() };
        let version_str = unsafe { std::ffi::CStr::from_ptr(get_version) };
        let version_str_slice = version_str.to_str().unwrap();
        let version = version_str_slice.to_owned();

        unsafe { ffi::projectm_free_string(get_version) };

        version
    }

    pub fn get_vcs_version_string() -> String {
        let get_vcs_version = unsafe { ffi::projectm_get_vcs_version_string() };
        let vcs_version_str = unsafe { std::ffi::CStr::from_ptr(get_vcs_version) };
        let vcs_version_str_slice = vcs_version_str.to_str().unwrap();
        let vcs_version = vcs_version_str_slice.to_owned();

        unsafe { ffi::projectm_free_string(get_vcs_version) };

        vcs_version
    }

    // -----------------
    // Callbacks
    // -----------------

    pub fn set_preset_switch_requested_event_callback<F: FnMut(bool)>(
        instance: ProjectMHandle,
        callback: F,
    ) {
        unsafe extern "C" fn trampoline<F: FnMut(bool)>(
            is_hard_cut: bool,
            user_data: *mut std::os::raw::c_void,
        ) {
            unsafe { (*user_data.cast::<F>())(is_hard_cut) }
        }
        unsafe {
            ffi::projectm_set_preset_switch_requested_event_callback(
                instance,
                Some(trampoline::<F>),
                (Box::leak(Box::new(callback)) as *mut F).cast::<std::os::raw::c_void>(),
            )
        }
    }

    pub fn set_preset_switch_failed_event_callback<F: FnMut(String, String)>(
        instance: ProjectMHandle,
        callback: F,
    ) {
        unsafe extern "C" fn trampoline<F: FnMut(String, String)>(
            preset_filename: *const i8,
            message: *const i8,
            user_data: *mut std::os::raw::c_void,
        ) {
            let preset_filename_str = unsafe { std::ffi::CStr::from_ptr(preset_filename) };
            let preset_filename_str_slice = preset_filename_str.to_str().unwrap();
            let preset_filename = preset_filename_str_slice.to_owned();

            let message_str = unsafe { std::ffi::CStr::from_ptr(message) };
            let message_str_slice = message_str.to_str().unwrap();
            let message = message_str_slice.to_owned();
            unsafe { (*user_data.cast::<F>())(preset_filename, message) }
        }
        unsafe {
            ffi::projectm_set_preset_switch_failed_event_callback(
                instance,
                Some(trampoline::<F>),
                (Box::leak(Box::new(callback)) as *mut F).cast::<std::os::raw::c_void>(),
            )
        }
    }

    // -----------------
    // Parameters
    // -----------------

    pub fn set_texture_search_paths(
        instance: ProjectMHandle,
        texture_search_paths: &Vec<String>,
        count: usize,
    ) {
        let texture_search_paths_cstr: Vec<_> = texture_search_paths
            .iter()
            .map(|arg| CString::new(arg.as_str()).unwrap())
            .collect();

        let mut texture_search_paths_pointer: Vec<_> = texture_search_paths_cstr
            .iter() // do NOT into_iter()
            .map(|arg| arg.as_ptr())
            .collect();

        texture_search_paths_pointer.push(std::ptr::null());

        unsafe {
            ffi::projectm_set_texture_search_paths(
                instance,
                texture_search_paths_pointer.as_ptr() as *mut *const ::std::os::raw::c_char,
                count,
            )
        };
    }

    pub fn get_beat_sensitivity(instance: ProjectMHandle) -> f32 {
        unsafe { ffi::projectm_get_beat_sensitivity(instance) }
    }

    pub fn set_beat_sensitivity(instance: ProjectMHandle, sensitivity: f32) {
        unsafe { ffi::projectm_set_beat_sensitivity(instance, sensitivity) };
    }

    pub fn get_hard_cut_duration(instance: ProjectMHandle) -> f64 {
        unsafe { ffi::projectm_get_hard_cut_duration(instance) }
    }

    pub fn set_hard_cut_duration(instance: ProjectMHandle, seconds: f64) {
        unsafe { ffi::projectm_set_hard_cut_duration(instance, seconds) };
    }

    pub fn get_hard_cut_enabled(instance: ProjectMHandle) -> bool {
        unsafe { ffi::projectm_get_hard_cut_enabled(instance) }
    }

    pub fn set_hard_cut_enabled(instance: ProjectMHandle, enabled: bool) {
        unsafe { ffi::projectm_set_hard_cut_enabled(instance, enabled) }
    }

    pub fn get_hard_cut_sensitivity(instance: ProjectMHandle) -> f32 {
        unsafe { ffi::projectm_get_hard_cut_sensitivity(instance) }
    }

    pub fn set_hard_cut_sensitivity(instance: ProjectMHandle, sensitivity: f32) {
        unsafe { ffi::projectm_set_hard_cut_sensitivity(instance, sensitivity) }
    }

    pub fn get_soft_cut_duration(instance: ProjectMHandle) -> f64 {
        unsafe { ffi::projectm_get_soft_cut_duration(instance) }
    }

    pub fn set_soft_cut_duration(instance: ProjectMHandle, seconds: f64) {
        unsafe { ffi::projectm_set_soft_cut_duration(instance, seconds) }
    }

    pub fn get_preset_duration(instance: ProjectMHandle) -> f64 {
        unsafe { ffi::projectm_get_preset_duration(instance) }
    }

    pub fn set_preset_duration(instance: ProjectMHandle, seconds: f64) {
        unsafe { ffi::projectm_set_preset_duration(instance, seconds) }
    }

    pub fn get_mesh_size(instance: ProjectMHandle) -> (usize, usize) {
        #[derive(Debug, Default, Copy, Clone)]
        #[repr(C, packed)]
        struct Mesh {
            mesh_x: usize,
            mesh_y: usize,
        }

        let mut mesh = Mesh::default();

        unsafe {
            ffi::projectm_get_mesh_size(
                instance,
                std::ptr::addr_of_mut!(mesh.mesh_x),
                std::ptr::addr_of_mut!(mesh.mesh_y),
            );
        }

        (mesh.mesh_x, mesh.mesh_y)
    }

    pub fn set_mesh_size(instance: ProjectMHandle, mesh_x: usize, mesh_y: usize) {
        unsafe {
            ffi::projectm_set_mesh_size(instance, mesh_x, mesh_y);
        }
    }

    pub fn get_fps(instance: ProjectMHandle) -> u32 {
        unsafe { ffi::projectm_get_fps(instance).try_into().unwrap() }
    }

    // FIXME: shouldn't it also be a usize?
    pub fn set_fps(instance: ProjectMHandle, fps: u32) {
        unsafe { ffi::projectm_set_fps(instance, fps as i32) };
    }

    pub fn get_aspect_correction(instance: ProjectMHandle) -> bool {
        unsafe { ffi::projectm_get_aspect_correction(instance) }
    }

    pub fn set_aspect_correction(instance: ProjectMHandle, enabled: bool) {
        unsafe { ffi::projectm_set_aspect_correction(instance, enabled) };
    }

    pub fn get_easter_egg(instance: ProjectMHandle) -> f32 {
        unsafe { ffi::projectm_get_easter_egg(instance) }
    }

    pub fn set_easter_egg(instance: ProjectMHandle, sensitivity: f32) {
        unsafe { ffi::projectm_set_easter_egg(instance, sensitivity) };
    }

    pub fn get_preset_locked(instance: ProjectMHandle) -> bool {
        unsafe { ffi::projectm_get_preset_locked(instance) }
    }

    pub fn set_preset_locked(instance: ProjectMHandle, lock: bool) {
        unsafe { ffi::projectm_set_preset_locked(instance, lock) };
    }

    pub fn get_window_size(instance: ProjectMHandle) -> (usize, usize) {
        #[derive(Debug, Default, Copy, Clone)]
        #[repr(C, packed)]
        struct Mesh {
            width: usize,
            height: usize,
        }

        let mut window = Mesh::default();

        unsafe {
            ffi::projectm_get_window_size(
                instance,
                std::ptr::addr_of_mut!(window.width),
                std::ptr::addr_of_mut!(window.height),
            );
        }

        (window.width, window.height)
    }

    pub fn set_window_size(instance: ProjectMHandle, width: usize, height: usize) {
        unsafe { ffi::projectm_set_window_size(instance, width, height) };
    }

    // -----------------
    // Render OpenGL
    // -----------------

    pub fn render_frame(instance: ProjectMHandle) {
        unsafe { ffi::projectm_opengl_render_frame(instance) };
    }

    // -----------------
    // Touch
    // -----------------

    pub fn touch(
        instance: ProjectMHandle,
        x: f32,
        y: f32,
        pressure: i32,
        touch_type: ProjectMTouchType,
    ) {
        unsafe { ffi::projectm_touch(instance, x, y, pressure, touch_type.try_into().unwrap()) };
    }

    pub fn touch_drag(instance: ProjectMHandle, x: f32, y: f32, pressure: i32) {
        unsafe { ffi::projectm_touch_drag(instance, x, y, pressure) };
    }

    pub fn touch_destroy(instance: ProjectMHandle, x: f32, y: f32) {
        unsafe { ffi::projectm_touch_destroy(instance, x, y) };
    }

    pub fn touch_destroy_all(instance: ProjectMHandle) {
        unsafe { ffi::projectm_touch_destroy_all(instance) };
    }

    // -----------------
    // Audio
    // -----------------

    pub fn pcm_get_max_samples() -> u32 {
        unsafe { ffi::projectm_pcm_get_max_samples() }
    }

    pub fn pcm_add_float(instance: ProjectMHandle, samples: Vec<f32>, channels: ProjectMChannels) {
        assert!(
            samples.len() <= Self::pcm_get_max_samples() as usize,
            "Number of samples is greater than max samples"
        );
        let samples_per_channel = samples.len() / channels as usize;
        unsafe {
            ffi::projectm_pcm_add_float(
                instance,
                samples.as_ptr(),
                samples_per_channel as u32,
                channels.try_into().unwrap(),
            )
        }
    }

    pub fn pcm_add_int16(instance: ProjectMHandle, samples: Vec<i16>, channels: ProjectMChannels) {
        assert!(
            samples.len() <= Self::pcm_get_max_samples() as usize,
            "Number of samples is greater than max samples"
        );
        let samples_per_channel = samples.len() / channels as usize;
        unsafe {
            ffi::projectm_pcm_add_int16(
                instance,
                samples.as_ptr(),
                samples_per_channel as u32,
                channels.try_into().unwrap(),
            )
        }
    }

    pub fn pcm_add_uint8(instance: ProjectMHandle, samples: Vec<u8>, channels: ProjectMChannels) {
        assert!(
            samples.len() <= Self::pcm_get_max_samples() as usize,
            "Number of samples is greater than max samples"
        );
        let samples_per_channel = samples.len() / channels as usize;
        unsafe {
            ffi::projectm_pcm_add_uint8(
                instance,
                samples.as_ptr(),
                samples_per_channel as u32,
                channels.try_into().unwrap(),
            )
        }
    }

    // -----------------
    // Debug
    // -----------------

    pub fn write_debug_image_on_next_frame(instance: ProjectMHandle, output_file: Option<&String>) {
        let output = if let Some(..) = output_file {
            std::ptr::null()
        } else {
            output_file.unwrap().as_ptr() as *mut i8
        };

        unsafe { ffi::projectm_write_debug_image_on_next_frame(instance, output) };
    }
}
