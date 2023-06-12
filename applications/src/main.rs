use iced::executor;
use iced::widget::{button, column, row, text, text_input};
use iced::{Application, Command, Element, Settings, Theme};
use iced_futures;

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
    type Executor = iced_futures::backend::native::tokio;
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

    fn theme(&self) -> Self::Theme {
        Self::Theme::default()
    }

    fn style(&self) -> <Self::Theme as iced::application::StyleSheet>::Style {
        <Self::Theme as iced::application::StyleSheet>::Style::default()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        iced::Subscription::none()
    }

    fn scale_factor(&self) -> f64 {
        1.0
    }

    fn run(settings: Settings<Self::Flags>) -> iced::Result
    where
        Self: 'static,
    {
        #[allow(clippy::needless_update)]
        let renderer_settings = iced::renderer::Settings {
            default_font: settings.default_font,
            default_text_size: settings.default_text_size,
            text_multithreading: settings.text_multithreading,
            antialiasing: if settings.antialiasing {
                Some(iced::renderer::settings::Antialiasing::MSAAx4)
            } else {
                None
            },
            ..iced::renderer::Settings::from_env()
        };

        Ok(iced::runtime::application::run::<
            Instance<Self>,
            Self::Executor,
            iced::renderer::window::Compositor<Self::Theme>,
        >(settings.into(), renderer_settings)?)
    }
}
