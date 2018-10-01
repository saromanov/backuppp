mod storage;
mod mysql;
mod mongo;
mod config;
mod dump;

use config::Config;

fn main() {
    let conf = Config{
        database:"".to_string(),
        host:"localhost".to_string(),
        username:"".to_string(),
        gZip:"true".to_string(),
    };
   match mongo::mongo_dump(conf) {
       Ok(dr) => {
           println!("{:}?", dr.name)
       }
       Err(why) => {
           println!("{:}?", why)
       }
   }
}
