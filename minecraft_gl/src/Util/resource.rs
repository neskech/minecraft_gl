use image::{self, DynamicImage};
use image::io::Reader as ImageReader;

pub fn GetImageFromPath(path: &str) -> Result<DynamicImage, String> {

    let img = ImageReader::open(path);
    if let Err(_) = img {
        return Err(format!("Error! Could not read texture atlas from path of: {}", path));
    }

    let decoded = img.unwrap().decode();
    if let Err(_) = decoded {
        return Err(format!("Error! Could not decode image from path of: {}", path));
    }

    Ok(decoded.unwrap())
}