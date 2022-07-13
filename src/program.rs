use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::instruction;

pub type Program = Vec<instruction::Instruction>;

pub fn compile_asm(filename: &str) -> Program {
    match File::open(&filename) {
        Err(why) => panic!("couldn't open {}: {}", filename, why),
        Ok(file) => {
            let mut prog = Program::new();
            for (_, line) in BufReader::new(file).lines().enumerate() {
                match line {
                    Err(why) => panic!("couldn't read line: {}", why),
                    Ok(line) => {
                        let cleaned = match line.find(';') {
                            Some(a) => line.as_str()[..a].trim(),
                            None => line.as_str().trim()
                        };
                        if cleaned.chars().count() > 0 {
                            prog.push(instruction::parse_instruction(cleaned));
                        }
                    }
                };
            }
            prog
        }
    }
}
