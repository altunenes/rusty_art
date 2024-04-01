use iced::{
    widget::{button, column, text, Button, Column, Text},
    Alignment, Application, Command, Element, Settings, Theme,
};
use std::process::Command as SystemCommand; 
use std::thread;

pub fn main() -> iced::Result {
    MyApp::run(Settings::default())
}

struct MyApp {

}

#[derive(Debug, Clone, Copy)]
enum Message {
    RunEnigma,
    RunRainbow,
}
impl Application for MyApp {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();
    type Theme = iced::theme::Theme; 
    fn new(_flags: ()) -> (MyApp, Command<Self::Message>) {
        (MyApp {}, Command::none())
    }
    fn title(&self) -> String {
        String::from("Binary Runner")
    }
    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::RunEnigma => {
                thread::spawn(|| run_animation("enigma"));
            },
            Message::RunRainbow => {
                thread::spawn(|| run_animation("rainbow"));
            },
        }
        Command::none()
    }
    fn view(&self) -> Element<Self::Message> {
        Column::new()
            .padding(20)
            .align_items(Alignment::Center)
            .push(Button::new(Text::new("Run Enigma")).on_press(Message::RunEnigma))
            .push(Button::new(Text::new("Run Rainbow")).on_press(Message::RunRainbow))
            .into()
    }
}
fn run_animation(bin_name: &str) {
    let status = SystemCommand::new("cargo")
        .args(&["run", "--release", "--bin", bin_name])
        .status()
        .expect("Failed to execute command");

    if !status.success() {
        eprintln!("Error running animation: {}", bin_name);
    }
}
