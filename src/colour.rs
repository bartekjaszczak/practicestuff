use crossterm::style::{Color, Stylize};

pub fn format_text(text: &str, apply: bool, colour: Color) -> String {
    if apply {
        text.with(colour).to_string()
    } else {
        text.to_string()
    }
}
