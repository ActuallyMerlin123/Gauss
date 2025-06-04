/// Abstracts audio suppliers
pub trait AudioSource {
    /// Get specification
    fn spec(&self) -> AudioSpec;
    /// Get samples
    fn samples(&self) -> &[u8];

}

/// Abstracts audio suppliers that can be modified
///
/// The main purpose of this trait is to allow
/// for distinction between audio spans and
/// audio buffers that can be modified.
pub trait AudioSourceMut: AudioSource {
    /// Get mutable samples
    fn samples_mut(&mut self) -> &mut [u8];
    /// Set samples
    fn set_samples(&mut self, samples: Vec<u8>);
}

/// Data format of audio samples
#[derive(Debug, Clone, Copy)]
pub enum SampleFormat {
    /// Floating Point Sample
    Float,
    /// Integer Sample
    Int,
}

/// Endianness of audio samples
#[derive(Debug, Clone, Copy)]
pub enum Endianness {
    /// Little-endian byte order
    Little,
    /// Big-endian byte order
    Big,
}

/// Represents audio information
#[derive(Debug, Clone, Copy)]
pub struct AudioSpec {
    /// Sample rate in Hz
    ///
    /// A sample rate represents the number of samples
    /// per second in an audio stream.
    ///
    /// Common sample rates include:
    /// - 22050 Hz (low quality)
    /// - 32000 Hz (medium quality)
    /// - 44100 Hz (CD quality)
    /// - 48000 Hz (professional audio)
    pub sample_rate: u32,

    /// Number of bits per sample
    ///
    /// A sample is a single audio data point. Each sample
    /// can be represented by a certain number of bits.
    ///
    /// Common bit depths include:
    /// - 8 bits (low quality, often used in telephony)
    /// - 16 bits (CD quality, common in music)
    /// - 24 bits (high quality, used in professional audio)
    /// - 32 bits (used in some high-end applications)
    pub bits_per_sample: u16,

    /// Sample format
    ///
    /// The sample format defines how each audio sample
    /// is represented in memory. Take a look at [`SampleFormat`]
    /// for more details.
    pub sample_format: SampleFormat,

    /// Number of audio channels
    ///
    /// A channel represents a single audio stream.
    /// Channels are usually more then one, when dealing
    /// with stereo or surround sound.
    ///
    /// Common channel configurations include:
    /// - 1 channel (mono)
    /// - 2 channels (stereo)
    /// - 4 channels (quadraphonic)
    /// - 6 channels (5.1 surround sound)
    /// - 8 channels (7.1 surround sound)
    ///
    /// Note: The number of channels can affect the
    /// perceived quality of the audio.
    pub channels: u16,

    /// Audio is interleaved or not
    ///
    /// Interleaved audio means that samples from different
    /// channels are stored in a single buffer, with each
    /// channel's samples alternating.
    ///
    /// For example, in a stereo interleaved buffer,
    /// the samples would be stored as:
    /// ```
    /// [left0, right0, left1, right1, ...]
    /// ```
    ///
    /// Non-interleaved audio means that each channel's samples
    /// are stored in separate buffers.
    /// In a stereo non-interleaved buffer,
    /// the samples would be stored as:
    /// ```
    /// [left0, left1, ...], [right0, right1, ...]
    /// ```
    pub interleaved: bool,

    /// Endianness of audio samples
    ///
    /// Endianness defines the byte order in which
    /// multi-byte audio samples are stored:
    /// - Little-endian (least significant byte first)
    /// - Big-endian (most significant byte first)
    pub endianness: Endianness,
}

/// Stores audio data in a buffer.
///
/// Samples are stored as raw bytes: `samples`.
/// Information about the audio data is stored in
/// [`AudioSpec`].
///
/// # Examples
/// ```rust
/// # use audio_buffer::{AudioBuffer, AudioSpec};
///
/// let spec = AudioSpec {
///     sample_rate: 44100,
///     bits_per_sample: 16,
///     channels: 2,
/// };
///
/// let buffer = AudioBuffer {
///     samples: Vec::new(),
///     spec,
/// };
///
/// // Generate 2 second cosine wave
///
/// ```
#[derive(Debug, Clone)]
pub struct AudioBuffer {
    samples: Vec<u8>,
    spec: AudioSpec,
}

impl AudioBuffer {
    /// Creates a new audio buffer from a specification
    pub fn new(spec: AudioSpec) -> Self {
        Self {
            samples: Vec::new(),
            spec,
        }
    }

    /// Resize buffer
    pub fn resize(&mut self, new_len: usize, value: u8) {
        self.samples.resize(new_len, value);
    }
}

impl AudioSource for AudioBuffer {
    fn spec(&self) -> AudioSpec {
        self.spec
    }

    fn samples(&self) -> &[u8] {
        &self.samples
    }
}

impl AudioSourceMut for AudioBuffer {
    fn samples_mut(&mut self) -> &mut [u8] {
        &mut self.samples
    }

    fn set_samples(&mut self, samples: Vec<u8>) {
        self.samples = samples;
    }
}

/// Represents a segment of an audio
#[derive(Debug, Clone)]
pub struct AudioSpan<'src> {
    buffer: &'src AudioBuffer,
    offset: usize,
    len: usize,
}

impl<'src> AudioSpan<'src> {
    /// Creates a new full audio span from a [`AudioBuffer`]
    pub fn new(buffer: &'src AudioBuffer) -> Self {
        Self {
            buffer,
            offset: 0,
            len: buffer.samples.len(),
        }
    }

    /// Creates a cropped audio span from a [`AudioBuffer`]
    pub fn new_spanned(buffer: &'src AudioBuffer, offset: usize, len: usize) -> Self {
        Self {
            buffer,
            offset,
            len,
        }
    }
}

impl<'src> AudioSource for AudioSpan<'src> {
    fn spec(&self) -> AudioSpec {
        self.buffer.spec
    }

    fn samples(&self) -> &[u8] {
        &self.buffer.samples[self.offset..self.offset + self.len]
    }
}
