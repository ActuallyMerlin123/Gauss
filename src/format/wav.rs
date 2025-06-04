use std::io::{Read, Seek, Write};
use crate::{audio::{AudioBuffer, AudioSource, AudioSourceMut, AudioSpec, Endianness, SampleFormat}, load::{FormatReader, FormatWriter}};

use super::samples_into_bytes;

type WavInfo = hound::WavSpec;

/// Reader implementation for Riff Wave audio files.
#[derive(Debug, Clone)]
pub struct WavReader<R: Read> {
    inner: R,
}

impl<R: Read> WavReader<R> {
    /// Creates a new [`WavReader`]
    pub fn new(inner: R) -> Self {
        Self { inner }
    }

    #[allow(missing_docs)]
    pub fn into_inner(self) -> R {
        self.inner
    }
}

impl<R: Read> FormatReader for WavReader<R> {
    type FormatInfo = WavInfo;

    fn read(&mut self) -> anyhow::Result<(AudioBuffer, Self::FormatInfo)> {
        let mut wav_reader = hound::WavReader::new(&mut self.inner)?;
        let spec = wav_reader.spec();

        let samples: Vec<u8> = match (spec.sample_format, spec.bits_per_sample) {
            (hound::SampleFormat::Int, 8) => {
                let samples: Vec<i8> = wav_reader.samples::<i8>().collect::<Result<Vec<_>, _>>()?;
                samples_into_bytes(samples)
            },
            (hound::SampleFormat::Int, 16) => {
                let samples: Vec<i16> = wav_reader.samples::<i16>().collect::<Result<_, _>>()?;
                samples_into_bytes(samples)
            },
            (hound::SampleFormat::Int, 24) => {
                let samples: Vec<i32> = wav_reader.samples::<i32>().collect::<Result<_, _>>()?;
                // 24-bit PCM is usually stored in 32-bit i32, but only lower 24 bits are used
                samples_into_bytes(samples)
            },
            (hound::SampleFormat::Int, 32) => {
                let samples: Vec<i32> = wav_reader.samples::<i32>().collect::<Result<_, _>>()?;
                samples_into_bytes(samples)
            },
            (hound::SampleFormat::Float, 32) => {
                let samples: Vec<f32> = wav_reader.samples::<f32>().collect::<Result<_, _>>()?;
                samples_into_bytes(samples)
            },
            _ => {
                return Err(anyhow::anyhow!(
                    "Unsupported WAV format: {:?} bits/sample: {}",
                    spec.sample_format,
                    spec.bits_per_sample
                ));
            }
        };

        let mut buffer = AudioBuffer::new(AudioSpec {
            sample_rate: spec.sample_rate,
            bits_per_sample: spec.bits_per_sample,
            channels: spec.channels,
            sample_format: match spec.sample_format {
                hound::SampleFormat::Int => SampleFormat::Int,
                hound::SampleFormat::Float => SampleFormat::Float,
            },
            interleaved: false,
            endianness: Endianness::Little,
        });

        buffer.set_samples(samples);

        Ok((buffer, spec))
    }
}

/// Writer implementation for Riff Wave audio files.
pub struct WavWriter<W: Write + Seek> {
    inner: W,
}

impl<W: Write + Seek> WavWriter<W> {
    /// Creates a new [`WavWriter`]
    pub fn new(inner: W) -> Self {
        Self { inner }
    }

    #[allow(missing_docs)]
    pub fn into_inner(self) -> W {
        self.inner
    }
}

impl<W: Write + Seek> FormatWriter for WavWriter<W> {
    fn write<T: AudioSource>(&mut self, src: &T) -> anyhow::Result<()> {
        let spec = src.spec();
        let wav_spec = hound::WavSpec {
            sample_rate: spec.sample_rate,
            bits_per_sample: spec.bits_per_sample,
            channels: spec.channels,
            sample_format: match spec.sample_format {
                SampleFormat::Int => hound::SampleFormat::Int,
                SampleFormat::Float => hound::SampleFormat::Float,
            },
        };

        let mut wav_writer = hound::WavWriter::new(&mut self.inner, wav_spec)?;
        let samples = src.samples();

        match (spec.sample_format, spec.bits_per_sample) {
            (SampleFormat::Int, 16) => {
                for chunk in samples.chunks_exact(2) {
                    let sample = i16::from_le_bytes([chunk[0], chunk[1]]);
                    wav_writer.write_sample(sample)?;
                }
            }
            (SampleFormat::Int, 24) => {
                for chunk in samples.chunks_exact(3) {
                    let sample = ((chunk[2] as i32) << 16)
                               | ((chunk[1] as i32) << 8)
                               | (chunk[0] as i32);
                    let sample = if sample & 0x800000 != 0 {
                        sample | !0xFFFFFF // sign-extend
                    } else {
                        sample
                    };
                    wav_writer.write_sample(sample as i32)?;
                }
            }
            (SampleFormat::Int, 32) => {
                for chunk in samples.chunks_exact(4) {
                    let sample = i32::from_le_bytes(chunk.try_into().unwrap());
                    wav_writer.write_sample(sample)?;
                }
            }
            (SampleFormat::Float, 32) => {
                for chunk in samples.chunks_exact(4) {
                    let sample = f32::from_le_bytes(chunk.try_into().unwrap());
                    wav_writer.write_sample(sample)?;
                }
            }
            _ => {
                return Err(anyhow::anyhow!(
                    "Unsupported sample format: {:?} bits: {}",
                    spec.sample_format,
                    spec.bits_per_sample
                ));
            }
        }

        wav_writer.finalize()?;
        Ok(())
    }
}
