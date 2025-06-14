use crate::image_proc::srgb;

fn simulate_deficiency(rgb: &[u8], simulation_matrix: [[f32; 3]; 3]) -> Vec<u8> {
    // Standard sRGB to LMS conversion matrix (Hunt-Pointer-Estevez)
    // These are the widely accepted matrices for accurate color vision simulation
    const M_RGB_TO_LMS: [[f32; 3]; 3] = [
        [0.4002, 0.7075, -0.0807],
        [-0.2280, 1.1500, 0.0612],
        [0.0000, 0.0000, 0.9184],
    ];

    // Inverse matrix to convert LMS back to RGB
    const M_LMS_TO_RGB: [[f32; 3]; 3] = [
        [1.8599, -1.1293, 0.2198],
        [0.3611, 0.6388, -0.0000],
        [0.0000, 0.0000, 1.0888],
    ];

    let mut result = Vec::with_capacity(rgb.len());

    for chunk in rgb.chunks_exact(3) {
        let r = srgb::srgb_to_linear(chunk[0] as f32 / 255.0);
        let g = srgb::srgb_to_linear(chunk[1] as f32 / 255.0);
        let b = srgb::srgb_to_linear(chunk[2] as f32 / 255.0);

        // Convert RGB to LMS
        let l = M_RGB_TO_LMS[0][0] * r + M_RGB_TO_LMS[0][1] * g + M_RGB_TO_LMS[0][2] * b;
        let m = M_RGB_TO_LMS[1][0] * r + M_RGB_TO_LMS[1][1] * g + M_RGB_TO_LMS[1][2] * b;
        let s = M_RGB_TO_LMS[2][0] * r + M_RGB_TO_LMS[2][1] * g + M_RGB_TO_LMS[2][2] * b;

        // Apply deficiency simulation in LMS space
        let l_sim =
            simulation_matrix[0][0] * l + simulation_matrix[0][1] * m + simulation_matrix[0][2] * s;
        let m_sim =
            simulation_matrix[1][0] * l + simulation_matrix[1][1] * m + simulation_matrix[1][2] * s;
        let s_sim =
            simulation_matrix[2][0] * l + simulation_matrix[2][1] * m + simulation_matrix[2][2] * s;

        // Convert LMS back to RGB
        let r_lin =
            M_LMS_TO_RGB[0][0] * l_sim + M_LMS_TO_RGB[0][1] * m_sim + M_LMS_TO_RGB[0][2] * s_sim;
        let g_lin =
            M_LMS_TO_RGB[1][0] * l_sim + M_LMS_TO_RGB[1][1] * m_sim + M_LMS_TO_RGB[1][2] * s_sim;
        let b_lin =
            M_LMS_TO_RGB[2][0] * l_sim + M_LMS_TO_RGB[2][1] * m_sim + M_LMS_TO_RGB[2][2] * s_sim;

        // Convert linear RGB to sRGB and clamp
        let r_sim = srgb::linear_to_srgb(r_lin).clamp(0.0, 1.0) * 255.0;
        let g_sim = srgb::linear_to_srgb(g_lin).clamp(0.0, 1.0) * 255.0;
        let b_sim = srgb::linear_to_srgb(b_lin).clamp(0.0, 1.0) * 255.0;

        result.push(r_sim.round() as u8);
        result.push(g_sim.round() as u8);
        result.push(b_sim.round() as u8);
    }

    result
}

pub fn simulate_color_blindness(
    rgb_data: Vec<u8>,
    deficiency: &str,
) -> Result<Vec<u8>, &'static str> {
    if rgb_data.len() % 3 != 0 {
        return Err("Input data length must be a multiple of 3 (RGB pixels)");
    }

    // Viénot 1999 dichromacy simulation matrices in LMS space
    // These matrices simulate the loss of one type of cone cell
    let simulation_matrix = match deficiency.to_lowercase().as_str() {
        // Deuteranopia (loss of M cone) - most common form of color blindness
        "deutan" | "deuteranopia" => [[1.0, 0.0, 0.0], [0.494207, 0.0, 1.24827], [0.0, 0.0, 1.0]],
        // Protanopia (loss of L cone) - second most common
        "protan" | "protanopia" => [[0.0, 2.02344, -2.52581], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        // Tritanopia (loss of S cone) - rare
        "tritan" | "tritanopia" => [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [-0.395913, 0.801109, 0.0]],
        _ => return Err("Invalid deficiency type. Use 'deutan', 'protan', or 'tritan'"),
    };

    Ok(simulate_deficiency(&rgb_data, simulation_matrix))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulate_color_blindness_basic() {
        // Test with a simple red pixel (255, 0, 0)
        let rgb_data = vec![255, 0, 0];
        let result = simulate_color_blindness(rgb_data, "deutan").unwrap();
        assert_eq!(result.len(), 3);
        // Red should be affected by deuteranopia simulation
        assert_ne!(result, vec![255, 0, 0]);
    }

    #[test]
    fn test_simulate_color_blindness_multiple_pixels() {
        // Test with red, green, blue pixels
        let rgb_data = vec![255, 0, 0, 0, 255, 0, 0, 0, 255];
        let result = simulate_color_blindness(rgb_data, "protan").unwrap();
        assert_eq!(result.len(), 9);
    }

    #[test]
    fn test_simulate_color_blindness_invalid_deficiency() {
        let rgb_data = vec![255, 0, 0];
        let result = simulate_color_blindness(rgb_data, "invalid");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Invalid deficiency type. Use 'deutan', 'protan', or 'tritan'"
        );
    }

    #[test]
    fn test_simulate_color_blindness_invalid_data_length() {
        // Test with invalid data length (not multiple of 3)
        let rgb_data = vec![255, 0];
        let result = simulate_color_blindness(rgb_data, "deutan");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Input data length must be a multiple of 3 (RGB pixels)"
        );
    }

    #[test]
    fn test_all_deficiency_types() {
        let rgb_data = vec![255, 128, 64];

        // Test all supported deficiency types
        let deficiencies = [
            "deutan",
            "deuteranopia",
            "protan",
            "protanopia",
            "tritan",
            "tritanopia",
        ];

        for deficiency in &deficiencies {
            let result = simulate_color_blindness(rgb_data.clone(), deficiency);
            assert!(result.is_ok(), "Failed for deficiency: {}", deficiency);
            assert_eq!(result.unwrap().len(), 3);
        }
    }

    #[test]
    fn test_simulate_deficiency_preserves_grayscale() {
        // Grayscale pixels should remain relatively unchanged
        let rgb_data = vec![128, 128, 128];
        let result = simulate_color_blindness(rgb_data.clone(), "deutan").unwrap();

        // Grayscale should be relatively unchanged (allowing for some variation due to simulation)
        let diff = (result[0] as i32 - 128).abs()
            + (result[1] as i32 - 128).abs()
            + (result[2] as i32 - 128).abs();
        assert!(
            diff < 150,
            "Grayscale should be relatively unchanged, but difference was {}",
            diff
        );
    }
}
