use iced::{
    widget::{Column, Row},
    Alignment, Element, Length,
};

///The main direction of [`Wrap`]
///
///Funtions similar to `flex-direction` in CSS flexbox. A wrap will grow in the specified
///and wrap every n elements as specified by the contained value
#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Row(u32),
    Column(u32),
}

///A container that distributes its contents in both directions by wrapping
pub struct Wrap<'a, Message, Theme = iced::Theme, Renderer = iced::Renderer> {
    direction: Direction,
    width: Length,
    height: Length,
    spacing: u16,
    padding: u16,
    children: Vec<Element<'a, Message, Theme, Renderer>>,
}

impl<'a, Message, Theme, Renderer> Wrap<'a, Message, Theme, Renderer> {
    ///Creates an empty [`Wrap`]
    pub fn new(direction: Direction) -> Self {
        Self::with_children(direction, Vec::new())
    }

    ///Creates a [`Wrap`] with the given elements
    pub fn with_children(
        direction: Direction,
        children: Vec<Element<'a, Message, Theme, Renderer>>,
    ) -> Self {
        Self {
            direction,
            width: Length::Shrink,
            height: Length::Shrink,
            spacing: 0,
            padding: 0,
            children,
        }
    }

    ///Sets the width of the [`Wrap`]
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    ///Sets the height of the [`Wrap`]
    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    ///Sets the spacing *between* elements
    pub fn spacing(mut self, units: u16) -> Self {
        self.spacing = units;
        self
    }

    ///Sets the padding of the [`Wrap`]
    pub fn padding(mut self, units: u16) -> Self {
        self.padding = units;
        self
    }

    ///Adds an [`Element`] to the [`Wrap`]
    pub fn push<E>(mut self, child: E) -> Self
    where
        E: Into<Element<'a, Message, Theme, Renderer>>,
    {
        self.children.push(child.into());
        self
    }
}

impl<'a, Message, Theme, Renderer> From<Wrap<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: 'a,
    Renderer: iced::advanced::Renderer + 'a,
{
    fn from(wrap: Wrap<'a, Message, Theme, Renderer>) -> Self {
        //defining the macro here allows it to access wrap without passing it
        macro_rules! resolve {
            //Double brackets to allow a more complex macro
            //blocks are considered a single expression
            ($limit:ident, $main:ident, $cross:ident) => {{
                let mut cross = $cross::new()
                    .align_items(Alignment::Center)
                    .width(wrap.width)
                    .height(wrap.height)
                    .padding(wrap.padding)
                    .spacing(wrap.spacing);
                let mut iter = wrap.children.into_iter().peekable();

                while iter.peek().is_some() {
                    let mut main = $main::new().spacing(wrap.spacing);

                    for _ in 0..$limit {
                        let Some(child) = iter.next() else { break };
                        main = main.push(child);
                    }

                    cross = cross.push(main);
                }

                cross.into()
            }};
        }

        match wrap.direction {
            Direction::Row(limit) => resolve!(limit, Row, Column),
            Direction::Column(limit) => resolve!(limit, Column, Row),
        }
    }
}
