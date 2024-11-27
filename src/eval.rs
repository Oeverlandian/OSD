use std::collections::HashMap;

use crate::lang::*;

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