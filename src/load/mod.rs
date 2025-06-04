/// Audio decoder module
pub mod read;
pub use read::AudioReader;
/// Audio encoder module
pub mod write;
pub use write::AudioWriter;

use std::io::{Read, Seek, Write};

use crate::{
    audio::{AudioBuffer, AudioSource},
    format::SupportedFormat,
};

/// Trait abstracting audio format reader implemntations.
pub trait FormatReader {
    /// The type of format information that the reader will return
    type FormatInfo;
    /// Read from [`std::io::Reader`] and return an [`AudioBuffer`] and
    /// format information.
    fn read(&mut self) -> anyhow::Result<(AudioBuffer, Self::FormatInfo)>;
}

/// Trait abstracting audio format writer implementations
pub trait FormatWriter {
    /// Write to [`std::io::Writer`]
    fn write<T: AudioSource>(&mut self, audio: &T) -> anyhow::Result<()>;
}

/// A dummy format info type that can be used when no format information is needed.
pub struct NoFormatInfo;

#[inline(always)]
pub(crate) fn find_reader<R: Read>(
    format: SupportedFormat,
    src: &mut R,
) -> anyhow::Result<impl FormatReader> {
    match format {
        SupportedFormat::Wav => Ok(crate::format::wav::WavReader::new(src)),
        SupportedFormat::Mp3 => todo!(),
        SupportedFormat::Flac => todo!(),
        SupportedFormat::Ogg => todo!(),
    }
}

#[inline(always)]
pub(crate) fn find_writer<W: Write + Seek>(
    format: SupportedFormat,
    src: &mut W,
) -> anyhow::Result<impl FormatWriter> {
    match format {
        SupportedFormat::Wav => Ok(crate::format::wav::WavWriter::new(src)),
        SupportedFormat::Mp3 => todo!(),
        SupportedFormat::Flac => todo!(),
        SupportedFormat::Ogg => todo!(),
    }
}
