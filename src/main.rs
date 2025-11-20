use std::{
    fs::{self, File},
    io::BufReader,
};

use rodio::{Decoder, OutputStream, Sink};
use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;
use winput::{Action, message_loop};

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
    if act == Action::Release {
        s.push_str("_up");
    }
    if let Some(opt_path) = config.keyboard_bindings.get(&s) {
        if let Some(path) = opt_path {
            play_audio(path, sink);
        }
    }
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
        match receiver.try_next_event() {
            Some(message_loop::Event::Keyboard {
                vk,
                scan_code: _,
                action,
            }) => {
                keyboard_bindings(&config, vk, action, &mut audio.1, &mut last_action);
            }
            _ => {}
        }
    }
}
