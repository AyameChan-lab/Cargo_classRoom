use anyhow::Result;
use base64::{Engine as _, engine::general_purpose};

#[derive(Debug, Clone)]
pub struct Base64Image(String); // tuple struct

impl Base64Image {
    pub fn new(data: String) -> Result<Self> {
        if data.is_empty() {
            return Err(anyhow::anyhow!("Base64 image data cannot be empty !!"));
        }
        let bytes = match general_purpose::STANDARD.decode(&data) {
            Ok(bs) => bs,
            Err(_) => return Err(anyhow::anyhow!("Unsupported or Invalid base64 image type.")),
        };  

        let file_type = match infer::get(&bytes){
            Some(t) if t.mime_type() == "image/png" || t.mime_type() == "image/jpeg" => {
                t.mime_type()
            }
            _ => return Err(anyhow::anyhow!("Unsupported image type.")),
        };

        Ok(Self(format!("data:{};base64,{}", file_type, data)))
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}
