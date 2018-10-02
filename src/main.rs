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
        g_zip:"true".to_string(),
        backup_storage:"mongo".to_string(),
    };

    match conf.backup_storage {
        "mongo" => {
            
        }
    }
   match mongo::mongo_dump(conf) {
       Ok(dr) => {
           println!("Created archive file: {:}", dr.name)
       }
       Err(why) => {
           println!("{:}?", why)
       }
   }
}
