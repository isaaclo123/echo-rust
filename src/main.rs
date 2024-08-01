use std::convert::TryInto;
use std::io::Cursor;
use std::mem::transmute;
use std::process::Command;
use std::ptr::{null, null_mut};
use std::thread::sleep;
use std::time::Duration;
extern crate libc;

mod bitconvert;
use crate::bitconvert::convert_i24_buf_to_le_i32;
use crate::bitconvert::convert_i32_list_to_bytes;

use libc::{c_int, c_uint, c_void, FILE};
use std::ffi::CStr;
use wave_stream::reader::ReadEx;

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
    pcm_close, pcm_config, pcm_format_PCM_FORMAT_MAX, pcm_format_PCM_FORMAT_S16_LE,
    pcm_format_PCM_FORMAT_S24_LE, pcm_format_PCM_FORMAT_S32_LE, pcm_format_to_bits,
    pcm_frames_to_bytes, pcm_get_buffer_size, pcm_get_error, pcm_is_ready, pcm_mmap_begin,
    pcm_mmap_commit, pcm_open, pcm_params_get, pcm_read, PCM_IN,
};
// pcm_bytes_to_frames

//const NAMED_PIPE: &str = "./test.wav";
// const NAMED_PIPE: &str = "./tests/resources/alexa.wav";
// const NAMED_PIPE: &str = "/data/local/tmp/audio_fifo";
// const AUDIO_FILE: &str = "./audio_fifo";

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

    Command::new("killall").arg("mixer").status().unwrap();
    sleep(Duration::from_secs(1));
    // Command::new("killall").arg("mixer").status().unwrap();

    println!("{}", openssl::version::version());
    println!("Hello, world!");

    // wav setup
    println!("setup wav reader!");

    let card: c_uint = 0;
    let device: c_uint = 24;
    let rate: c_uint = 16000;
    let channels: c_uint = 9;
    let period_size = 512;
    let period_count = 5;
    let format = pcm_format_PCM_FORMAT_MAX;
    //
    // let card: c_uint = 1;
    // let device: c_uint = 0;
    // let rate: c_uint = 48000;
    // let channels: c_uint = 2;
    // let period_size = 1024;
    // let format = pcm_format_PCM_FORMAT_S32_LE;

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

    let mut pcm_config = Box::into_raw(Box::new(pcm_config {
        channels: channels,
        rate: rate,
        format: format,
        period_size: period_size,
        period_count: period_count,
        start_threshold: 0,
        silence_threshold: 0,
        stop_threshold: 0,
    }));

    unsafe {
        let pcm = pcm_open(card, device, PCM_IN, pcm_config);

        if pcm_is_ready(pcm) == 0 {
            let error = pcm_get_error(pcm);
            eprintln!(
                "Unable to open PCM device ({:})",
                CStr::from_ptr(error).to_str().unwrap()
            );
            return ();
        }

        // pcm->buffer_size = config->period_count * config->period_size;
        // frames
        let buffer_size = pcm_get_buffer_size(pcm);
        let bytes = pcm_frames_to_bytes(pcm, buffer_size);
        let mut buf: Vec<u8> = vec![0; bytes as usize];

        // let mut i32_buf: Vec<i32> = vec![0; buffer_size.try_into().unwrap()];

        let bits = pcm_format_to_bits(format);
        println!(
            "Capturing sample: {} ch, {} hz, {} bit\n",
            channels, rate, bits
        );

        let mut stdout = stdout();

        loop {
            let frames_read = pcm_read(pcm, buf.as_mut_ptr() as *mut c_void, bytes);

            if frames_read < 0 {
                let error = pcm_get_error(pcm);
                eprintln!(
                    "{:} error in read ({:})",
                    frames_read,
                    CStr::from_ptr(error).to_str().unwrap()
                );
                break;
            }

            // samples.into_iter().take_while(predicate)

            // coverts i24 buf into i32
            let i32_buf = convert_i24_buf_to_le_i32(&buf);
            // println!(
            //     "pcm_buffer_size: {:} i32 buf len {:?}, rus_bytes/frame: {:}, rus_samp/frame: {:}",
            //     buf.len(),
            //     i32_buf.len(),
            //     rustpotter.get_bytes_per_frame(),
            //     rustpotter.get_samples_per_frame()
            // );

            let i32_in_bytes = convert_i32_list_to_bytes(&i32_buf);
            stdout.write(&i32_in_bytes).await;

            /*
            pub fn process_samples<T: Sample>(
                &mut self,
                audio_samples: Vec<T>,
            ) -> Option<RustpotterDetection> {
                if audio_samples.len() != self.get_samples_per_frame() {
                    return None;
                }
                let float_samples = self.wav_encoder.rencode_and_resample::<T>(audio_samples);
                self.process_audio(float_samples)
            }
             */
            let detection = rustpotter.process_samples::<i32>(i32_buf);

            if let Some(detection) = detection {
                println!("{:?}", detection);
            }
        }
        pcm_close(pcm);
    }
}
