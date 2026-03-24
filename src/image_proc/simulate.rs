const GAMMA: f64 = 2.2;

// Viénot 1999 step 3: BT.709 display RGB -> LMS after the paper's
// Judd-Vos and Smith-Pokorny transforms.
const M_RGB_TO_LMS: [[f64; 3]; 3] = [
    [17.8824, 43.5161, 4.11935],
    [3.45565, 27.1554, 3.86714],
    [0.029_956_6, 0.184_309, 1.46709],
];

// Inverse of M_RGB_TO_LMS, precomputed once so the per-pixel path stays simple.
const M_LMS_TO_RGB: [[f64; 3]; 3] = [
    [0.080_944_447_9, -0.130_504_409, 0.116_721_066],
    [-0.010_248_533_5, 0.054_019_326_6, -0.113_614_708],
    [-0.000_365_296_938, -0.004_121_614_69, 0.693_511_405],
];

#[derive(Clone, Copy)]
enum Deficiency {
    Deutan,
    Protan,
}

impl Deficiency {
    fn parse(deficiency: &str) -> Result<Self, &'static str> {
        match deficiency.trim().to_ascii_lowercase().as_str() {
            "deutan" | "deuteranopia" => Ok(Self::Deutan),
            "protan" | "protanopia" => Ok(Self::Protan),
            "tritan" | "tritanopia" => {
                Err("Tritan simulation is not implemented in this source-backed Viénot 1999 build")
            }
            _ => Err("Invalid deficiency type. Use 'deutan' or 'protan'"),
        }
    }

    fn scale_params(self) -> (f64, f64) {
        match self {
            Self::Protan => (0.992_052, 0.003_974),
            Self::Deutan => (0.957_237, 0.021_381_4),
        }
    }

    fn simulation_matrix(self) -> [[f64; 3]; 3] {
        match self {
            Self::Protan => [[0.0, 2.02344, -2.52581], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
            Self::Deutan => [[1.0, 0.0, 0.0], [0.494_207, 0.0, 1.24827], [0.0, 0.0, 1.0]],
        }
    }
}

fn multiply_matrix_vector(matrix: [[f64; 3]; 3], vector: [f64; 3]) -> [f64; 3] {
    [
        matrix[0][0] * vector[0] + matrix[0][1] * vector[1] + matrix[0][2] * vector[2],
        matrix[1][0] * vector[0] + matrix[1][1] * vector[1] + matrix[1][2] * vector[2],
        matrix[2][0] * vector[0] + matrix[2][1] * vector[1] + matrix[2][2] * vector[2],
    ]
}

fn decode_gamma(value: u8) -> f64 {
    (value as f64 / 255.0).powf(GAMMA)
}

fn encode_gamma(value: f64) -> u8 {
    (value.clamp(0.0, 1.0).powf(1.0 / GAMMA) * 255.0).round() as u8
}

fn simulate_deficiency(rgb: &[u8], deficiency: Deficiency) -> Vec<u8> {
    let (scale, bias) = deficiency.scale_params();
    let simulation_matrix = deficiency.simulation_matrix();
    let mut result = Vec::with_capacity(rgb.len());

    for chunk in rgb.chunks_exact(3) {
        let rgb_linear = [
            decode_gamma(chunk[0]),
            decode_gamma(chunk[1]),
            decode_gamma(chunk[2]),
        ];

        // Viénot 1999 step 2: shrink the source gamut before projection so
        // the simulated colors remain inside the display gamut.
        let reduced_rgb = [
            scale * rgb_linear[0] + bias,
            scale * rgb_linear[1] + bias,
            scale * rgb_linear[2] + bias,
        ];
        let lms = multiply_matrix_vector(M_RGB_TO_LMS, reduced_rgb);
        let simulated_lms = multiply_matrix_vector(simulation_matrix, lms);
        let simulated_rgb = multiply_matrix_vector(M_LMS_TO_RGB, simulated_lms);

        result.push(encode_gamma(simulated_rgb[0]));
        result.push(encode_gamma(simulated_rgb[1]));
        result.push(encode_gamma(simulated_rgb[2]));
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

    let deficiency = Deficiency::parse(deficiency)?;
    Ok(simulate_deficiency(&rgb_data, deficiency))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protan_reference_outputs() {
        let rgb_data = vec![
            255, 0, 0, //
            0, 255, 0, //
            0, 0, 255, //
            128, 128, 128,
        ];

        let result = simulate_color_blindness(rgb_data, "protan").unwrap();
        assert_eq!(
            result,
            vec![
                96, 96, 28, //
                241, 241, 0, //
                21, 21, 255, //
                129, 129, 129,
            ]
        );
    }

    #[test]
    fn test_deutan_reference_outputs() {
        let rgb_data = vec![
            255, 0, 0, //
            0, 255, 0, //
            0, 0, 255, //
            128, 128, 128,
        ];

        let result = simulate_color_blindness(rgb_data, "deutan").unwrap();
        assert_eq!(
            result,
            vec![
                148, 148, 0, //
                217, 217, 61, //
                44, 44, 253, //
                131, 131, 131,
            ]
        );
    }

    #[test]
    fn test_aliases_match_canonical_deficiencies() {
        let rgb_data = vec![255, 128, 64];

        assert_eq!(
            simulate_color_blindness(rgb_data.clone(), "protan").unwrap(),
            simulate_color_blindness(rgb_data.clone(), "protanopia").unwrap(),
        );
        assert_eq!(
            simulate_color_blindness(rgb_data.clone(), "deutan").unwrap(),
            simulate_color_blindness(rgb_data.clone(), "deuteranopia").unwrap(),
        );
    }

    #[test]
    fn test_invalid_deficiency_is_rejected() {
        let rgb_data = vec![255, 0, 0];
        let result = simulate_color_blindness(rgb_data, "invalid");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Invalid deficiency type. Use 'deutan' or 'protan'"
        );
    }

    #[test]
    fn test_tritan_is_explicitly_unsupported() {
        let rgb_data = vec![255, 0, 0];
        let result = simulate_color_blindness(rgb_data, "tritan");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Tritan simulation is not implemented in this source-backed Viénot 1999 build"
        );
    }

    #[test]
    fn test_invalid_data_length() {
        let rgb_data = vec![255, 0];
        let result = simulate_color_blindness(rgb_data, "deutan");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Input data length must be a multiple of 3 (RGB pixels)"
        );
    }
}
