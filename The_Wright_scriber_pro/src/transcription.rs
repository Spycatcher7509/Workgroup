use std::error::Error;
use std::path::Path;
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext};
use hound;

/// Transcribe an audio file using Whisper (offline).
///
/// This function loads a Whisper model (path specified by the `WHISPER_MODEL_PATH` environment variable or default),
/// reads the provided WAV file, processes it through the model, and returns the transcribed text.
/// Returns an error if loading the model fails, reading the file fails, or transcription fails.
pub fn transcribe_file<P: AsRef<Path>>(path: P) -> Result<String, Box<dyn Error>> {
    let path = path.as_ref();

    // Get model path from environment variable or use default
    let model_path = std::env::var("WHISPER_MODEL_PATH")
        .unwrap_or_else(|_| "models/ggml-base.en.bin".to_string());

    // Initialize Whisper context
    let mut context = WhisperContext::new(&model_path)
        .map_err(|e| format!("Failed to load Whisper model: {}", e))?;

    // Open WAV file and convert samples to f32
    let mut reader = hound::WavReader::open(path)?;
    let samples: Vec<f32> = reader
        .samples::<i16>()
        .map(|s| s.unwrap() as f32 / 32768.0)
        .collect();

    // Set up transcription parameters
    let params = FullParams::new(SamplingStrategy::default());

    // Create state and run transcription
    let mut state = context
        .create_state()
        .map_err(|e| format!("Failed to create Whisper state: {}", e))?;
    state
        .full(params, &samples)
        .map_err(|e| format!("Whisper transcription failed: {}", e))?;

    // Retrieve segments and assemble result
    let num_segments = state.full_n_segments();
    let mut result = String::new();
    for i in 0..num_segments {
        let segment = state.full_get_segment_text(i)?;
        result.push_str(segment);
        result.push('\n');
    }
    Ok(result)
}
