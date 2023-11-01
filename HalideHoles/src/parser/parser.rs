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
        s
    }
}

fn handle_hole(sequence: Pair<Rule>) -> ProgramElement {
    match sequence.as_rule() {
        Rule::hole => {
            let mut inner = sequence.into_inner();
            let name = inner.next().unwrap().as_str();

            ProgramElement::Hole(name.to_string())
        },
        _ => panic!("Not a hole {}", sequence.as_str()),
    }
}

fn handle_synth_escapes(sequence: Pair<Rule>) -> Vec<ProgramElement> {
    match sequence.as_rule() {
        Rule::synthesis_escapes => {
            let mut inner = sequence.clone().into_inner();
            if inner.len() == 1 {
                vec![ProgramElement::ProgramChunk(sequence.as_str().to_string())]
            } else {
                let chrs = inner.next().unwrap().as_str();
                let hole = handle_hole(inner.next().unwrap());
                let mut rest = handle_synth_escapes(inner.next().unwrap());

                let mut this_vec = vec![
                    ProgramElement::ProgramChunk(chrs.to_string()),
                    hole
                ];
                this_vec.append(&mut rest);
                this_vec
            }
        },
        _ => panic!("Not a synth escape")
    }
}

fn handle_program(sequence: Pair<Rule>) -> Program {
    match sequence.as_rule() {
        Rule::file => {
            let mut inner = sequence.into_inner();
            if inner.len() <= 1 {
                Program { program: vec![] }
            } else {
                let synth_escapes = inner.next().unwrap();
                Program{ program: handle_synth_escapes(synth_escapes) }
            }
        },
        _ => panic!("Unexpected rule"),
    }
}

pub fn parse(opts: &Options, filename: &String) -> Program {
    let input = fs::read_to_string(filename).expect("Unable to read file");

    let mut sequence = SketchParser::parse(Rule::file, &input[..]).unwrap();
    let program = handle_program(sequence.next().unwrap());

    println!("Parsed is {}", program.to_string());
    program
}
