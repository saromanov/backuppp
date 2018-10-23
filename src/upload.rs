use s3::bucket::Bucket;
use s3::credentials::Credentials;

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