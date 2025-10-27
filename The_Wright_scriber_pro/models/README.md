# Models Directory

This directory is reserved for Whisper model files used by The_Wright_scriber_pro for offline transcription.

Please download a compatible GGML model file (e.g. `ggml-base.en.bin` from the official Whisper models repository) and place it in this `models` directory.

To tell the app which model to use, set the `WHISPER_MODEL_PATH` environment variable to the absolute path of your model file. If this variable is not set, the application will default to `models/ggml-base.en.bin`.
