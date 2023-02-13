use {
    frost::{
        clickable::Clickable,
        text::*,
        wrap::{Direction, Wrap},
    },
    iced::{
        widget::{column, Slider},
        Alignment, Length, Sandbox,
    },
};

fn main() {
    //TODO slider to change fill limit
    Sink::run(Default::default()).unwrap()
}

#[derive(Debug, Clone)]
enum Message {
    Swap,
    Slide(u32),
}

struct Sink(Direction);

impl Sandbox for Sink {
    type Message = Message;

    fn new() -> Self {
        Sink(Direction::Row(3))
    }

    fn title(&self) -> String {
        "frost example".into()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::Swap => {
                self.0 = match self.0 {
                    Direction::Row(n) => Direction::Column(n),
                    Direction::Column(n) => Direction::Row(n),
                }
            }
            Message::Slide(n) => {
                self.0 = match self.0 {
                    Direction::Row(_) => Direction::Row(n),
                    Direction::Column(_) => Direction::Column(n),
                }
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        column![
            H4::new(format!(
                "Below is a Wrap with direction: {:?}, click on it to change direction",
                self.0
            )),
            Clickable::new(
                Wrap::with_children(
                    self.0,
                    vec![
                        H0::new("I am a H0").into(),
                        H1::new("I am a H1").into(),
                        H2::new("I am a H3").into(),
                        H3::new("I am a H3").into(),
                        H4::new("I am a H4").into(),
                    ]
                )
                .spacing(10)
            )
            .on_press(Message::Swap),
            Slider::new(
                1..=4_u32,
                match self.0 {
                    Direction::Row(n) | Direction::Column(n) => n,
                },
                Message::Slide
            )
        ]
        .spacing(10)
        .padding(10)
        .width(Length::Fill)
        .align_items(Alignment::Center)
        .into()
    }
}
