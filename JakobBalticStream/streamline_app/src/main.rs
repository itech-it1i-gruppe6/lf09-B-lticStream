use iced::{Element, Application, Settings, Command, widget::{text, Column, button}};

struct App {
    count: u32,
}

impl Application for App {
    type Message = ();
    type Executor = iced::executor::Default;
    type Flags = ();
    type Theme = iced::Theme;

    fn new(_flags: ()) -> (App, iced::Command<Self::Message>) {
        (App {
            count: 0,
        }, Command::none())
    }

    fn title(&self) -> String {
        "Click Counter".into()
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        self.count += 1;
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        Column::new()
            .push(text(format!("Count: {}", self.count)))
            .push(button("increment").on_press(()))
            .into()
    }
}

fn main() -> iced::Result {
    App::run(Settings::default())
}
