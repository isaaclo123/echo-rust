use std::convert::TryInto;
use std::process::Command;
use std::ptr::{null, null_mut};
extern crate libc;

use libc::{c_int, c_uint, c_void, FILE};
use std::ffi::CStr;

use libc::free;
use libc::malloc;
use rustpotter::{
    AudioFmt, BandPassConfig, DetectorConfig, FiltersConfig, GainNormalizationConfig, Rustpotter,
    RustpotterConfig, RustpotterDetection, ScoreMode,
};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::io::{stdout, AsyncReadExt};
mod libtinyalsa;
use crate::libtinyalsa::{
    pcm_close, pcm_config, pcm_format_PCM_FORMAT_S32_LE, pcm_format_to_bits, pcm_frames_to_bytes,
    pcm_get_buffer_size, pcm_get_error, pcm_is_ready, pcm_mmap_begin, pcm_mmap_commit, pcm_open,
    pcm_read, PCM_IN,
};
// pcm_bytes_to_frames

//const NAMED_PIPE: &str = "./test.wav";
// const NAMED_PIPE: &str = "./tests/resources/alexa.wav";
// const NAMED_PIPE: &str = "/data/local/tmp/audio_fifo";
const AUDIO_FILE: &str = "./audio_fifo";

#[tokio::main]
async fn main() {
    // Test linking against OpenSSL.
    openssl::init();
    assert!(openssl::version::version().starts_with("OpenSSL "));

    println!("rustpotter config!");
    let mut rustpotter_config = RustpotterConfig::default();
    rustpotter_config.detector.avg_threshold = 0.;
    rustpotter_config.detector.min_scores = 20;
    rustpotter_config.detector.eager = true;

    // Command::new("killall")
    //     .arg("mixer")
    //     .status().unwrap();

    println!("{}", openssl::version::version());
    println!("Hello, world!");

    // wav setup
    println!("setup wav reader!");

    let card: c_uint = 1;
    let device: c_uint = 0;
    let rate: c_uint = 48000;
    let channels: c_uint = 2;
    let period_size = 1024;
    let format = pcm_format_PCM_FORMAT_S32_LE;

    // let wav_spec: rustpotter::AudioFmt = AudioFmt::try_from(wav_reader.spec()).expect("Unable to get wav spec")
    rustpotter_config.fmt = AudioFmt {
        sample_rate: rate as usize,
        sample_format: rustpotter::SampleFormat::I32,
        channels: channels as u16,
        endianness: rustpotter::Endianness::Little,
    };

    // rustpotter
    println!("setup rustpotter!");
    let mut rustpotter = Rustpotter::new(&rustpotter_config).unwrap();
    println!("add wakeword!");
    rustpotter
        .add_wakeword_from_file("alexa", "./tests/resources/alexa.rpw")
        .unwrap();

    println!("at loop");

    // let mut buf = vec![0; rustpotter.get_bytes_per_frame()];

    unsafe {
        let mut pcm_config = Box::into_raw(Box::new(pcm_config {
            channels: channels,
            rate: rate,
            format: format,
            period_size: period_size,
            period_count: 2,
            start_threshold: 0,
            silence_threshold: 0,
            stop_threshold: 0,
        }));

        let pcm = pcm_open(card, device, PCM_IN, pcm_config);

        if pcm_is_ready(pcm) == 0 {
            let error = pcm_get_error(pcm);
            eprintln!(
                "Unable to open PCM device ({:})",
                CStr::from_ptr(error).to_str().unwrap()
            );
            return ();
        }

        let buffer_size = pcm_get_buffer_size(pcm);
        let mut bytes = pcm_frames_to_bytes(pcm, buffer_size);
        // let buf = malloc(bytes.try_into().unwrap());
        // let mut buf = Vec::<u8>::with_capacity(bytes.try_into().unwrap());
        let mut buf: [u8; 16384] = [0; 16384];
        // let mut buf: Vec<u8> = vec![0.try_into().unwrap(), bytes.try_into().unwrap()];
        println!("{:?} BUF ", buf);

        let bits = pcm_format_to_bits(format);
        println!(
            "Capturing sample: {} ch, {} hz, {} bit\n",
            channels, rate, bits
        );

        println!("before file");
        // let mut file = File::create(AUDIO_FILE).await.expect("Failed to open file");
        // file.write_all(b"TESTING").await.unwrap();

        let mut stdout = stdout();

        loop {
            let frames_read = pcm_read(pcm, buf.as_mut_ptr() as *mut c_void, bytes);
            stdout.write_all(&buf).await.unwrap();

            if frames_read < 0 {
                let error = pcm_get_error(pcm);
                eprintln!(
                    "error in read ({:})",
                    CStr::from_ptr(error).to_str().unwrap()
                );
                continue;
            }

            let detection = rustpotter.process_bytes(&buf);

            if let Some(detection) = detection {
                println!("{:?}", detection);
            }

            // println!("{:} bytes read", read_success);
            // println!("{:}", CStr::from_ptr(buf as *const i8 ).to_str().unwrap());
            // println!("{:}", CStr::from_ptr(buf).to_string_lossy());
        }
        // while (i < 50000) {
        //   let read_success = pcm_read(pcm, buf, bytes);
        //   if ( bytes < i || read_success ) {
        //     break;
        //   }
        //   // if ( _fwrite_chk(buf, 1, bytes, filename, -1) != bytes ) {          eprintln!("Error capturing sample\n");
        //   //   break;
        //   // }
        //   i += bytes;
        // }
        // frames = pcm_bytes_to_frames(pcm, i);
        pcm_close(pcm);
    }

    /*
    loop {
        let named_pipe = File::open(NAMED_PIPE)
            .await
            .expect("Failed to open named pipe");
        let mut handle = named_pipe.take(rustpotter.get_bytes_per_frame().try_into().unwrap());

        handle.read(&mut buf).await;

        // println!("{:?}", buf);

        let detection = rustpotter.process_bytes(&buf);

        if let Some(detection) = detection {
            println!("{:?}", detection);
        }
    }
    */
}
