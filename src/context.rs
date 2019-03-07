use crate::Algorithm;

use chromaprint_sys as sys;
use failure::Fail;

use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr::null_mut;

#[derive(Debug, Fail, Copy, Clone)]
pub enum ContextError {
    #[fail(display = "Error allocating new chromaprint context")]
    InitializationError,
    #[fail(display = "Error starting fingerprint computation")]
    StartError,
    #[fail(display = "Error sending audio data to the fingerprint calculator")]
    FeedError,
    #[fail(display = "Error processing remaining buffered data")]
    FinishError,
    #[fail(display = "Error computing fingerprint")]
    FingerprintError,
    #[fail(display = "Error casting C-string into Rust: _0")]
    StringError(std::str::Utf8Error),
}

impl From<std::str::Utf8Error> for ContextError {
    fn from(e: std::str::Utf8Error) -> Self {
        ContextError::StringError(e)
    }
}

pub struct Context(*mut sys::ChromaprintContext);

macro_rules! chroma_call {
    ($f: expr, $err: path) => {{
        if unsafe { $f } != 1 {
            return Err($err);
        }
    }};
}

impl Context {
    /// Allocate and initialize the Chromaprint context.
    ///
    /// Note that when Chromaprint is compiled with FFTW, this function is not reentrant and you
    /// need to call it only from one thread at a time. This is not a problem when using FFmpeg or
    /// vDSP.
    pub fn new(algorithm: Algorithm) -> Result<Self, ContextError> {
        let ctx = unsafe { sys::chromaprint_new(algorithm.into()) };
        if ctx.is_null() {
            return Err(ContextError::InitializationError);
        }
        Ok(Context(ctx))
    }

    /// Send audio data to the fingerprint calculator.
    pub fn feed(&mut self, data: &[i16]) -> Result<(), ContextError> {
        let data_ptr = data.as_ptr();
        let size = data.len() as i32;
        chroma_call!(
            sys::chromaprint_feed(self.0, data_ptr, size),
            ContextError::FeedError
        );
        Ok(())
    }

    /// Process any remaining buffered audio data.
    pub fn finish(&mut self) -> Result<(), ContextError> {
        chroma_call!(sys::chromaprint_finish(self.0), ContextError::FinishError);
        Ok(())
    }

    /// Return the calculated fingerprint as a compressed string.
    pub fn get_fingerprint(&mut self) -> Result<String, ContextError> {
        let mut fingerprint: *mut c_char = null_mut();
        chroma_call!(
            sys::chromaprint_get_fingerprint(self.0, &mut fingerprint as *mut *mut c_char),
            ContextError::FingerprintError
        );
        if fingerprint.is_null() {
            return Err(ContextError::FingerprintError);
        }

        let cstr = unsafe { CStr::from_ptr(fingerprint) };
        let str_slice = cstr.to_str()?;
        Ok(str_slice.to_owned())
    }

    /// Restart the computation of a fingerprint with a new audio stream.
    pub fn start(&mut self, sample_rate: usize, num_channels: usize) -> Result<(), ContextError> {
        chroma_call!(
            sys::chromaprint_start(self.0, sample_rate as i32, num_channels as i32),
            ContextError::StartError
        );
        Ok(())
    }
}

impl Default for Context {
    fn default() -> Self {
        Context::new(Algorithm::default()).expect("Error allocating new chromaprint context")
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe { sys::chromaprint_free(self.0) }
    }
}
