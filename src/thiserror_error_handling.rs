use std::{
    fs::File,
    io::{self, Read},
};

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
    fn execute(&self) -> Result<f32, RunTimeError> {
        let Operands(a, b) = self.operands;

        Ok(match self.operator {
            Operator::Add => add(a, b),
            Operator::Sub => sub(a, b),
            Operator::Div => div(a, b)?,
            Operator::Mul => mul(a, b),
        })
    }
}

fn div(a: f32, b: f32) -> Result<f32, RunTimeError> {
    if b == 0.0 {
        return Err(RunTimeError::Execution("division by 0".to_string()));
    }
    Ok(a / b)
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

fn read_file(path: &str) -> Result<Vec<u8>, RunTimeError> {
    let mut file = File::open(path)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;
    Ok(contents)
}

fn parse_file(path: &str) -> Result<Operation, RunTimeError> {
    let data = read_file(path)?;
    let operation = serde_json::from_slice(data.as_slice())?;
    Ok(operation)
}

pub fn execute_file(path: &str) -> Result<(), RunTimeError> {
    let operation = parse_file(path)?;
    let result = operation.execute()?;
    println!("result for {}: {}", path, result);
    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum RunTimeError {
    #[error("failed to read file: {0}")]
    ReadFile(#[from] io::Error),
    #[error("failed to parse JSON content: {0}")]
    ParseContent(#[from] serde_json::Error),
    #[error("failed to execute operation: {0:?}")]
    Execution(String),
}
