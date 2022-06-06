use {
    iced_native::Renderer,
    iced_pure::{
        widget::button::{Button, Style, StyleSheet},
        Element,
    },
};

/// An invisible button around an `Element`
pub fn Clickable<'a, M, E, R>(content: E) -> Button<'a, M, R>
where
    M: 'a + Clone,
    E: Into<Element<'a, M, R>>,
    R: Renderer,
{
    Button::new(content).style(ClickableStyle)
}

/// A `StyleSheet` to make buttons invisible
pub struct ClickableStyle;
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