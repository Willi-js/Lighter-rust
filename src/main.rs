mod fonts;
mod lighter;

use lighter::Lighter;
use fonts::DEFAULT_FONT;

fn main() -> iced::Result {

    iced::application("Lighter", Lighter::update, Lighter::view).settings(iced::Settings {
        id: None,
        fonts: fonts::load_font(),
        default_font: DEFAULT_FONT,
        default_text_size: iced::Pixels(10.0),
        antialiasing: false,
    }).theme(Lighter::theme).run()

}