use crate::config::DotConfig;
use std::{fs, fmt};
use serde_json::from_reader;
use std::fmt::{Display, Formatter};

pub fn read_config(path: &str) -> Result<DotConfig, self::MyError> {
    let file = fs::File::open(path)?;
    // .expect(&format!("Cannot Open File:\"{}\"\n", path));
    let res = serde_json::from_reader(file);
    if let Ok(res) = res {
        Ok(res)
    } else {
        Err(res.err().unwrap().into())
    }
}

pub struct MyError {
    pub message: String,
}

impl<E: std::error::Error> From<E> for MyError {
    fn from(e: E) -> Self {
        MyError {
            message: e.to_string()
        }
    }
}

#[cfg(test)]
mod test {
    use crate::operation::read_config;

    #[test]
    fn read_file() {
        let config = read_config("S:/a.txt");
        if let Ok(config) = config {
            println!("{}", serde_json::to_string(&config).unwrap());
        } else {
            println!("{}",config.err().unwrap().message);
        }
    }
}