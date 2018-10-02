use std::process::Command;
use std::{error, fmt};
use config::Config;
use dump::DumpResult;
use storage::Storage;

const MONGO_DUMP_COMMAND: &str = "mongodump";

#[derive(Debug)]
pub enum MongoError {
    HostNotDefined,
}
pub type MongoDumpResult = Result<DumpResult,MongoError>;

impl fmt::Display for MongoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MongoError::HostNotDefined => write!(f, "host is not defined"),
        }
    }
}

impl error::Error for MongoError {
    fn description(&self) -> &str {
         match *self {
            MongoError::HostNotDefined => "host is not defined",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

pub struct MongoDump {
    host:String,
}

impl Storage for MongoDump {
    fn build(&mut self, conf:Config){
        self.host = conf.host;
    }
}

// mysql_dump provides dumping of mysql db
pub fn mongo_dump(conf:Config) -> MongoDumpResult {
    if conf.host == "" {
        return Err(MongoError::HostNotDefined)
    }

    let mut output = Command::new(MONGO_DUMP_COMMAND);
    if conf.g_zip != "" {
        output.arg("--gzip");
    }
    output.arg("--archive=.")
    .output()
    .expect("unable to execute command");
    let dr = DumpResult{name:"archive.gz".to_string()};
    Ok(dr)
}