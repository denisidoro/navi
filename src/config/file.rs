use crate::terminal::style::Color;

pub struct Comment {
    pub color: Color,
    pub width: usize,
}
pub struct Style {
    pub comment: Comment,
}
pub struct Yaml {
    pub style: Style,
}

impl Yaml {
    pub fn new() -> Self {
        Self {
            style: Style {
                comment: Comment {
                    color: Color::Cyan,
                    width: 13,
                },
            },
        }
    }
}
