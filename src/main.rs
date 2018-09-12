mod storage;
mod mysql;
mod mongo;
mod config;
mod dump;

use config::Config;

fn main() {
    println!("Hello, world!");
    let conf = Config{
        database:"AAA".to_string(),
        host:"AAA".to_string(),
        username:"AAA".to_string(),
        gZip:"AAA".to_string(),
    };
   match mongo::mongo_dump(conf) {
       Ok(dr) => {

       }
       Err(why) => {
           println!("{:}?", why)
       }
   }
}
