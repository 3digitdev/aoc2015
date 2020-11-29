use std::str::Chars;
use std::fs;

#[derive(Debug)]
pub struct Instructions {
    pub position: i32,
    text: Vec<char>,
}

impl Instructions {
    pub fn new() -> Instructions{ Instructions { position: 0, text: vec![] } }

    pub fn parse_file(&mut self, path: &str) {
        self.text = fs::read_to_string(&path).unwrap().chars().collect();
    }
}

impl Iterator for Instructions {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let idx = self.position as usize;
        self.position += 1;
        if self.text.len() <= idx {
            None
        } else {
            Some(self.text[idx])
        }
    }
}