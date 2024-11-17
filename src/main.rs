use anyhow::{bail, Result};
use std::env;
use std::fs;
use std::path::PathBuf;

mod error;
mod scanner;
use scanner::Scanner;

fn main() -> Result<()> {
    match env::args().nth(1) {
        Some(p) => {
            let path = PathBuf::from(p);
            if !path.exists() {
                bail!(format!("given path `{:?}` does not exists", path));
            }

            let scanner = Scanner::new(fs::read(path).unwrap());

            for token in scanner {
                if let Err(e) = token {
                    bail!(format!("{}", e));
                }

                let token = token.unwrap();
                println!("{}", token);
            }
            Ok(())
        }
        None => Ok(()),
    }
}
