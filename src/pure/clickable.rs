use {
    iced_native::{alignment, Length, Padding, Renderer},
    iced_pure::{
        widget::{
            button::{Button, Style, StyleSheet},
            container::{self, Container},
        },
        Element,
    },
};

///A clickable `Container`
pub struct Clickable<'a, M, R> {
    content: Container<'a, M, R>,
    msg: Option<M>,
    height: Length,
    width: Length,
}

impl<'a, M, R> Clickable<'a, M, R>
where
    M: 'a + Clone,
    R: Renderer,
{
    /// Creates an empty `Clickable`.
    pub fn new<T>(content: T) -> Self
    where
        T: Into<Element<'a, M, R>>,
    {
        Self {
            content: Container::new(content),
            msg: None,
            height: Length::Shrink,
            width: Length::Shrink,
        }
    }

    ///Sets the message that will be produced when the `Clickable` is pressed.
    ///
    ///Unless `on_press` is called, nothing will happen.
    pub fn on_press(mut self, msg: M) -> Self {
        self.msg = Some(msg);
        self
    }

    /// Sets the `Padding` of the `Clickable`.
    pub fn padding<P: Into<Padding>>(mut self, padding: P) -> Self {
        self.content = self.content.padding(padding);
        self
    }

    /// Sets the width of the `Clickable`.
    pub fn width(mut self, width: Length) -> Self {
        self.content = self.content.width(width);
        self.width = width;
        self
    }

    /// Sets the height of the `Clickable`.
    pub fn height(mut self, height: Length) -> Self {
        self.content = self.content.height(height);
        self.height = height;
        self
    }

    /// Sets the content alignment for the horizontal axis of the `Clickable`.
    pub fn align_x(mut self, alignment: alignment::Horizontal) -> Self {
        self.content = self.content.align_x(alignment);
        self
    }

    /// Sets the content alignment for the vertical axis of the `Clickable`.
    pub fn align_y(mut self, alignment: alignment::Vertical) -> Self {
        self.content = self.content.align_y(alignment);
        self
    }

    /// Centers the contents in the horizontal axis of the `Clickable`.
    pub fn center_x(mut self) -> Self {
        self.content = self.content.center_x();
        self
    }

    /// Centers the contents in the vertical axis of the `Clickable`.
    pub fn center_y(mut self) -> Self {
        self.content = self.content.center_y();
        self
    }

    /// Sets the style of the `Clickable`.
    pub fn style(mut self, style_sheet: impl Into<Box<dyn container::StyleSheet + 'a>>) -> Self {
        self.content = self.content.style(style_sheet);
        self
    }
}

impl<'a, M, R> From<Clickable<'a, M, R>> for Element<'a, M, R>
where
    M: 'a + Clone,
    R: 'a + Renderer,
{
    fn from(c: Clickable<'a, M, R>) -> Element<'a, M, R> {
        let b = Button::new(c.content)
            .height(c.height)
            .width(c.width)
            .style(ClickableStyle);
        match c.msg {
            Some(m) => b.on_press(m),
            None => b,
        }
        .into()
    }
}

/// A `StyleSheet` to make buttons invisible
struct ClickableStyle;
impl StyleSheet for ClickableStyle {
    fn active(&self) -> Style {
        Style::default()
    }

    fn hovered(&self) -> Style {
        self.active()
    }

    fn pressed(&self) -> Style {
        self.active()
    }

    fn disabled(&self) -> Style {
        self.active()
    }
}
