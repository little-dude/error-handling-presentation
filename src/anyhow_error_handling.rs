use std::{fs::File, io::Read};

use anyhow::{Context, Result};
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
    fn execute(&self) -> Result<f32> {
        let Operands(a, b) = self.operands;

        Ok(match self.operator {
            Operator::Add => add(a, b),
            Operator::Sub => sub(a, b),
            Operator::Div => div(a, b).context("division failed")?,
            Operator::Mul => mul(a, b),
        })
    }
}

fn div(a: f32, b: f32) -> Result<f32> {
    if b == 0.0 {
        anyhow::bail!("division by 0");
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

fn read_file(path: &str) -> Result<Vec<u8>> {
    let mut file = File::open(path).context(format!("could not open file {}", path))?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)
        .context(format!("could not read file {}", path))?;
    Ok(contents)
}

fn parse_file(path: &str) -> Result<Operation> {
    let data = read_file(path)?;
    serde_json::from_slice(data.as_slice()).context("invalid json content")
}

pub fn execute_file(path: &str) -> Result<()> {
    let operation = parse_file(path).context("could not parse operation from file")?;
    let result = operation.execute().context("operation execution failed")?;
    println!("result for {}: {}", path, result);
    Ok(())

}
