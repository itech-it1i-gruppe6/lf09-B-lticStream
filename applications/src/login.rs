use iced::executor;
use iced::widget::{button, column, row, text, text_input};
use iced::{Application, Command, Element, Settings, Theme};

fn main() -> iced::Result {
    Login::run(Settings::default())
}

struct Login {
    username: String,
    password: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    Username(String),
    Password(String),
    LoginClick,
}

impl Application for Login {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(_flags: ()) -> (Login, Command<Self::Message>) {
        (
            Login {
                username: String::new(),
                password: String::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Cisco Client")
    }

    fn view(&self) -> Element<Self::Message> {
        column![
            text("B@ltic Networks"),
            row![
                text("username"),
                text_input("test", &self.username).on_input(Message::Username),
            ],
            row![
                text("password"),
                text_input("test", &self.password)
                    .on_input(Message::Password)
                    .password(),
            ],
            button("Log in").on_press(Message::LoginClick),
        ]
        .into()
    }

    fn update(&mut self, message: Message) -> Command<Self::Message> {
        match message {
            Message::Username(username) => {
                self.username = username;
                Command::none()
            }
            Message::Password(password) => {
                self.password = password;
                Command::none()
            }
            Message::LoginClick => todo!(),
        }
    }
}
