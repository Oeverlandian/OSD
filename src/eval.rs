use std::collections::HashMap;
use crate::lang::*;

// Evaluator

fn evaluate_circuit(program: &Program, subcircuit_name: Option<&str>, input_values: HashMap<String, bool>) -> HashMap<String, bool> {
    let mut values = input_values.clone();
   
    let components = if let Some(name) = subcircuit_name {
        &program.subcircuits[name].components
    } else {
        &program.components
    };

    for component in components {
        let input_values: Vec<bool> = component.inputs.iter()
            .map(|input| *values.get(input).expect("Input not found"))
            .collect();

        let results = match &component.gate_type {
            GateType::And => vec![eval_and(input_values)],
            GateType::Or => vec![eval_or(input_values)],
            GateType::Not => vec![eval_not(input_values[0])],
            GateType::Nand => vec![eval_nand(input_values)],
            GateType::Nor => vec![eval_nor(input_values)],
            GateType::Xor => vec![eval_xor(input_values)],
            GateType::Xnor => vec![eval_xnor(input_values)],
            GateType::Subcircuit(subcircuit_name) => {
                let subcircuit = &program.subcircuits[subcircuit_name];
               
                let mut subcircuit_inputs = HashMap::new();
                for (i, input_name) in subcircuit.inputs.iter().enumerate() {
                    subcircuit_inputs.insert(
                        input_name.clone(),
                        input_values[i]
                    );
                }
               
                let subcircuit_results = evaluate_circuit(
                    program,
                    Some(subcircuit_name),
                    subcircuit_inputs
                );
               
                subcircuit.outputs.iter()
                    .map(|output_name| *subcircuit_results.get(output_name)
                        .expect("Subcircuit output not found"))
                    .collect()
            }
        };

        for (output_name, result) in component.outputs.iter().zip(results.iter()) {
            values.insert(output_name.clone(), *result);
        }
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

    let header: Vec<String> = program.inputs.iter().cloned().collect();
    let header_str = header.join(", ");
    println!("{}, {}", header_str, program.outputs.join(", "));

    for combination in input_combinations {
        let mut input_values = HashMap::new();
        for (i, input) in program.inputs.iter().enumerate() {
            input_values.insert(input.clone(), combination[i]);
        }

        let results = evaluate_circuit(program, None, input_values);

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
