use iced::{widget, Element as IcedElement};

#[derive(Clone, Debug, PartialEq)]
pub struct Element<Data: Clone + PartialEq> {
    pub data: Data,
    pub text: String,
}

#[derive(Clone, Debug,PartialEq, Eq)]
pub enum Flow {
    Vertical,
    Horizontal,
}

#[derive(Clone, Debug)]
pub struct Menu<Message, Data: Clone + PartialEq> {
    elements: Vec<Element<Data>>,
    on_select: fn(Data) -> Message,
    selected: Option<Data>,
    flow: Flow,
    element_size: f32,
}

impl<Message: Clone, Data: Clone + PartialEq> Menu<Message, Data> {
    pub fn new(elements: Vec<Element<Data>>, on_select: fn(Data) -> Message, flow: Flow) -> Self {
        Self {
            elements,
            on_select,
            selected: None,
            flow,
            element_size: 200.0,
        }
    }

    pub fn select(&mut self, selection: Data) {
        self.selected = Some(selection)
    }

    pub fn selected(&self) -> &Option<Data> {
        &self.selected
    }

    pub fn set_elements(&mut self, elements: Vec<Element<Data>>) {
        self.elements = elements;
    }
}

impl<'a, Message: Clone + 'a, Data: Clone + PartialEq + 'a> From<Menu<Message, Data>>
    for IcedElement<'a, Message>
{
    fn from(menu: Menu<Message, Data>) -> Self {
        let options = menu
            .elements
            .iter()
            .map(|key| {
                let mut content = widget::button(widget::text(key.text.clone()))
                    // .on_press(Message::Editor(ModuleMessage::SelectModule(key.clone())))
                    .on_press((menu.on_select)(key.data.clone()));
                // .width(iced::Length::Fill);

                if Some(key.data.clone()) == menu.selected {
                    content = content.style(iced::theme::Button::Positive);
                }
                content = content
                    .height(iced::Length::Shrink)
                    .width(iced::Length::Shrink);
                match menu.flow {
                    Flow::Vertical => {
                        content = content.width(iced::Length::Fixed(menu.element_size))
                    }
                    Flow::Horizontal => {
                        // content = content.width(iced::Length::Fixed(menu.element_size))
                        content = content.width(iced::Length::FillPortion(1))
                        // ()
                    }
                }

                content.into()
            })
            .collect::<Vec<IcedElement<_>>>();

        let content: IcedElement<'_, Message> = match menu.flow {
            Flow::Vertical => widget::column(options).into(),
            Flow::Horizontal => widget::row(options).into(),
        };
        let mut content: IcedElement<'_, Message>  = widget::container(content)
            .height(iced::Length::Shrink)
            .width(iced::Length::Shrink)
            .into();
        if menu.flow == Flow::Vertical {
            content =
                widget::scrollable(content).direction(widget::scrollable::Direction::Both {
                    vertical: widget::scrollable::Properties::new(),
                    horizontal: widget::scrollable::Properties::new(),
                }).into();
        }
        content.into()
    }
}
