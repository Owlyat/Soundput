use std::{
    fs::{self, File},
    io::BufReader,
};

use rodio::{Decoder, OutputStream, Sink};
use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;
use winput::{Action, Vk, message_loop};

const ARROW_ACTION: &[Vk] = &[Vk::UpArrow, Vk::DownArrow, Vk::LeftArrow, Vk::RightArrow];

const DIGITS_ACTION: &[Vk] = &[
    /* Vk::_0,
    Vk::_1,
    Vk::_2,
    Vk::_3,
    Vk::_4,
    Vk::_5,
    Vk::_6,
    Vk::_7,
    Vk::_8,
    Vk::_9, */
    Vk::Numpad0,
    Vk::Numpad1,
    Vk::Numpad2,
    Vk::Numpad3,
    Vk::Numpad4,
    Vk::Numpad5,
    Vk::Numpad6,
    Vk::Numpad7,
    Vk::Numpad8,
    Vk::Numpad9,
];

const ALPHABETIC_ACTION: &[Vk] = &[
    Vk::A,
    Vk::B,
    Vk::C,
    Vk::D,
    Vk::E,
    Vk::F,
    Vk::G,
    Vk::H,
    Vk::I,
    Vk::J,
    Vk::K,
    Vk::L,
    Vk::M,
    Vk::N,
    Vk::O,
    Vk::P,
    Vk::Q,
    Vk::R,
    Vk::S,
    Vk::T,
    Vk::U,
    Vk::V,
    Vk::W,
    Vk::X,
    Vk::Y,
    Vk::Z,
];

#[derive(Deserialize)]
struct Config {
    keyboard_bindings: HashMap<String, Option<PathBuf>>,
}

fn keyboard_bindings(
    config: &Config,
    vk: winput::Vk,
    action: Action,
    sink: &mut Sink,
    last_action: &mut Vec<(winput::Vk, Action)>,
) {
    last_action.pop_if(|(_, act)| *act == Action::Release);
    if last_action.contains(&(vk, action)) {
        return;
    }
    println!("{vk:?}");
    get_audio_and_play(config, sink, format!("{vk:?}").to_lowercase(), action);
    last_action.pop_if(|(_, act)| *act == Action::Press);
    last_action.push((vk, action));
}

fn get_audio_and_play(config: &Config, sink: &mut Sink, s: impl Into<String>, act: Action) {
    let mut s: String = s.into();
    let mut default_s: String = "default_key".into();
    let mut digit_s: String = "digit_key".into();
    let mut alphabetic_s: String = "alphabet_key".into();
    let mut arrow_s: String = "arrow_key".into();
    if act == Action::Release {
        s.push_str("_up");
        default_s.push_str("_up");
        digit_s.push_str("_up");
        alphabetic_s.push_str("_up");
        arrow_s.push_str("_up");
    }
    println!(
        "Action: {s} | is_digit: {} => {digit_s} | is_alphabet: {} => {alphabetic_s}",
        is_digit(&s),
        is_alphabet(&s)
    );
    if let Some(opt_path) = config.keyboard_bindings.get(&s) {
        if let Some(path) = opt_path {
            play_audio(path, sink);
            return;
        }
    } else if is_digit(&s) {
        if let Some(opt_path) = config.keyboard_bindings.get(&digit_s) {
            if let Some(path) = opt_path {
                play_audio(path, sink);
                return;
            }
        }
    } else if is_alphabet(&s) {
        if let Some(opt_path) = config.keyboard_bindings.get(&alphabetic_s) {
            if let Some(path) = opt_path {
                play_audio(path, sink);
                return;
            }
        } else {
            println!("No audio found as {alphabetic_s}");
        }
    } else if is_arrow_key(&s) {
        if let Some(opt_path) = config.keyboard_bindings.get(&arrow_s) {
            if let Some(path) = opt_path {
                play_audio(path, sink);
                return;
            }
        }
    }

    if let Some(opt_path) = config.keyboard_bindings.get(&default_s) {
        if let Some(path) = opt_path {
            play_audio(path, sink);
        }
    }
}

fn is_digit(i: &String) -> bool {
    DIGITS_ACTION
        .iter()
        .find(|v| {
            format!("{:?}", v).to_lowercase() == *i || format!("{:?}_up", v).to_lowercase() == *i
        })
        .is_some()
}

fn is_alphabet(i: &String) -> bool {
    ALPHABETIC_ACTION
        .iter()
        .find(|v| {
            format!("{:?}", v).to_lowercase() == *i || format!("{:?}_up", v).to_lowercase() == *i
        })
        .is_some()
}

fn is_arrow_key(i: &String) -> bool {
    ARROW_ACTION
        .iter()
        .find(|v| {
            format!("{:?}", v).to_lowercase() == *i || format!("{:?}_up", v).to_lowercase() == *i
        })
        .is_some()
}

fn audio_innit() -> (OutputStream, Sink) {
    let mut stream_handle =
        rodio::OutputStreamBuilder::open_default_stream().expect("Open default audio stream");
    stream_handle.log_on_drop(false);
    let sink = rodio::Sink::connect_new(stream_handle.mixer());
    (stream_handle, sink)
}

fn play_audio(file: &PathBuf, sink: &mut Sink) {
    let file = std::env::home_dir()
        .expect("Could not get HomeDir")
        .join(".config/Soundput/")
        .join(file);
    let file = BufReader::new(
        File::open(&file).expect(&format!("Could not open file {}", file.to_string_lossy())),
    );
    if let Ok(src) = Decoder::try_from(file) {
        sink.skip_one();
        sink.append(src);
    }
}

fn get_default_config() -> Config {
    let mut toml_path = std::env::home_dir().expect("Could not get HomeDir");
    toml_path.push(".config/Soundput/config.toml");
    assert!(toml_path.exists());
    let file = fs::read(toml_path)
        .expect("Could not read config.toml")
        .iter()
        .map(|c| *c as char)
        .collect::<String>();
    let config: Config = toml::from_str(&file).expect("Config.toml is not to a proper format!");
    config
}

fn main() {
    let config = get_default_config();
    let receiver = message_loop::start().expect("Could not start message loop");
    let mut last_action = Vec::default();
    let mut audio = audio_innit();
    loop {
        match receiver.next_event() {
            message_loop::Event::Keyboard {
                vk,
                scan_code: _,
                action,
            } => {
                keyboard_bindings(&config, vk, action, &mut audio.1, &mut last_action);
            }
            _ => {}
        }
    }
}
