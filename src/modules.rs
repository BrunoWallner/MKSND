use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

use crate::widgets::{self, menu};
use crate::{graph, widgets::Menu, Message};
use iced::widget;
use iced::widget::{
    text_editor::{Action, Content},
    TextEditor,
};
use iced::Element;
// use crate::Element;

#[derive(Clone, Debug)]
pub enum ModuleMessage {
    Save,
    SelectModule(String),
    RemoveModule(String),
    Editor(Action),
    AddModule,
    AddModuleInput(String),
    RunModule,
}

pub struct Modules {
    path: PathBuf,
    content: Content,
    module_add_text: String,
    modules: HashMap<String, String>,
    files: Menu<Message, String>,
    executor: Result<bs::executor::Executor, String>,
}
impl Modules {
    pub fn new(path: PathBuf) -> Self {
        let path = path.join(PathBuf::from("modules"));
        let _ = fs::create_dir_all(&path);
        let path = path.canonicalize().unwrap();
        let mut modules = Self {
            content: Content::new(),
            modules: HashMap::default(),
            module_add_text: String::new(),
            path,
            files: Menu::new(
                Vec::new(),
                |module| Message::Editor(ModuleMessage::SelectModule(module)),
                menu::Flow::Vertical,
            ),
            executor: Err(String::new()),
        };
        modules.load_modules().unwrap();
        modules.files.set_elements(modules.get_file_elements());
        modules
    }

    pub fn save_modules(&mut self) -> io::Result<()> {
        if let Some(module) = &self.files.selected() {
            if let Some(module) = self.modules.get_mut(module) {
                *module = self.content.text();
            }
        }

        for (path, content) in self.modules.iter() {
            let path = self.path.join(PathBuf::from(&path));
            let dir_path = path.parent().unwrap();
            let _ = fs::create_dir_all(dir_path);

            let _ = fs::remove_file(&path);
            let mut file = fs::File::create(path)?;
            if !content.is_empty() {
                file.write_all(&content.clone().into_bytes())?;
            }
        }

        Ok(())
    }

    fn get_file_elements(&self) -> Vec<widgets::menu::Element<String>> {
        let mut options = self.modules.clone().into_keys().collect::<Vec<_>>();

        options.sort_unstable();

        // options
        options
            .iter()
            .map(|o| widgets::menu::Element {
                data: o.clone(),
                text: o
                    .trim_start_matches(&self.path.to_string_lossy().to_string())
                    .into(),
            })
            .collect()
    }

    pub fn load_modules(&mut self) -> io::Result<()> {
        let dir = match fs::read_dir(&self.path) {
            Ok(dir) => dir,
            Err(_) => {
                fs::create_dir_all(&self.path)?;
                return Ok(());
            }
        };
        read_dir(dir, &mut self.modules);

        fn read_dir(dir: fs::ReadDir, modules: &mut HashMap<String, String>) {
            for entry in dir.into_iter() {
                let Ok(entry) = entry else { continue };
                let Ok(file_type) = entry.file_type() else {
                    continue;
                };
                if file_type.is_file() {
                    let Ok(name) = entry.path().canonicalize() else {
                        continue;
                    };

                    let name = name.into_os_string().to_string_lossy().to_string();
                    // let name = name.trim_start_matches(&self.path.to_string_lossy().to_string());
                    let Ok(content) = fs::read_to_string(entry.path()) else {
                        continue;
                    };
                    modules.insert(name, content);
                } else if file_type.is_dir() {
                    let Ok(dir) = fs::read_dir(entry.path()) else {
                        continue;
                    };
                    read_dir(dir, modules)
                }
            }
        }

        Ok(())
    }

    pub fn view<'a>(&'a self) -> Element<'a, Message> {
        let content = widget::row([self.file_select().into(), self.text_editor().into()])
            .spacing(iced::Pixels(10.0))
            .padding(iced::Padding::new(10.0));

        Element::from(content)
    }

    pub fn update(&mut self, message: ModuleMessage) {
        match message {
            ModuleMessage::Editor(action) => self.content.perform(action),
            ModuleMessage::AddModule => {
                if self.module_add_text.is_empty() {
                    return;
                }
                let path = self.path.join(PathBuf::from(self.module_add_text.clone()));

                let _ = fs::create_dir_all(&path.parent().unwrap());

                if !(fs::File::open(&path).is_ok() || fs::File::create(&path).is_ok()) {
                    return;
                }
                let path = path.to_string_lossy().to_string();
                self.module_add_text.clear();
                self.modules.insert(path, String::new());
                self.files.set_elements(self.get_file_elements());
            }
            ModuleMessage::AddModuleInput(input) => {
                let input = input.replace(" ", "_");
                self.module_add_text = input
            }
            ModuleMessage::Save => self.save_modules().unwrap(),
            ModuleMessage::SelectModule(module) => {
                if let Some(module) = &self.files.selected() {
                    if let Some(module) = self.modules.get_mut(module) {
                        *module = self.content.text();
                    }
                }

                // self.selected_module = Some(module.clone());
                self.files.select(module.clone());

                if let Some(module) = self.modules.get(&module) {
                    self.content = Content::with_text(module);
                }
            }
            ModuleMessage::RemoveModule(_module) => {
                // self.module_nav_model.remove(entity);
            }
            ModuleMessage::RunModule => {
                let module = self.content.text();
                let tokens = bs::lexer::tokenize(&module);
                let ast = match bs::parser::parse(tokens) {
                    Ok(a) => bs::parser::Ast::new(a),
                    Err(e) => {
                        let e = e.format_with(&module, "parse error", false);
                        self.executor = Err(e);
                        return;
                    }
                };
                let executor = match bs::executor::Executor::build(ast) {
                    Ok(e) => e,
                    Err(e) => {
                        let e = e.format_with(&module, "parse error", false);
                        self.executor = Err(e);
                        return;
                    }
                };

                self.executor = Ok(executor);
            }
        };
    }

    fn output<'a>(&'a self) -> Element<'a, Message> {
        let inner: Element<'_, _> = match &self.executor {
            Ok(e) => {
                // let points = (0..100)
                //     .into_iter()
                //     .map(|x| (x as f32 * 0.5).sin() + 1.0)
                //     .collect();
                let points = (0..100)
                    .into_iter()
                    .map(|x| {
                        let input = x as f64 / 100.0;
                        match e.execute("main", vec![&input]) {
                            Ok(value) => match value {
                                Some(v) => match v {
                                    bs::data::Value::Data(d) => match d {
                                        bs::data::DataType::Float(f) => f as f32,
                                        _ => 0.0
                                    },
                                    _ => 0.0,
                                },
                                None => 0.0,
                            },
                            Err(_) => 0.0,
                        }
                    })
                    .collect::<Vec<_>>();
                let graph = graph::Graph::new(points).scale(0.5);

                graph.into()
            }
            Err(msg) => widget::text(msg).into(),
        };

        let inner = widget::container(inner)
            .height(iced::Length::FillPortion(1))
            .width(iced::Length::Fill)
            .style(iced::theme::Container::Box);

        inner.into()
    }

    fn text_editor<'a>(&'a self) -> Element<'a, Message> {
        let mut text = TextEditor::new(&self.content)
            .padding(iced::Padding::new(10.0))
            .height(iced::Length::Fill);
        if self.files.selected().is_some() {
            text = text.on_action(|a| Message::Editor(ModuleMessage::Editor(a)));
        }
        let text = widget::container(text).height(iced::Length::FillPortion(2));
        let output = self.output();

        let content =
            widget::Column::with_children([text.into(), output]).spacing(iced::Pixels::from(5.0));
        // let content = text;

        let centered = widget::container(content)
            .height(iced::Length::Fill)
            .align_x(iced::alignment::Horizontal::Center)
            .align_y(iced::alignment::Vertical::Center);

        Element::from(centered)
    }

    fn file_select<'a>(&'a self) -> Element<'a, Message> {
        let add_module = widget::text_input("add module", self.module_add_text.as_str())
            .on_input(|input| Message::Editor(ModuleMessage::AddModuleInput(input)))
            .on_submit(Message::Editor(ModuleMessage::AddModule));

        let save = widget::button(widget::text("SAVE"))
            .on_press(Message::Editor(ModuleMessage::Save))
            .width(iced::Length::Fill);
        let save = widget::container(save)
            .height(iced::Length::Shrink)
            .width(iced::Length::Fill)
            .align_x(iced::alignment::Horizontal::Center)
            .align_y(iced::alignment::Vertical::Center);

        let files = self.files.clone();

        let run = widget::button(widget::text("RUN"))
            .on_press(Message::Editor(ModuleMessage::RunModule))
            .width(iced::Length::Fill);

        // let content = widget::list_column().add(save).add(add_module).add(files);
        let content = widget::column([
            run.into(),
            save.into(),
            add_module.into(),
            widget::vertical_space()
                .height(iced::Length::Fixed(10.0))
                .into(),
            files.into(),
        ]);
        let content = widget::container(content).width(iced::Length::Fixed(200.0));

        Element::from(content)
    }
}
