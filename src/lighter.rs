use iced::{widget::{ container, text_editor}, Element, Theme};

#[derive(Default)]
pub struct Lighter {
    content: text_editor::Content
}

#[derive(Debug, Clone)]
pub enum Message {
    Edit(text_editor::Action)
}

impl Lighter {
    pub fn view(&self) -> Element<'_, Message> {
        container(
            text_editor(&self.content)
                .on_action(Message::Edit)
        ).into()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Edit(action) => {
                self.content.perform(action);
            }
        }
    }

    pub fn theme(&self) -> Theme{
        Theme::CatppuccinFrappe
    }
}
