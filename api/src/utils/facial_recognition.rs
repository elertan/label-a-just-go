use std::process::{Command, Stdio};

#[derive(Serialize, Deserialize)]
pub struct ExtractFaceResult {
    pub start_x: u32,
    pub end_x: u32,
    pub start_y: u32,
    pub end_y: u32,
}

pub fn extract_face<'a>(filepath: &'a str) -> Result<ExtractFaceResult, &'static str> {
//    python extract_face.py --detector face_detection_model --embedding-model openface_nn4.small2.v1.t7 --image test.temp
    let output = Command::new("python3")
//        .current_dir(std::fs::canonicalize("./python").unwrap())
        .stdout(Stdio::piped())
        .args(&[
            "extract_face.py",
            "--detector",
            "face_detection_model",
            "--embedding-model",
            "openface_nn4.small2.v1.t7",
            "--image",
            filepath
        ])
        .output()
        .expect("failed to execute process");

    if !output.status.success() {
        return Err("Face extraction script failed");
    }
    let output_string_raw = String::from_utf8_lossy(&output.stdout).to_string();
    let output_string = output_string_raw.trim();
    let output_data: ExtractFaceResult = serde_json::from_str(output_string).unwrap();

    Ok(output_data)
}