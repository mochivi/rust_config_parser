// use std::{convert::Into, io::Read};
use std::collections::HashMap;
use std::io::{self, BufRead};
use std::panic::UnwindSafe;
use std::str::FromStr;
use std::path::Path;
use std::fs::File;
use anyhow::Error;

#[derive(Debug)]
pub struct ConfigParser {
    args: Option<HashMap<String, ArgType>>
}

#[derive(Debug)]
enum LineKind {
    TYPE,
    ARGUMENT,
    COMMENT,
    BLANK
}


#[derive(Debug, Clone)]
pub enum ArgType {
    STRING(Option<String>),
    INTEGER(Option<i64>),
    FLOAT(Option<f64>),
    BOOL(Option<bool>),
    VECTOR(Option<Vec<i64>>),
}

// Conversion implementations
impl ArgType {
    pub fn inner<I: From<ArgType>>(self) -> I{
        self.into()
    }
}
impl From<ArgType> for String {
    fn from(arg_type: ArgType) -> String {
        if let ArgType::STRING(content) = arg_type {
            return content.unwrap();
        } else {
            panic!();
        }
    }
}
impl From<ArgType> for i64 {
    fn from(arg_type: ArgType) -> i64 {
        if let ArgType::INTEGER(content) = arg_type {
            return content.unwrap();
        } else {
            panic!();
        }
    }
}
impl From<ArgType> for f64 {
    fn from(arg_type: ArgType) -> f64 {
        if let ArgType::FLOAT(content) = arg_type {
            return content.unwrap();
        } else {
            panic!();
        }
    }
}
impl From<ArgType> for bool {
    fn from(arg_type: ArgType) -> bool {
        if let ArgType::BOOL(content) = arg_type {
            return content.unwrap();
        } else {
            panic!();
        }
        
    }
}

impl ConfigParser {
    pub fn new() -> Self {
        Self {args: None}
    }
    
    pub fn parse(mut self, filepath: &str) -> Self {    
        // Initialize the args hashmap
        self.args = Some(HashMap::new());        

        if let Ok(lines) = self.read_lines(filepath) {
            let mut next_argument: Option<ArgType> = None;
            
            for (line_count, line) in lines.flatten().enumerate() {
                println!("{}: {}", line_count, line);
                // Match on all kinds of possible lines and take action
                match self.interpret_line(&line, line_count) {
                    Ok(LineKind::TYPE) => {
                        next_argument = Some(self.parse_type(&line, line_count).unwrap());
                    },
                    Ok(LineKind::ARGUMENT) => {
                        if let Some(ref arg_type) = next_argument {
                            self.parse_argument_line(arg_type, &line).expect(format!("Could not parse contents on line: {}: {}", line_count, line).as_str());
                        }
                    },
                    Ok(LineKind::COMMENT) => {
                        continue;
                    },
                    Ok(LineKind::BLANK) => {
                        continue;
                    },
                    Err(e) => {panic!("Error while parsing config file: {e}")}
                }
            }
        
        }

        self
    }

    pub fn get_arg<T: From<ArgType>>(&self, arg_name: &str) -> T {
        let args_hashmap = self.args.as_ref().unwrap();
        let value = args_hashmap.get(arg_name).unwrap().clone();
        let inner_value = value.inner::<T>();
        return inner_value
    }

    fn parse_argument_line(&mut self, arg_type: &ArgType, line: &String) -> Result<(), Error>{
        let (arg_name, arg_value) = self.get_arg_parts(&line);

        match arg_type {
            ArgType::STRING(_) => {
                let result = self.parse_argument::<String>(arg_value)?;
                let args = self.args.as_mut().unwrap();
                args.insert(
                    arg_name,
                    ArgType::STRING(Some(result))
                );
            },
            ArgType::INTEGER(_) => {
                let result = self.parse_argument::<i64>(arg_value)?;
                let args = self.args.as_mut().unwrap();
                args.insert(
                    arg_name,
                    ArgType::INTEGER(Some(result))
                );
            },
            ArgType::FLOAT(_) => {
                let result = self.parse_argument::<f64>(arg_value)?;
                let args = self.args.as_mut().unwrap();
                args.insert(
                    arg_name,
                    ArgType::FLOAT(Some(result))
                );
            },
            ArgType::BOOL(_) => {
                let result = self.parse_argument::<bool>(arg_value)?;
                let args = self.args.as_mut().unwrap();
                args.insert(
                    arg_name,
                    ArgType::BOOL(Some(result))
                );
            },
            ArgType::VECTOR(_) => {}//self.parse_complex_argument(arg_value);},
        }

        Ok(())
    }

    fn get_arg_parts(&self, line: &String) -> (String, String){
        let line_vector: Vec<&str> = line.split(":=").collect();
        let arg_name = line_vector.get(0).unwrap().trim().to_string();
        let arg_value = line_vector.get(1).unwrap().trim().to_string();
        (arg_name, arg_value)
    }

    fn parse_argument<F: FromStr>(&self, argument: String) -> Result<F, F::Err> {
        let result = argument.parse::<F>()?;
        Ok(result)
    }

    // fn parse_complex_argument<T>(&self, argument: String) -> Result<T, ()>{
    //     let result: T;
    // }

    fn parse_type(&self, line: &String, line_count: usize) -> Result<ArgType, Error> {
        match line.to_lowercase().trim() {
            "#int" => return Ok(ArgType::INTEGER(None)),
            "#str" => return Ok(ArgType::STRING(None)),
            "#string" => return Ok(ArgType::STRING(None)),
            "#bool" => return Ok(ArgType::BOOL(None)),
            "#float" => return Ok(ArgType::FLOAT(None)),
            "#vec" => return Ok(ArgType::VECTOR(None)),
            _ => Err(Error::msg(format!("Could not parse contents on line {}: {}", line_count + 1, line)))
        }
    }

    fn interpret_line(&self, line: &String, line_count: usize) -> Result<LineKind, Error> {
        if line.starts_with("#") {return Ok(LineKind::TYPE);}
        else if line.starts_with("//") {return Ok(LineKind::COMMENT);}
        else if line.trim() == "" {return Ok(LineKind::BLANK);}
        else if line.contains(":=") {return Ok(LineKind::ARGUMENT);}
        return Err(Error::msg(format!("Could not parse contents on line {}: {}", line_count + 1, line)));   
    }

    fn read_lines<P>(&mut self, filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path> {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }
}
