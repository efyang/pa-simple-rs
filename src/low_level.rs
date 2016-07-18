//! Provides low-level pulseaudio bindings and types.
//!
//! See also: http://freedesktop.org/software/pulseaudio/doxygen/simple_8h.html

/// The direction of information which will stream from the server.
///
/// See also:
/// [`pa_stream_direction_t`](http://freedesktop.org/software/pulseaudio/doxygen/def_8h.html#a637b1451881b8c0b0f98bafe115d7254)
/// and
/// [`pa_stream_direction`](http://freedesktop.org/software/pulseaudio/doxygen/def_8h.html#a7311932553b3f7962a092906576bc347).
pub enum StreamDirection {
//  /// Invalid direction.
//  NoDirection = 0,
//  /// Playback stream.
    Playback = 1,
    /// Record stream.
    Record = 2,
//  /// Sample upload stream.
//  Upload = 3,
}

/// Wire formats of individual audio samples.
///
/// See also:
/// [`pa_sample_format`](http://freedesktop.org/software/pulseaudio/doxygen/sample_8h.html#a3c622fc51f4fc6ebfdcc7b454ac9c05f)
pub enum SampleFormat {
    /// Unsigned 8 Bit PCM.
    U8,
    /// 8 Bit a-Law.
    ALAW,
    /// 8 Bit mu-Law.
    ULAW,
    /// Signed 16 Bit PCM, little endian (PC).
    S16LE,
//  /// Signed 16 Bit PCM, big endian.
//  S16BE,
    /// 32 Bit IEEE floating point, little endian (PC), range -1.0 to 1.0.
    FLOAT32LE,
//  /// 32 Bit IEEE floating point, big endian, range -1.0 to 1.0.
//  FLOAT32BE,
    /// Signed 32 Bit PCM, little endian (PC).
    S32LE,
//  /// Signed 32 Bit PCM, big endian.
//  S32BE,
//  /// Signed 24 Bit PCM packed, little endian (PC).
//  S24LE,
//  /// Signed 24 Bit PCM packed, big endian.
//  S24BE,
//  /// Signed 24 Bit PCM in LSB of 32 Bit words, little endian (PC).
//  S24_32LE,
//  /// Signed 24 Bit PCM in LSB of 32 Bit words, big endian.
//  S24_32BE,
//  /// Upper limit of valid sample types.
//  MAX,
//  /// An invalid value.
//  INVALID = -1,
}
