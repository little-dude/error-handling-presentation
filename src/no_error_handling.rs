use std::{fs::File, io::Read};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Operator {
    #[serde(rename = "+")]
    Add,
    #[serde(rename = "-")]
    Sub,
    #[serde(rename = "*")]
    Mul,
    #[serde(rename = "/")]
    Div,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Operands(f32, f32);

#[derive(Serialize, Deserialize, Debug)]
pub struct Operation {
    operands: Operands,
    operator: Operator,
}

impl Operation {
    fn execute(&self) -> f32 {
        let Operands(a, b) = self.operands;

        match self.operator {
            Operator::Add => add(a, b),
            Operator::Sub => sub(a, b),
            Operator::Div => div(a, b),
            Operator::Mul => mul(a, b),
        }
    }
}

fn div(a: f32, b: f32) -> f32 {
    if b == 0.0 {
        panic!("division by 0");
    }
    a / b
}

fn sub(a: f32, b: f32) -> f32 {
    a - b
}

fn add(a: f32, b: f32) -> f32 {
    a + b
}

fn mul(a: f32, b: f32) -> f32 {
    a * b
}

fn read_file(path: &str) -> Vec<u8> {
    let mut file = File::open(path).unwrap();
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).unwrap();
    contents
}

fn parse_file(path: &str) -> Operation {
    let data = read_file(path);
    serde_json::from_slice(data.as_slice()).unwrap()
}

pub fn execute_file(path: &str) {
    let operation = parse_file(path);
    println!("result for {}: {}", path, operation.execute());
}
