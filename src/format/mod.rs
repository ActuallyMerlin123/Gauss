/// Encoder/Decoer backend for Riff Wave
pub mod wav;


use std::path::Path;
use anyhow::anyhow;

/// Audio codecs/formats supported by gauss
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SupportedFormat {
    /// Riff Wave
    Wav,
    /// Mp3
    Mp3,
    /// Flac
    Flac,
    /// Ogg Vorbis
    Ogg,
}

impl SupportedFormat {
    /// Derive format from file extension
    ///
    /// - "wav" -> Wav,
    /// - "mp3" -> Mp3,
    /// - "flac" -> Flac,
    /// - "ogg" -> Ogg,
    #[inline(always)]
    pub fn from_extension<P>(path: P) -> anyhow::Result<Self>
    where
        P: AsRef<Path>,
    {
        match path.as_ref().extension().and_then(|e| e.to_str()) {
            Some(ext) => match ext.len() {
                3 => match ext.as_bytes() {
                    b"wav" | b"WAV" | b"Wav" => Ok(SupportedFormat::Wav),
                    b"mp3" | b"MP3" | b"Mp3" => Ok(SupportedFormat::Mp3),
                    b"ogg" | b"OGG" | b"Ogg" => Ok(SupportedFormat::Ogg),
                    _ => Err(anyhow!("Unsupported format: .{}", ext)),
                },
                4 => match ext.as_bytes() {
                    b"flac" | b"FLAC" | b"Flac" => Ok(SupportedFormat::Flac),
                    _ => Err(anyhow!("Unsupported format: .{}", ext)),
                },
                _ => Err(anyhow!("Unsupported extension length: .{}", ext)),
            },
            None => Err(anyhow!(
                "Failed to extract extension from path: {}",
                path.as_ref().display()
            )),
        }
    }
}

pub(crate) fn samples_into_bytes<T>(samples: Vec<T>) -> Vec<u8> {
    let bytes_len = samples.len() * std::mem::size_of::<T>();
    let mut bytes = Vec::with_capacity(bytes_len);

    unsafe {
        std::ptr::copy_nonoverlapping(
            samples.as_ptr() as *const u8,
            bytes.as_mut_ptr(),
            bytes_len
        );
        bytes.set_len(bytes_len);
    }

    bytes
}
