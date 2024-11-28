use std::collections::HashMap;
use crate::lang::*;

// Evaluator

fn evaluate_circuit(program: &Program, input_values: Vec<bool>) -> HashMap<String, bool> {
    let mut values = HashMap::new();

    for (i, input) in program.inputs.iter().enumerate() {
        values.insert(input.clone(), input_values[i]);
    }

    for component in &program.components {
        let input_values: Vec<bool> = component.inputs.iter()
            .map(|input| *values.get(input).expect("Input not found"))
            .collect();

        let result = match component.gate_type {
            GateType::And => eval_and(input_values),
            GateType::Or => eval_or(input_values),
            GateType::Not => eval_not(input_values[0]),
            GateType::Nand => eval_nand(input_values),
            GateType::Nor => eval_nor(input_values),
            GateType::Xor => eval_xor(input_values),
            GateType::Xnor => eval_xnor(input_values),
        };

        values.insert(component.output.clone(), result);
    }

    values
}

fn eval_and(inputs: Vec<bool>) -> bool {
    inputs.iter().all(|&x| x)
}

fn eval_or(inputs: Vec<bool>) -> bool {
    inputs.iter().any(|&x| x)
}

fn eval_not(input: bool) -> bool {
    !input
}

fn eval_nand(inputs: Vec<bool>) -> bool {
    !eval_and(inputs)
}

fn eval_nor(inputs: Vec<bool>) -> bool {
    !eval_or(inputs)
}

fn eval_xor(inputs: Vec<bool>) -> bool {
    inputs.iter().fold(false, |acc, &x| acc ^ x)
}

fn eval_xnor(inputs: Vec<bool>) -> bool {
    !eval_xor(inputs)
}

// Printer

pub fn print_truth_table(program: &Program) {
    let input_combinations = generate_input_combinations(program.inputs.len());

    let header: Vec<String> = program.inputs.iter().map(|i| i.clone()).collect();
    let header_str = header.join(", ");
    println!("{}, {}", header_str, program.outputs.join(", "));

    for combination in input_combinations {
        let results = evaluate_circuit(program, combination);

        let input_str: Vec<String> = program.inputs.iter()
            .map(|i| results[i].to_string())
            .collect();
        let input_str = input_str.join(", ");

        let output_str: Vec<String> = program.outputs.iter()
            .map(|o| results[o].to_string())
            .collect();
        let output_str = output_str.join(", ");

        println!("{}, {}", input_str, output_str);
    }
}

fn generate_input_combinations(n: usize) -> Vec<Vec<bool>> {
    let mut combinations = Vec::new();
    let num_combinations = 2_usize.pow(n as u32);

    for i in 0..num_combinations {
        let mut combination = Vec::new();
        for j in 0..n {
            combination.push((i >> (n - 1 - j)) & 1 == 1);
        }
        combinations.push(combination);
    }
    combinations
}
