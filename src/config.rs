#[derive(Clone, Debug)]
pub struct Config {
   pub database:String,
   pub host:String,
   pub username:String,
   pub g_zip:String,
   pub backup_storage:String,
   pub backup_upload:String,
   pub ftp_login:String,
   pub ftp_password:String,
   pub ftp_address:String,
}