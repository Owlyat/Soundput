use std::{
    collections::HashMap,
    fs::{self, File},
    io::BufReader,
    path::PathBuf,
};

use rodio::{Decoder, OutputStream, Sink};
use serde::Deserialize;
use tray_icon::{MouseButton, TrayIcon, TrayIconBuilder, TrayIconEvent, menu::MenuEvent};
use winit::{application::ApplicationHandler, event_loop::EventLoop};
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

fn default_break() -> bool {
    true
}

#[derive(Deserialize)]
struct Config {
    #[serde(rename = "break", default = "default_break")]
    break_audio: bool,
    keyboard_bindings: HashMap<String, Option<PathBuf>>,
}

fn keyboard_bindings(
    config: &Config,
    vk: winput::Vk,
    action: Action,
    audio: &mut Audio,
    last_action: &mut HashMap<winput::Vk, Action>,
) {
    if last_action.get(&vk) == Some(&action) {
        return;
    }
    last_action.insert(vk, action);
    println!("{vk:?}");
    get_audio_and_play(config, audio, format!("{vk:?}").to_lowercase(), action);
}

fn get_audio_and_play(config: &Config, audio: &mut Audio, s: impl Into<String>, act: Action) {
    let suffix = if act == Action::Release { "_up" } else { "" };
    let s = format!("{}{suffix}", s.into());
    let default_s = format!("default_key{suffix}");

    let path = match config.keyboard_bindings.get(&s) {
        Some(Some(path)) => Some(path),
        Some(None) => config
            .keyboard_bindings
            .get(&default_s)
            .and_then(|p| p.as_ref()),
        None => {
            let category = if is_digit(&s) {
                "digit_key"
            } else if is_alphabet(&s) {
                "alphabet_key"
            } else if is_arrow_key(&s) {
                "arrow_key"
            } else {
                ""
            };
            let category_path = (!category.is_empty())
                .then(|| format!("{category}{suffix}"))
                .and_then(|key| config.keyboard_bindings.get(&key).and_then(|p| p.as_ref()));
            category_path.or_else(|| {
                config
                    .keyboard_bindings
                    .get(&default_s)
                    .and_then(|p| p.as_ref())
            })
        }
    };

    if let Some(path) = path {
        play_audio(path, audio, config.break_audio);
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

struct Audio {
    stream: OutputStream,
    sinks: Vec<(PathBuf, Sink)>,
}

fn audio_innit() -> Audio {
    let mut stream =
        rodio::OutputStreamBuilder::open_default_stream().expect("Open default audio stream");
    stream.log_on_drop(false);
    Audio {
        stream,
        sinks: Vec::new(),
    }
}

fn play_audio(file: &PathBuf, audio: &mut Audio, break_audio: bool) {
    let path = std::env::home_dir()
        .expect("Could not get HomeDir")
        .join(".config/Soundput/")
        .join(file);
    let file = BufReader::new(
        File::open(&path)
            .unwrap_or_else(|err| panic!("Could not open file {}: {err}", path.to_string_lossy())),
    );
    if let Ok(src) = Decoder::try_from(file) {
        if break_audio {
            audio.sinks.clear();
        } else {
            audio.sinks.retain(|(_, sink)| !sink.empty());
            audio.sinks.retain(|(p, _)| p != &path);
        }
        let sink = Sink::connect_new(audio.stream.mixer());
        sink.append(src);
        audio.sinks.push((path, sink));
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
    _tray_icon: TrayIcon,
    audio: Audio,
    last_action: HashMap<Vk, Action>,
    config: Config,
}

impl Default for Application {
    fn default() -> Self {
        Self {
            _tray_icon: TrayIconBuilder::new()
                .with_icon(
                    tray_icon::Icon::from_resource_name("app-icon", None)
                        .expect("Error loading App Icon"),
                )
                .with_tooltip("Soundput - LeftClick: Reload | RightClick: Quit")
                .build()
                .expect("Error building Tray"),
            audio: audio_innit(),
            last_action: HashMap::new(),
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
    fn resumed(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {}

    fn window_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        _event: winit::event::WindowEvent,
    ) {
    }

    fn new_events(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _cause: winit::event::StartCause,
    ) {
    }

    fn user_event(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop, event: UserEvent) {
        match event {
            UserEvent::TrayIconEvent(tray_icon_event) => match tray_icon_event {
                TrayIconEvent::Click { button, .. } => {
                    if button == MouseButton::Right {
                        println!("Right Clicked!");
                        std::process::exit(0);
                    }
                    if button == MouseButton::Left {
                        println!("Left Clicked!");
                        self.config = get_default_config();
                    }
                }
                TrayIconEvent::DoubleClick { .. } => {}
                TrayIconEvent::Enter { .. } => {}
                TrayIconEvent::Move { .. } => {}
                TrayIconEvent::Leave { .. } => {}
                _ => {}
            },
            UserEvent::MenuEvent(MenuEvent { id }) => {
                println!("id: {}", id.0);
            }
            UserEvent::Event(event) => match event {
                message_loop::Event::Keyboard { vk, action, .. } => {
                    keyboard_bindings(
                        &self.config,
                        vk,
                        action,
                        &mut self.audio,
                        &mut self.last_action,
                    );
                }
                message_loop::Event::MouseMoveRelative { .. } => {}
                message_loop::Event::MouseMoveAbsolute { .. } => {}
                message_loop::Event::MouseButton { .. } => {}
                message_loop::Event::MouseWheel { .. } => {}
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
