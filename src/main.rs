// imports
use rodio::{buffer::SamplesBuffer, source::Source, OutputStream, Sink};
use std::f32::consts::PI;
use std::io::{self};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

// constants and variable
const AMPLITUDE: f32 = 0.5;
const BASE_FREQUENCY: f32 = 300.0;
const FREQUENCY_INCREMENT: f32 = 200.0;

// waveform enum
#[derive(Clone, Copy)]
enum Waveform {
    Sine,
    Square,
    Triangle,
    Sawtooth,
}

// generate sample
fn generate_sample(t: f32, frequency: f32, waveform: &Waveform) -> f32 {
    match waveform {
        Waveform::Sine => (2.0 * PI * frequency * t).sin(),
        Waveform::Square => {
            if (2.0 * PI * frequency * t).sin() >= 0.0 {
                1.0
            } else {
                -1.0
            }
        }
        Waveform::Triangle => 2.0 * (2.0 * frequency * t).fract() - 1.0,
        Waveform::Sawtooth => 2.0 * (frequency * t).fract() - 1.0,
    }
}

// generate samples
fn generate_samples(
    frequency: f32,
    waveform: &Waveform,
    sample_rate: u32,
    duration: f32,
) -> Vec<i16> {
    let sample_count = (sample_rate as f32 * duration) as usize;
    let mut samples: Vec<f32> = Vec::with_capacity(sample_count);

    for t in 0..sample_count {
        let time = t as f32 / sample_rate as f32;
        let sample = generate_sample(time, frequency, waveform);
        samples.push(sample);
    }

    samples
        .iter()
        .map(|&s| (s * i16::MAX as f32) as i16)
        .collect()
}

fn main() {
    println!("Hello, world!");

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let mut sink = Sink::try_new(&stream_handle).unwrap();

    let sample_rate = 44100;
    let duration = 1.0;

    let waveform = Waveform::Sine;
    let mut frequency = BASE_FREQUENCY;

    let mut samples = generate_samples(frequency, &waveform, sample_rate, duration);
    let mut current_source =
        SamplesBuffer::<i16>::new(1, sample_rate, samples.clone()).repeat_infinite();
    sink.append(current_source);

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let stdin = io::stdin();
        let _stdout = io::stdout().into_raw_mode().unwrap();

        for key in stdin.keys() {
            match key.unwrap() {
                termion::event::Key::Char('a') => tx.send(0).unwrap(),
                termion::event::Key::Char('s') => tx.send(1).unwrap(),
                termion::event::Key::Char('d') => tx.send(2).unwrap(),
                termion::event::Key::Char('f') => tx.send(3).unwrap(),
                termion::event::Key::Char('g') => tx.send(4).unwrap(),
                termion::event::Key::Char('h') => tx.send(5).unwrap(),
                termion::event::Key::Char('j') => tx.send(6).unwrap(),
                termion::event::Key::Char('k') => tx.send(7).unwrap(),
                termion::event::Key::Char('l') => tx.send(8).unwrap(),
                termion::event::Key::Char('0') => {
                    tx.send(9).unwrap();
                    break;
                }
                _ => {}
            }
        }
    });

    loop {
        if let Ok(input) = rx.try_recv() {
            if input == 9 {
                break;
            }

            frequency = BASE_FREQUENCY + FREQUENCY_INCREMENT * input as f32;

            println!("Frequency: {}", frequency);

            samples = generate_samples(frequency, &waveform, sample_rate, duration);
            current_source =
                SamplesBuffer::<i16>::new(1, sample_rate, samples.clone()).repeat_infinite();
            sink.stop();
            sink.append(current_source);
            sink.play();
        }

        thread::sleep(Duration::from_millis(100));
    }

    println!("Goodbye!");
}
