use std::fs;

pub struct Santa {
    position: (i32, i32),
    pub delivered_to: Vec<(i32, i32)>,
}

impl Santa {
    pub fn new() -> Santa {
        Santa {
            position: (0, 0),
            delivered_to: vec![(0, 0)] // Starting house gets a present before we move on
        }
    }

    fn add_if_new(&mut self, coords: (i32, i32)) {
        if !self.delivered_to.contains(&coords) {
            self.delivered_to.push(coords);
        }
    }

    pub fn parse_instruction(&mut self, instruction: char) {
        let (mut x, mut y) = self.position;
        match instruction {
            '<' =>  x -= 1,
            '>' => x += 1,
            '^' => y -= 1,
            'v' => y += 1,
            _ => panic!("Received a character other than [<^>v]!"),
        }
        self.add_if_new((x, y));
        self.position = (x, y);
    }

    pub fn parse_instructions(&mut self, instructions: Vec<char>) {
        for c in instructions { self.parse_instruction(c); }
    }

    pub fn houses_found(&self) -> usize { self.delivered_to.len() }

    pub fn unique_against(&mut self, other: &Santa) -> Vec<(i32, i32)> {
        let mut unique: Vec<(i32, i32)> = self.delivered_to.clone();
        for house in &other.delivered_to {
            if !unique.contains(&house) {
                unique.push(*house);
            }
        }
        unique.dedup();
        unique
    }
}