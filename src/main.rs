#![warn(clippy::all, clippy::pedantic)]

mod document;
mod editor;
mod row;
mod terminal;
pub use document::Document;
use editor::Editor;
pub use editor::Position;
pub use row::Row;
pub use terminal::Terminal;

// fn to_ctrl_byte(c: char) -> u8 {
//     let byte = c as u8;
//     byte & 0b0001_1111
// }

fn main() {
    Editor::default().run();
}
