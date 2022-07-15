use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::instruction::Instruction;

pub type Program = Vec<Instruction>;

pub fn compile_asm(filename: &str) -> Program {
    match File::open(&filename) {
        Err(why) => panic!("couldn't open {}: {}", filename, why),
        Ok(file) => {
            let mut prog = Program::new();
            for (_, line) in BufReader::new(file).lines().enumerate() {
                match line {
                    Err(why) => panic!("couldn't read line: {}", why),
                    Ok(line) => {
                        let line = match line.find(';') {
                            Some(a) => String::from(&line[..a]),
                            None => line
                        }.trim().replace("\t", " ");
                        if line.len() > 0 {
                            prog.push(Instruction::parse(line.as_str()));
                        }
                    }
                };
            }
            prog
        }
    }
}
