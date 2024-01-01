use iced::{widget::{
    button,
    column,
    container,
    text
}, Color, Application, Alignment, Element, Font, Settings, Theme, executor, Command, theme, application};

fn main() -> iced::Result {
    GUI::run(Settings {
        default_font: Font::MONOSPACE,
        antialiasing: true,
        ..Settings::default()
    })
}

//See https://github.com/0x192/universal-android-debloater/blob/main/src/gui/style.rs
//https://github.com/iced-rs/iced/blob/dd249a1d11c68b8fee1828d58bae158946ee2ebd/examples/solar_system/src/main.rs#L86

struct GUI {
    value: i32,
}

#[derive(Debug, Clone, Copy)]
enum Messages {
    IncrementPressed,
    DecrementPressed,
}

impl Application for GUI {
    type Executor = executor::Default;
    type Message = Messages;
    type Theme = Theme;
    type Flags = (); //events or user interactions, for example using pressing a button

    // //state when it starts
    fn new(_flags: ()) -> (GUI, Command<Self::Message>) {
        (Self { value: 0 }, Command::none())
    }

    fn title(&self) -> String {
        String::from("My first app!")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Messages::IncrementPressed => {
                self.value += 1;
            }
            Messages::DecrementPressed => {
                self.value -= 1;
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        //text("Hello, iced!").into() //into() = turn any widget into generic widget for view

        container(
            column![
                button("Increment").on_press(Messages::IncrementPressed),
                text(self.value).size(50),
                button("Decrement").on_press(Messages::DecrementPressed),
            ]
                .padding(20)
                .align_items(Alignment::Center)
        )
            .padding(10)
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn style(&self) -> theme::Application {
        fn dark_background(_theme: &Theme) -> application::Appearance {
            application::Appearance {
                background_color: Color::BLACK,
                text_color: Color::WHITE,
            }
        }

        theme::Application::from(dark_background as fn(&Theme) -> _)
    }
}