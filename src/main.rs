#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use clap::Parser;
use std::path::Path;
use utils::write_to;

mod utils;

use whisper_cli::{Language, Model, Size, Whisper};

#[derive(Parser)]
#[command(
    author,
    version,
    about = "Locally transcribe audio files, using Whisper.",
    long_about = "Generate a transcript of an audio file using the Whisper speech-to-text engine. The transcript will be saved as a .txt, .vtt, and .srt file in the same directory as the audio file."
)]
struct Args {
    /// Name of the Whisper model to use
    #[clap(short, long, default_value = "medium")]
    model: Size,

    /// Language spoken in the audio. Attempts to auto-detect by default.
    #[clap(short, long)]
    lang: Option<Language>,

    /// Path to the audio file to transcribe
    audio: String,

    /// Toggle translation
    #[clap(short, long, default_value = "false")]
    translate: bool,

    /// Generate timestamps for each word
    #[clap(short, long, default_value = "false")]
    karaoke: bool,

    /// Logs verbose output
    #[clap(short, long, default_value = "false")]
    verbose: bool,
}

#[tokio::main]
async fn main() {
    let mut args = Args::parse();
    let audio = Path::new(&args.audio);
    let file_name = audio.file_name().unwrap().to_str().unwrap();

    assert!(audio.exists(), "The provided audio file does not exist.");

    if args.model.is_english_only() && (args.lang == Some(Language::Auto) || args.lang.is_none()) {
        args.lang = Some(Language::English);
    }

    assert!(
        !args.model.is_english_only() || args.lang == Some(Language::English),
        "The selected model only supports English."
    );

    let mut whisper = Whisper::new(Model::new(args.model), args.lang).await;
    let transcript = whisper
        .transcribe(audio, args.translate, args.karaoke, args.verbose)
        .unwrap();

    write_to(
        audio.with_file_name(format!("{file_name}.txt")),
        &transcript.as_text(),
    );
    write_to(
        audio.with_file_name(format!("{file_name}.vtt")),
        &transcript.as_vtt(),
    );
    write_to(
        audio.with_file_name(format!("{file_name}.srt")),
        &transcript.as_srt(),
    );

    println!("time: {:?}", transcript.processing_time);
}
