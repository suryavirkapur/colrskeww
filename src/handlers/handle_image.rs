use std::io::Cursor;

use image::GenericImageView;

#[poem::handler]
pub async fn upload(mut multipart: poem::web::Multipart) -> poem::Response {
    let mut deficiency = String::from("deutan");
    let mut file_bytes: Option<Vec<u8>> = None;

    while let Ok(Some(field)) = multipart.next_field().await {
        let field_name = match field.name() {
            Some(name) => name.to_string(),
            None => continue,
        };

        match field_name.as_str() {
            "deficiency" => {
                if let Ok(bytes) = field.bytes().await {
                    if let Ok(s) = String::from_utf8(bytes) {
                        deficiency = s.trim().to_lowercase();
                    }
                }
            }
            "upload" => {
                if let Ok(bytes) = field.bytes().await {
                    if !bytes.is_empty() {
                        file_bytes = Some(bytes);
                    }
                }
            }
            _ => {} // Ignore any other fields
        }
    }

    // Process the data extracted from the form
    if let Some(bytes) = file_bytes {
        let img = match image::load_from_memory(&bytes) {
            Ok(img) => img,
            Err(e) => {
                return poem::Response::builder()
                    .status(poem::http::StatusCode::BAD_REQUEST)
                    .body(format!("Failed to decode image: {}", e));
            }
        };

        let (width, height) = img.dimensions();
        let rgb_data = img.to_rgb8().into_raw();

        let simulated_rgb = match crate::image_proc::simulate::simulate_color_blindness(
            rgb_data,
            &deficiency,
        ) {
            Ok(data) => data,
            Err(e) => {
                return poem::Response::builder()
                    .status(poem::http::StatusCode::BAD_REQUEST)
                    .body(format!(
                        "Simulation error: {}. Supported types: 'deutan', 'deuteranopia', 'protan', 'protanopia'",
                        e
                    ));
            }
        };

        let simulated_img_buf =
            image::ImageBuffer::<image::Rgb<u8>, _>::from_raw(width, height, simulated_rgb)
                .expect("Failed to create image buffer from raw simulated data");

        let mut buf = Cursor::new(Vec::new());
        if let Err(e) = simulated_img_buf.write_to(&mut buf, image::ImageFormat::Png) {
            return poem::Response::builder()
                .status(poem::http::StatusCode::INTERNAL_SERVER_ERROR)
                .body(format!("Failed to encode image: {}", e));
        }
        let simulated_png_bytes = buf.into_inner();

        return poem::Response::builder()
            .content_type("image/png")
            .body(simulated_png_bytes);
    }

    poem::Response::builder()
        .status(poem::http::StatusCode::BAD_REQUEST)
        .body("No image file provided. Please upload an image file with the 'upload' field name.")
}
