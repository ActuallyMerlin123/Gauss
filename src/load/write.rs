use std::{fs::File, io::{BufWriter, Seek, Write}, path::Path};
use crate::{audio::AudioSource, format::SupportedFormat};
use super::FormatWriter;


/// A dynamic writer for multiple audio formats.
#[derive(Debug, Clone)]
pub struct AudioWriter<W: Write + Seek> {
    inner: W,
}

impl<W: Write + Seek> AudioWriter<W> {
    /// Creates a new `AudioWriter` with the given inner writer.
    pub fn new(inner: W) -> Self {
        Self { inner }
    }

    #[allow(missing_docs)]
    pub fn into_inner(self) -> W {
        self.inner
    }

    /// Writes the audio source to a reader destination in the specified format.
    pub fn write<T: AudioSource>(&mut self, src: &T, format: SupportedFormat) -> anyhow::Result<()> {
        let mut writer_impl = super::find_writer(format, &mut self.inner)?;
        writer_impl.write(src)?;

        Ok(())
    }
}

impl AudioWriter<BufWriter<File>> {
    /// Creates a new [`AudioWriter`] that writes to the specified file path.
    pub fn create<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let path = path.as_ref();

        Ok(Self { inner: BufWriter::new(File::create(path)?) })
    }
}


#[cfg(test)]
mod tests {
    use std::f32::consts::PI;
    use crate::audio::{AudioBuffer, AudioSourceMut, AudioSpan, AudioSpec, Endianness, SampleFormat};
    use super::*;

    #[test]
    fn test_write() -> anyhow::Result<()> {
        let mut writer = AudioWriter::create("tests/write.wav").unwrap();

        const DURATION_SECS: usize = 2;
        const SAMPLE_RATE: usize = 44100;
        const NUM_SAMPLES: usize = SAMPLE_RATE * DURATION_SECS;
        const FREQUENCY: f32 = 440.0; // Hz

        let mut audio = AudioBuffer::new(AudioSpec {
            channels: 1,
            sample_rate: SAMPLE_RATE as u32,
            bits_per_sample: 16,
            endianness: Endianness::Little,
            sample_format: SampleFormat::Int,
            interleaved: false,
        });

        audio.resize(NUM_SAMPLES * 2, 0); // 2 bytes per sample for i16
        let samples = audio.samples_mut();

        for i in 0..NUM_SAMPLES {
            let t = i as f32 / SAMPLE_RATE as f32;
            let value = (i16::MAX as f32 * (2.0 * PI * FREQUENCY * t).cos()) as i16;
            let bytes = value.to_le_bytes();
            samples[i * 2] = bytes[0];
            samples[i * 2 + 1] = bytes[1];
        }

        let span = AudioSpan::new_spanned(&audio, 100, 500);

        writer.write(&span, SupportedFormat::Wav)?;

        Ok(())
    }

}
