use std::convert::TryInto;
use std::io::Cursor;
use std::mem::transmute;
use std::os::unix::thread;
use std::process::Command;
use std::ptr::{null, null_mut};
use std::sync::Arc;
use std::thread::{sleep, spawn};
use std::time::Duration;
extern crate libc;

mod bitconvert;
use crate::bitconvert::convert_i24_buf_to_le_i32;
use crate::bitconvert::convert_i32_list_to_bytes;

use libc::{c_int, c_uint, c_void, FILE};
use ringbuf::{traits::*, HeapRb, SharedRb};
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

#[tokio::main]
async fn main() {
    let mut rustpotter_config = RustpotterConfig::default();
    rustpotter_config.detector.avg_threshold = 0.;
    rustpotter_config.detector.min_scores = 20;
    rustpotter_config.detector.eager = true;

    Command::new("killall").arg("mixer").status().unwrap();
    sleep(Duration::from_secs(1));
    // Command::new("killall").arg("mixer").status().unwrap();

    // wav setup
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

    let rbuf = HeapRb::<i32>::new((60000 * 2) as usize);
    let (mut prod, mut cons) = rbuf.split();

    // let mut stdout = stdout();
    spawn(move || unsafe {
        println!("Spawned PCM");
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

            // coverts i24 buf into i32
            let i32_buf = convert_i24_buf_to_le_i32(&buf);
            println!(
                "pcm_buffer_size: {:} i32 buf len {:?}",
                buf.len(),
                i32_buf.len(),
            );

            let mut backoff = 0;
            let mut pushed_len = prod.push_slice(&i32_buf);
            while pushed_len < i32_buf.len() {
                pushed_len += prod.push_slice(&i32_buf[pushed_len..]);

                // if buf not increasing, start backing off of reads
                if pushed_len == 0 {
                    sleep(Duration::from_millis(100 * backoff));
                    backoff += 1;
                }
            }
        }

        pcm_close(pcm);
    });

    // consumer thread
    println!("Spawned Consumer");
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
    loop {
        let mut buf: Vec<i32> = Vec::new();
        let mut backoff = 1;

        while buf.len() < rustpotter.get_samples_per_frame() {
            let old_buf_len = buf.len();
            // keep taking until we reach rustpotter samples per frame
            buf.extend(
                cons.pop_iter()
                    .take(rustpotter.get_samples_per_frame() - buf.len()),
            );

            // if buf not increasing, start backing off of reads
            if buf.len() == old_buf_len {
                sleep(Duration::from_millis(100 * backoff));
                backoff += 1;
            }
        }

        println!("processing {:} bytes in rustpotter", buf.len());
        let detection = rustpotter.process_samples::<i32>(buf);

        if let Some(detection) = detection {
            println!("{:?}", detection);
        }
        // let i32_in_bytes = convert_i32_list_to_bytes(&i32_buf);
        // stdout.write(&i32_in_bytes).await;

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
    }
}
