use iced::{
    theme,
    widget::{self, canvas},
};

// use crate::Element;
use iced::Element;

pub struct Graph {
    points: Vec<f32>,
    scale: f32,
    cache: canvas::Cache,
}

impl<Message> canvas::Program<Message> for Graph {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer,
        theme: &iced::Theme,
        bounds: iced::Rectangle,
        _cursor: iced::advanced::mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let color = theme.extended_palette().secondary.strong.color;
        let geometry = self.cache.draw(renderer, bounds.size(), |frame| {
            let mut offset = frame.center();
            offset.x -= frame.size().width / 2.0;
            offset.y -= frame.size().height / 2.0;
            let mut x = 0.0;
            for i in 0..self.points.len() - 1 {
                let start = self.points[i] * self.scale;
                let end = self.points[i + 1] * self.scale;

                let x1 = x * frame.width() + offset.x;
                let y1 = start * frame.height() + offset.y;

                x += 1.0 / self.points.len() as f32;

                let x2 = x * frame.width() + offset.x;
                let y2 = end * frame.height() + offset.y;
                // let p2 = iced::Point::new(x, *end) + center;

                let path = canvas::Path::line(iced::Point::new(x1, y1), iced::Point::new(x2, y2));
                let stroke = canvas::Stroke {
                    style: canvas::Style::Solid(color),
                    width: 4.0,
                    line_cap: canvas::LineCap::Round,
                    line_join: canvas::LineJoin::Round,
                    line_dash: canvas::LineDash::default(),
                };
                frame.stroke(
                    &path,
                    stroke,
                )
            }
        });
        vec![geometry]
    }
}

impl Graph {
    pub fn new(points: Vec<f32>) -> Self {
        Self {
            points,
            scale: 1.0,
            cache: canvas::Cache::new(),
        }
    }

    pub fn scale(mut self: Self, scale: f32) -> Self {
        self.scale = scale;
        self
    }
}

impl<'a, Message: 'a> From<Graph> for Element<'a, Message> {
    fn from(value: Graph) -> Self {
        let canvas = canvas(value).width(iced::Length::Fill).height(iced::Length::Fill);
        let container = widget::container(canvas)
            .style(theme::Container::Box)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill);
        container.into()
    }
}
