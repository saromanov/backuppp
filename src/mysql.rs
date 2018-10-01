use std::process::Command;
use std::process;
use std::{error, fmt};
use std::io;
use config::Config;

const DUMP_COMMAND:&str = "mysqldump";
const DEFAULT_MYSQL_PORT:u32 = 3306;

#[derive(Debug, Clone)]
pub enum MySQLError {
    ConsoleError,
}

impl fmt::Display for MySQLError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MySQLError::ConsoleError => write!(f, "unable to add new block. Genesis block is not defined"),
        }
    }
}

impl error::Error for MySQLError {
    fn description(&self) -> &str {
         match *self {
            MySQLError::ConsoleError => "unable to add new block. Genesis block is not defined",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

// mysql_dump provides dumping of mysql db
pub fn mysql_dump(_conf:Config) -> Result<process::Output,io::Error> {
    Command::new(DUMP_COMMAND)
            .arg("-c")
            .arg("echo hello")
            .output()
}