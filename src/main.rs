mod storage;
mod mysql;
mod mongo;
mod config;
mod dump;
mod upload;

use config::Config;

fn backup_mongo(conf:Config){
    match mongo::mongo_dump(conf) {
       Ok(dr) => {
           println!("Created archive file: {:}", dr.name)

       }
       Err(why) => {
           println!("{:}?", why)
       }
   }
}

fn backup_mysql(conf:Config) {
    match mysql::mysql_dump(conf) {
       Ok(dr) => {
           println!("Created archive file: {:}", dr.name)
       }
       Err(why) => {
           println!("{:}?", why)
       }
    }
}
fn main() {
    let conf = Config{
        database:"".to_string(),
        host:"localhost".to_string(),
        username:"".to_string(),
        g_zip:"true".to_string(),
        backup_storage:"mongo".to_string(),
        backup_upload:"gce".to_string(),
        ftp_login:"".to_string(),
        ftp_password:"".to_string(),
        ftp_address:"127.0.0.1:21".to_string(),
    };

    match &conf.backup_storage.as_ref() {
        &"mongo" => {
            backup_mongo(conf);
        }
        &"mysql" => {
            backup_mysql(conf);
        }
        _ => {
            println!("unable to find backup storage")
        }
    }
}
