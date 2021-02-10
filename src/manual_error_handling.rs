use std::{
    error::Error,
    fmt,
    fs::File,
    io::{self, Read},
};

use serde::{Deserialize, Serialize};

// macro_rules! tryy {
//     ($e:expr) => {
//         match $e {
//             Ok(res) => res,
//             Err(e) => return Err(e.into()),
//         }
//     };
// }

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
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(io_err) => return Err(io_err.into()),
    };
    let mut contents = Vec::new();
    let _ = match file.read_to_end(&mut contents) {
        Ok(_) => (),
        Err(io_err) => return Err(io_err.into()),
    };
    Ok(contents)
}

fn parse_file(path: &str) -> Result<Operation, RunTimeError> {
    let data = match read_file(path) {
        Ok(data) => data,
        Err(runtime_err) => return Err(runtime_err.into()),
    };
    let operation = match serde_json::from_slice(data.as_slice()) {
        Ok(operation) => operation,
        Err(serde_json_err) => return Err(serde_json_err.into()),
    };
    Ok(operation)
}

pub fn execute_file(path: &str) -> Result<(), RunTimeError> {
    let operation = match parse_file(path) {
        Ok(operation) => operation,
        Err(runtime_err) => return Err(runtime_err.into()),
    };
    let result = match operation.execute() {
        Ok(result) => result,
        Err(runtime_err) => return Err(runtime_err.into()),
    };
    println!("result for {}: {}", path, result);
    Ok(())
}

#[derive(Debug)]
pub enum RunTimeError {
    ReadFile(io::Error),
    ParseContent(serde_json::Error),
    Execution(String),
}

impl fmt::Display for RunTimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "runtime error: ")?;
        match self {
            RunTimeError::ReadFile(io_err) => write!(f, "could not read file: {}", io_err),
            RunTimeError::ParseContent(serde_json_err) => {
                write!(f, "could parse content as json {}", serde_json_err)
            }
            RunTimeError::Execution(exec_err) => {
                write!(f, "operation execution failed: {}", exec_err)
            }
        }
    }
}

impl Error for RunTimeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        use RunTimeError::*;
        match self {
            ReadFile(e) => Some(e),
            ParseContent(e) => Some(e),
            Execution(_) => None,
        }
    }
}

impl From<serde_json::Error> for RunTimeError {
    fn from(e: serde_json::Error) -> Self {
        Self::ParseContent(e)
    }
}

impl From<io::Error> for RunTimeError {
    fn from(e: io::Error) -> Self {
        Self::ReadFile(e)
    }
}

impl From<String> for RunTimeError {
    fn from(e: String) -> Self {
        Self::Execution(e)
    }
}
