mod floor;
mod instructions;
mod present;
mod delivery;

use floor::Floor;
use std::{fs, fmt};
use crate::instructions::Instructions;
use crate::present::Present;
use crate::delivery::Santa;
use md5;
use fancy_regex::Regex;

fn main() {
    // day_1();
    // day_2();
    // day_3();
    // day_4();
    day_5();
}

fn day_5() {
    let double_re = Regex::new(r"(\w)\1").unwrap();
    let banned_re = Regex::new(r"(ab|cd|pq|xy)").unwrap();
    let lines = fs::read_to_string("inputs/5-1.txt").unwrap();
    let mut nice_strs: Vec<&str> = vec![];
    for line in lines.split('\n') {
        let mut vc = 0;
        if double_re.is_match(&line).unwrap() && !banned_re.is_match(&line).unwrap() {
            for vowel in "aeiou".split("").filter(|x| x != &"") {
                if line.contains(&vowel) {
                    vc += line.matches(vowel).count();
                }
            }
            if vc >= 3 {
                nice_strs.push(&line);
            }
        }
    }
    println!("There are a total of {} nice strings", nice_strs.len());
    // Part 2
    let split_re = Regex::new(r"(\w)\w\1").unwrap();
    let pair_re = Regex::new(r"(\w\w)\w*\1").unwrap();
    nice_strs.clear();
    for line in lines.split('\n') {
        if split_re.is_match(&line).unwrap(){
            if pair_re.is_match(&line).unwrap() {
                nice_strs.push(line);
            }
        }
    }
    println!("There are a total of {} ACTUAL nice strings", nice_strs.len());
}

fn day_4() {
    println!("Digest with 5 leading zeroes:");
    find_digest(5);
    println!("Digest with 6 leading zeroes:");
    find_digest(6);
}

fn find_digest(zeroes: usize) {
    let mut prefix = String::new();
    for i in 0..zeroes {
        prefix.push_str("0");
    }
    let is_correct = |hash: &md5::Digest| -> bool { &format!("{:x}", hash).as_str()[..zeroes] == &prefix };
    let mut current = 0;
    loop {
        current += 1;
        let digest = md5::compute(format!("ckczppom{}", current));
        if is_correct(&digest) {
            println!("The answer is {}, digest: {:x}", current, digest);
            break;
        }
    }
}

fn day_3() {
    // Part 1
    let instructions: Vec<char> = fs::read_to_string("inputs/3-1.txt").unwrap().chars().collect();
    let mut og_santa = Santa::new();
    og_santa.parse_instructions(instructions);
    println!("Santa delivered presents to {} houses the 1st year", og_santa.houses_found());
    // Part 2
    let mut real_santa = Santa::new();
    let mut robo_santa = Santa::new();
    let instructions: Vec<char> = fs::read_to_string("inputs/3-1.txt").unwrap().chars().collect();
    let mut santa = true;
    for instruction in instructions {
        if santa {
            real_santa.parse_instruction(instruction);
        } else {
            robo_santa.parse_instruction(instruction);
        }
        santa = !santa;
    }
    let house_list = real_santa.unique_against(&robo_santa);
    println!("Real Santa and Robo-Santa delivered presents to {} houses the 2nd year", house_list.len());
}

fn day_2() {
    // Part 1
    let mut total_paper = 0i32;
    // Part 2
    let mut total_ribbon = 0i32;
    for line in fs::read_to_string("inputs/2-1.txt").unwrap().split('\n') {
        let parts: Vec<&str> = line.split('x').collect();
        if parts.len() == 3 {
            let pres = Present::new((
                parts[0].parse().unwrap(),
                parts[1].parse().unwrap(),
                parts[2].parse().unwrap()
            ));
            // Part 1
            total_paper += pres.surface_area();
            // Part 2
            total_ribbon += pres.ribbon();
        }
    }
    // Part 1
    println!("Surface area needed: {} sqft", total_paper);
    // Part 2
    println!("Ribbon length needed: {} ft", total_ribbon);
}

fn day_1() {
    let mut instructions = Instructions::new();
    instructions.parse_file("inputs/1-1.txt");
    let mut floor = Floor::new();
    floor.parse_instructions(&mut instructions);
}