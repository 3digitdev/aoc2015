use std::fs;
use crate::instructions::Instructions;

pub struct Floor {
    value: i32
}

impl Floor {
    pub fn new() -> Floor { Floor { value: 0 }}

    pub fn print(&self) { println!("Floor {}", self.value); }

    fn up(&mut self) { self.value += 1; }

    fn down(&mut self) { self.value -= 1; }

    fn parse_char(&mut self, c: char) {
        match c {
            '(' => self.up(),
            ')' => self.down(),
            a => panic!("Expected a '(' or ')' but got '{}'", a),
        }
    }

    pub fn parse_instructions(&mut self, instructions: &mut Instructions) {
        while let Some(c) = instructions.next() {
            self.parse_char(c);
            // if self.value == -1 {
            //     println!("Instruction [{}] put us on Floor -1", instructions.position)
            // }
        }
    }
}