macro_rules! generate_text_fn {
    ($($fn:ident = $size:literal),+) => {
        $(
        #[doc = concat!("A header of size ", stringify!($size))]
        pub type $fn = Text<$size>;
        )*
    };
}

generate_text_fn![H0 = 40, H1 = 32, H2 = 26, H3 = 22, H4 = 20];

///Intermediate widget for `iced_pure::widget::Text` with size `SIZE`
///
///used mainly to define shorthands for text of specific sizes
///see [`H0`](../H0), [`H1`](../H1), [`H2`](../H2), [`H3`](../H3) and [`H4`](../H4)
pub struct Text<const SIZE: u16>;

use {
    iced::{advanced, widget},
    std::borrow::Cow,
};

#[allow(clippy::new_ret_no_self)]
impl<const SIZE: u16> Text<SIZE> {
    pub fn new<'a, Text, Theme, Renderer>(text: Text) -> widget::Text<'a, Theme, Renderer>
    where
        Text: Into<Cow<'a, str>>,
        Theme: widget::text::StyleSheet,
        Renderer: advanced::text::Renderer,
    {
        widget::Text::new(text).size(SIZE)
    }

    ///Fetch the `SIZE` for uses elsewhere
    pub const fn size() -> u16 {
        SIZE
    }
}
