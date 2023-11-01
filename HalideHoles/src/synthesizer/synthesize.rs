use crate::parser::parser::Program;
use crate::parser::parser::ProgramElement;

fn explode_program(program: Program) {
    let mut contents = program.program.map(|prog_element| vec![prog_element]);

    for content in contents {
        // todo --- explode the program
    }
}

// Write a cross-product fnction in rust from Vec<Vec<ProgramElement>> -> Vec<Vec<ProgramElement>>
fn cross_product(input: Vec<Vec<ProgramElement>>) -> Vec<Vec<ProgramElement>> {
    let mut result: Vec<Vec<ProgramElement>> = Vec::new();
    let mut stack: Vec<Vec<ProgramElement>> = Vec::new();
    stack.push(Vec::new());

    for elements in input {
        let mut new_stack: Vec<Vec<ProgramElement>> = Vec::new();
        for element in elements {
            for mut combination in stack {
                let mut new_combination = combination.clone();
                new_combination.push(element);
                new_stack.push(new_combination);
            }
        }
        stack = new_stack;
    }

    result.append(&mut stack);
    result
}

pub synthesize(program: Program): Vec<Program> {
    let candidates = explode_program(program);

    let program_variants = cross_product(candidates);
    program_variants.map(|prog| Program { program: prog })
}
