use std::process::Command;
const GCE_UPLOAD_COMMAND: &str = "gsutil";

pub fn upload_to_gce(bucket:String, name:String){
    let mut output = Command::new(GCE_UPLOAD_COMMAND);
    output.arg("cp").arg(format!("{}", name)).arg(format!("gs://{}/{}", bucket, name))
    .output()
    .expect("unable to execute command");
}