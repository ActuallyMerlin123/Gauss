//! # Gauss

#![warn(missing_docs)]
#![warn(clippy::pedantic)]
#![allow(dead_code)]

/// Encoding/Decoding implementations for audio formats.
///
/// This module provides implementations for these formats:
/// - Riff Wave
pub mod format;

/// Audio reading and writing.
pub mod load;

/// Objects for audio representation
///
/// This module provides structs and traits for audio representation
/// in memory, such as a spanned segmemt of audio ([`AudioSpan`]) or a
/// full audio buffer ([`AudioBuffer`]). It also provides a trait to
/// abstract audio sources ([`AudioSource`]).
pub mod audio;
