use std::{
    cell::OnceCell,
    fs::{self, File},
    io::BufReader,
    sync::{Arc, Mutex},
};

use crossbeam_channel::{select, select_biased};
use rodio::{Decoder, OutputStream, Sink};
use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;
use tray_icon::{
    Icon, MouseButton, TrayIcon, TrayIconBuilder, TrayIconEvent,
    menu::{Menu, MenuEvent, MenuItem},
};
use winit::{application::ApplicationHandler, event_loop::EventLoop};
use winput::{
    Action, Vk,
    message_loop::{self, EventReceiver},
};

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

struct Application {
    tray_icon: TrayIcon,
    audio: (OutputStream, Sink),
    last_action: Vec<(Vk, Action)>,
    config: Config,
}

impl Default for Application {
    fn default() -> Self {
        Self {
            tray_icon: TrayIconBuilder::new()
                .with_icon(
                    tray_icon::Icon::from_resource_name("app-icon", None)
                        .expect("Error loading App Icon"),
                )
                .with_tooltip("Soundput - LeftClick: Reload | RightClick: Quit")
                .build()
                .expect("Error building Tray"),
            audio: audio_innit(),
            last_action: vec![],
            config: get_default_config(),
        }
    }
}

#[derive(Debug)]
enum UserEvent {
    TrayIconEvent(tray_icon::TrayIconEvent),
    MenuEvent(tray_icon::menu::MenuEvent),
    Event(message_loop::Event),
}

impl ApplicationHandler<UserEvent> for Application {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {}

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
    }

    fn new_events(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        cause: winit::event::StartCause,
    ) {
    }

    fn user_event(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop, event: UserEvent) {
        match event {
            UserEvent::TrayIconEvent(tray_icon_event) => match tray_icon_event {
                TrayIconEvent::Click {
                    id,
                    position,
                    rect,
                    button,
                    button_state,
                } => {
                    if button == MouseButton::Right {
                        println!("Right Clicked!");
                        std::process::exit(0);
                    }
                    if button == MouseButton::Left {
                        println!("Left Clicked!");
                        self.config = get_default_config();
                    }
                }
                TrayIconEvent::DoubleClick {
                    id,
                    position,
                    rect,
                    button,
                } => {}
                TrayIconEvent::Enter { id, position, rect } => {}
                TrayIconEvent::Move { id, position, rect } => {}
                TrayIconEvent::Leave { id, position, rect } => {}
                _ => {}
            },
            UserEvent::MenuEvent(menu_event) => match menu_event {
                MenuEvent { id } => match id.0.as_str() {
                    _ => {
                        println!("id: {}", id.0);
                    }
                },
            },
            UserEvent::Event(event) => match event {
                message_loop::Event::Keyboard {
                    vk,
                    scan_code,
                    action,
                } => {
                    keyboard_bindings(
                        &self.config,
                        vk,
                        action,
                        &mut self.audio.1,
                        &mut self.last_action,
                    );
                }
                message_loop::Event::MouseMoveRelative { x, y } => {}
                message_loop::Event::MouseMoveAbsolute { x, y, virtual_desk } => {}
                message_loop::Event::MouseButton { action, button } => {}
                message_loop::Event::MouseWheel { delta, direction } => {}
            },
        }
    }
}

fn main() {
    let mut app = Application::default();
    let event_loop = EventLoop::<UserEvent>::with_user_event()
        .build()
        .expect("Failed to build an event loop");
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Wait);
    let proxy = event_loop.create_proxy();
    TrayIconEvent::set_event_handler(Some(move |event| {
        let _ = proxy.send_event(UserEvent::TrayIconEvent(event));
    }));
    let proxy = event_loop.create_proxy();
    MenuEvent::set_event_handler(Some(move |event| {
        let _ = proxy.send_event(UserEvent::MenuEvent(event));
    }));

    let proxy = event_loop.create_proxy();
    let receiver = message_loop::start().expect("Could not start message loop");
    std::thread::spawn(move || {
        while let Ok(event) = receiver.receiver().recv() {
            let _ = proxy.send_event(UserEvent::Event(event));
        }
    });

    event_loop
        .run_app(&mut app)
        .is_err()
        .then(|| std::process::exit(1));
}
