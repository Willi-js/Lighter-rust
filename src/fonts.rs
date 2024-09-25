use std::borrow::Cow;

use iced::Font;

pub static DEFAULT_FONT: Font = Font::with_name("Iosevka Term");

pub fn load_font() ->  Vec<Cow<'static, [u8]>> {
    vec![
        include_bytes!("../fonts/iosevka-term-regular.ttf")
            .as_slice()
            .into()
    ]
}