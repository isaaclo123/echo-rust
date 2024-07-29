use std::convert::TryInto;
use std::{process::Command};

use rustpotter::{AudioFmt, BandPassConfig, DetectorConfig, FiltersConfig, GainNormalizationConfig, Rustpotter, RustpotterConfig, RustpotterDetection, ScoreMode};
use tokio::fs::File;
use tokio::io::{AsyncRead, AsyncReadExt, BufReader};

//const NAMED_PIPE: &str = "./test.wav";
const NAMED_PIPE: &str = "./tests/resources/alexa.wav";
// const NAMED_PIPE: &str = "/data/local/tmp/audio_fifo";

#[tokio::main]
async fn main() {
    // Test linking against OpenSSL.
    openssl::init();
    assert!(openssl::version::version().starts_with("OpenSSL "));
    println!("{}", openssl::version::version());
    println!("Hello, world!");

    println!("rustpotter config!");
    let mut rustpotter_config = RustpotterConfig::default();
    rustpotter_config.detector.avg_threshold = 0.;
    rustpotter_config.detector.min_scores = 20;
    rustpotter_config.detector.eager = true;

    // start tinycap for recording
    // println!("before audiorecord");
    // let mut audiorecord = Command::new("sh")
    //     .arg("-c")
    //     .arg(format!("killall mixer; killall mixer; tinycap {} -D 0 -d 24 -r 16000 -c 9 -b 24 -p 512 -n 5 -f", NAMED_PIPE))
    //     .spawn()
    //     .expect("failed to start tinycap");
    // println!("after audiorecord");

    // wav setup
    println!("setup wav reader!");

    // let wav_spec: rustpotter::AudioFmt = AudioFmt::try_from(wav_reader.spec()).expect("Unable to get wav spec")
    rustpotter_config.fmt = AudioFmt {
        sample_rate: 16000,
        sample_format: rustpotter::SampleFormat::I32,
        channels: 9,
        endianness: rustpotter::Endianness::Native
    };

    // rustpotter
    println!("setup rustpotter!");
    let mut rustpotter = Rustpotter::new(&rustpotter_config).unwrap();
    println!("add wakeword!");
    rustpotter.add_wakeword_from_file("alexa", "./tests/resources/alexa.rpw").unwrap();

    println!("at loop");

    let mut buf = vec![0; rustpotter.get_bytes_per_frame()];

    loop {
        let named_pipe = File::open(NAMED_PIPE).await.expect("Failed to open named pipe");
        let mut handle = named_pipe.take(rustpotter.get_bytes_per_frame().try_into().unwrap());
        
        handle.read(&mut buf).await;

        // println!("{:?}", buf);

        let detection = rustpotter.process_bytes(&buf);

        if let Some(detection) = detection {
            println!("{:?}", detection);
        }
    }
}
