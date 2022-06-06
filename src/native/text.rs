use iced_native::{text::Renderer, widget::Text};

macro_rules! generate_text_fn {
    ($($fn:ident = $size:literal),+) => {
        $(
        #[doc = concat!("Create a header of size ", stringify!($size))]
        pub fn $fn<T, R>(text: T) -> Text<R>
        where
            T: Into<String>,
            R: Renderer,
        {
            Text::new(text).size($size)
        }
        )*
    };
}

generate_text_fn![H0 = 40, H1 = 32, H2 = 26, H3 = 22, H4 = 20];
