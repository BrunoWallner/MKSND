mod graph;
mod modules;
mod widgets;

use iced::{theme::palette, Color, Command, Element, Settings};

use modules::ModuleMessage;
use modules::Modules;

use std::path::PathBuf;
use std::time::Duration;

#[derive(Clone, Copy, Debug)]
pub enum Page {
    Editor,
    Sequencer,
}

/// Runs application with these settings
#[rustfmt::skip]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let input = vec![
    //     ("Editor".into(), Page::Editor),
    //     ("Sequencer".into(), Page::Sequencer),
    // ];

    let settings = Settings::default();

    <App as iced::Application>::run(settings)?;

    Ok(())
}

/// Messages that are used specifically by our [`App`].
#[derive(Clone, Debug)]
pub enum Message {
    ButtonClick,
    Editor(ModuleMessage),
    Tick,
}

/// The [`App`] stores application-specific state.
pub struct App {
    editor: Modules,
    time: f32,
}

/// Implement [`cosmic::Application`] to integrate with COSMIC.
impl iced::Application for App {
    type Executor = iced::executor::Default;
    type Flags = Vec<(String, Page)>;
    type Message = Message;
    // type Theme = theme::Theme;
    type Theme = iced::Theme;

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        iced::time::every(Duration::from_millis(50)).map(|_| Message::Tick)
    }

    /// Creates the application, and optionally emits command on initialize.
    fn new(_input: Self::Flags) -> (Self, Command<Self::Message>) {
        let editor = Modules::new(PathBuf::from("./").canonicalize().unwrap());
        let app = App { editor, time: 0.0 };

        (app, iced::Command::none())
    }

    /// Handle application events here.
    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::ButtonClick => println!("clicked"),
            Message::Editor(e) => self.editor.update(e),
            Message::Tick => (),
        };
        self.time += 0.5;

        Command::none()
    }

    fn theme(&self) -> Self::Theme {
        // theme::Theme {}
        // iced::Theme::Dark
        let palette = iced::theme::Palette {
            background: Color::from_rgb(0.1, 0.1, 0.1),
            text: Color::WHITE,
            primary: Color::from_rgb(0.2, 0.2, 0.2),
            success: Color::from_rgb(0.5, 1.0, 0.5),
            danger: Color::from_rgb(1.0, 0.5, 0.5),
        };
        iced::Theme::custom_with_fn(String::from("theme"), palette, |palette| {
            palette::Extended {
                background: palette::Background {
                    base: palette::Pair::new(Color::from_rgb(0.1, 0.1, 0.1), palette.text),
                    weak: palette::Pair::new(Color::from_rgb(0.15, 0.15, 0.15), palette.text),
                    strong: palette::Pair::new(Color::from_rgb(0.2, 0.2, 0.2), palette.text),
                },
                primary: palette::Primary {
                    base: palette::Pair::new(Color::from_rgb(0.4, 0.4, 0.4), palette.text),
                    weak: palette::Pair::new(Color::from_rgb(0.2, 0.2, 0.2), palette.text),
                    strong: palette::Pair::new(Color::from_rgb(0.25, 0.25, 0.25), palette.text),
                },
                secondary: palette::Secondary {
                    base: palette::Pair::new(Color::from_rgb(0.2, 0.6, 0.4), palette.text),
                    weak: palette::Pair::new(Color::from_rgb(0.3, 0.6, 0.5), palette.text),
                    strong: palette::Pair::new(Color::from_rgb(0.1, 0.9, 1.0), palette.text),
                },
                success: palette::Success {
                    base: palette::Pair::new(Color::from_rgb(0.2, 0.6, 0.4), palette.text),
                    weak: palette::Pair::new(Color::from_rgb(0.4, 0.6, 0.5), palette.text),
                    strong: palette::Pair::new(Color::from_rgb(0.3, 0.6, 0.5), palette.text),
                },
                danger: palette::Danger {
                    base: palette::Pair::new(Color::from_rgb(1.0, 1.0, 1.0), palette.text),
                    weak: palette::Pair::new(Color::from_rgb(0.7, 0.7, 0.7), palette.text),
                    strong: palette::Pair::new(Color::from_rgb(0.8, 0.9, 1.0), palette.text),
                },
                is_dark: true,
            }
        })
    }

    /// Creates a view after each update.
    fn view(&self) -> Element<Self::Message> {
        // let element = match self.nav_model.active_data() {
        //     Some(Page::Editor) => self.editor.view(),
        //     _ => return Element::from(widget::text("TODO")),
        // };

        // element

        self.editor.view()
    }

    fn title(&self) -> String {
        String::from("MkSND")
    }
}
