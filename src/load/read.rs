use crate::{audio::AudioBuffer, format::SupportedFormat};
use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

use super::FormatReader;

/// A dynamic reader for multiple audio formats.
///
/// The design follows the common 'Reader' pattern.
/// It can read from any source that implements the [`std::io::Read`]
/// trait. One can directly pass a source into the reader by using
/// [`AudioReader::new`]. For utility the [`AudioReader::open`] function,
/// that can construct a reader from a input file.
/// is provided.
#[derive(Debug, Clone)]
pub struct AudioReader<R: Read> {
    inner: R,
    format: SupportedFormat,
}

impl<R: Read> AudioReader<R> {
    /// Constructs a audio reader from a generic [`std::io::Read`] reader.
    pub fn new(inner: R, format: SupportedFormat) -> Self {
        AudioReader { inner, format }
    }

    /// Reads from the audio source into a [`AudioBuffer`].
    pub fn read(&mut self) -> anyhow::Result<AudioBuffer> {
        let mut reader_impl = super::find_reader(self.format, &mut self.inner)?;
        let (audio_buffer, _info) = reader_impl.read()?;
        Ok(audio_buffer)
    }

    #[allow(missing_docs)]
    pub fn into_inner(self) -> R {
        self.inner
    }
}

impl AudioReader<BufReader<File>> {
    /// Constructs a [`std::io::BufReader<std::fs::File>`] from a path
    pub fn open<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let file = File::open(&path)?;
        let reader = BufReader::new(file);
        let format = SupportedFormat::from_extension(&path)?;

        Ok(AudioReader::new(reader, format))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    /// General purpose test
    #[test]
    fn test_read() -> anyhow::Result<()> {
        let mut reader = AudioReader::open("assets/example.wav").unwrap();
        let audio = reader.read().unwrap();
        println!("Audio: {:#?}", audio);
        Ok(())
    }
}
