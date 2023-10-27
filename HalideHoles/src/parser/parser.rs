use pest::Parser;
use pest::iterators::Pair;
use pest_derive::Parser;
use std::fs;
use crate::options::options::Options;

#[derive(Parser)]
#[grammar = "parser/grammar.pest"]

struct SketchParser;
#[derive(Clone)]
pub struct Hole {
    pub typ: String
}

#[derive(Clone)]
pub enum ProgramElement {
    ProgramChunk(String),
    Hole(String)
}

pub struct Program {
    program: Vec<ProgramElement>
}

impl ToString for Program
 {
    fn to_string(&self) -> String {
        let mut s = String::new();
        for element in self.program.iter() {
            match element {
                ProgramElement::ProgramChunk(chunk) => s.push_str(chunk),
                ProgramElement::Hole(hole) => s.push_str(&format!("<<{}>>", hole)),
            }
        }
    }
}

pub fn parse(opts: &Options, filename: &String) -> Program {
    let input = fs::read_to_string(filename).expect("Unable to read file");

    let mut sequence = SketchParser::parse(Rule::file, &input[..]).unwrap();

    println!("Parsed is {}", sequence.to_string());
}
