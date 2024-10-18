use std::io::LineWriter;
use std::io::{self, Write};

#[allow(dead_code)]
pub fn clear_term() {
    let s = format!("{}[2J", 27 as char);
}
