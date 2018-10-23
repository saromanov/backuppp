use s3::bucket::Bucket;
use s3::credentials::Credentials;
use config::Config;

use std::process::Command;
const GCE_UPLOAD_COMMAND: &str = "gsutil";

pub fn upload_to_gce(bucket:String, name:String){
    let mut output = Command::new(GCE_UPLOAD_COMMAND);
    output.arg("cp").arg(format!("{}", name)).arg(format!("gs://{}/{}", bucket, name))
    .output()
    .expect("unable to execute command");
}

pub fn upload_to_aws(bucket:String, name:String){
    let credentials = Credentials::default();
    let region = REGION.parse().unwrap();
    let bucket = Bucket::new(bucket, region, credentials);
    let (_, code) = bucket.put("test_file", MESSAGE.as_bytes(), "text/plain").unwrap();
}

//
// upload_to_ftp provides upload
// of backup file to ftp
pub fn upload_to_ftp(conf:Config, bucket:String, name:String) {
    let mut ftp_stream = FtpStream::connect(conf.ftp_address).unwrap();
    let _ = ftp_stream.login(conf.ftp_login, conf.ftp_password).unwrap();
    println!("Current directory: {}", ftp_stream.pwd().unwrap());
    let _ = ftp_stream.cwd("backup").unwrap();
    let mut reader = Cursor::new("Hello from the Rust \"ftp\" crate!".as_bytes());
    let _ = ftp_stream.put("greeting.txt", &mut reader);
    println!("Successfully wrote greeting.txt");
    let _ = ftp_stream.quit();
}