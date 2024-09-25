use fonts::DEFAULT_FONT;
use iced::widget::{button, column, text, Column};

mod fonts;

#[derive(Default)]
struct Counter {
    value: i32,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Increment,
    Decrement,
}

impl Counter {
    pub fn view(&self) -> Column<Message> {
        // We use a column: a simple vertical layout
        column![
            // The increment button. We tell it to produce an
            // `Increment` message when pressed
            button("+").on_press(Message::Increment),

            // We show the value of the counter here
            text(self.value),

            // The decrement button. We tell it to produce a
            // `Decrement` message when pressed
            button("-").on_press(Message::Decrement),
        ]
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.value += 1;
            }
            Message::Decrement => {
                self.value -= 1;
            }
        }
    }
}



fn main() -> iced::Result {

    iced::application("Lighter", Counter::update, Counter::view).settings(iced::Settings {
        id: None,
        fonts: fonts::load_font(),
        default_font: DEFAULT_FONT,
        default_text_size: iced::Pixels(30.0),
        antialiasing: false,
    }).run()

}