use iced::widget::{button, column, row, text, text_input};
use iced::{Application, Command, Element, Settings, Theme, executor};

fn main() -> iced::Result {
    Mainpage::run(Settings::default())
}

struct Mainpage {
    ip_address: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    IpAddress(String),
    GetAllDevicesClick,
    GetAllClientsClick,
    GetHostCountClick,
    GetDeviceByIpClick,
}

impl Application for Mainpage {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(_flags: ()) -> (Mainpage, Command<Self::Message>) {
        (
            Mainpage {
                ip_address: String::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Cisco Client")
    }

    fn view(&self) -> Element<Self::Message> {
        column![
            text("Im folgenden sind Buttons, die verschiedene Daten in unserem Cisco-Netzwerk darstellen."),
            row![
                button("get all devices").on_press(Message::GetAllDevicesClick),
            ],
            row![
                button("get all clients").on_press(Message::GetAllClientsClick),
            ],
            row![
                button("get host count").on_press(Message::GetHostCountClick),
            ],
            row![
                text_input("ipv4-address", &self.ip_address)
                    .on_input(Message::IpAddress),button("device by ip").on_press(Message::GetDeviceByIpClick)
            ],
            text("Output"),
        ]
        .into()
    }

    fn update(&mut self, message: Message) -> Command<Self::Message> {
        match message {
            Message::GetDeviceByIpClick => Command::none(),
            Message::GetAllClientsClick => Command::none(),
            Message::IpAddress(ip_address) => {
                self.ip_address = ip_address;
                Command::none()
            }
            Message::GetAllDevicesClick => Command::none(),
            Message::GetHostCountClick => Command::none(),
        }
    }
}
