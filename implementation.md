# Color Blindness Simulation Implementation

This document explains the color blindness simulation implementation in this project, which simulates how people with different types of color vision deficiency (CVD) perceive colors.

## Overview

The implementation uses the **Viénot 1999** algorithm, which is a widely accepted and scientifically validated method for simulating dichromacy (complete loss of one type of cone cell). This approach transforms colors through the LMS color space to accurately simulate the visual experience of people with color blindness.

## Implementation Architecture

### 1. sRGB Gamma Correction (`src/image_proc/srgb.rs`)

Color vision simulation must be performed in linear light space, not gamma-corrected sRGB space. The implementation includes proper gamma correction functions:

- `srgb_to_linear()`: Converts sRGB values to linear RGB
- `linear_to_srgb()`: Converts linear RGB back to sRGB

These functions use the standard sRGB gamma correction formula with the precise thresholds and coefficients defined in the sRGB specification.

### 2. Color Space Transformation (`src/image_proc/simulate.rs`)

The simulation process involves several color space transformations:

1. **sRGB → Linear RGB**: Remove gamma correction
2. **Linear RGB → LMS**: Transform to cone response space
3. **LMS Simulation**: Apply color blindness simulation matrix
4. **LMS → Linear RGB**: Transform back to RGB
5. **Linear RGB → sRGB**: Apply gamma correction

#### RGB to LMS Conversion

The implementation uses the Hunt-Pointer-Estevez transformation matrices:

```rust
// RGB to LMS transformation matrix
const M_RGB_TO_LMS: [[f32; 3]; 3] = [
    [0.4002, 0.7075, -0.0807],
    [-0.2280, 1.1500, 0.0612],
    [0.0000, 0.0000, 0.9184],
];

// LMS to RGB transformation matrix (inverse of above)
const M_LMS_TO_RGB: [[f32; 3]; 3] = [
    [1.8599, -1.1293, 0.2198],
    [0.3611, 0.6388, -0.0000],
    [0.0000, 0.0000, 1.0888],
];
```

These matrices are based on established research and provide accurate transformation between RGB and LMS color spaces.

### 3. Dichromacy Simulation Matrices

The simulation uses different matrices for each type of color blindness:

#### Deuteranopia (Green-blind)
- **Prevalence**: Most common form (~5% of males)
- **Description**: Loss of M (medium wavelength) cone cells
- **Matrix**: Projects LMS colors onto a plane that excludes M cone responses

#### Protanopia (Red-blind)
- **Prevalence**: Second most common (~2% of males)
- **Description**: Loss of L (long wavelength) cone cells
- **Matrix**: Projects LMS colors onto a plane that excludes L cone responses

#### Tritanopia (Blue-blind)
- **Prevalence**: Very rare (~0.01% of population)
- **Description**: Loss of S (short wavelength) cone cells
- **Matrix**: Projects LMS colors onto a plane that excludes S cone responses

## Algorithm Flow

```
Input RGB Image
       ↓
Convert to Linear RGB (remove gamma)
       ↓
Transform to LMS Color Space
       ↓
Apply Dichromacy Simulation Matrix
       ↓
Transform back to Linear RGB
       ↓
Apply Gamma Correction (to sRGB)
       ↓
Output Simulated Image
```

## API Usage

The main function is `simulate_color_blindness()`:

```rust
pub fn simulate_color_blindness(
    rgb_data: Vec<u8>,
    deficiency: &str,
) -> Result<Vec<u8>, &'static str>
```

**Parameters:**
- `rgb_data`: Raw RGB pixel data (must be multiple of 3 bytes)
- `deficiency`: Type of color blindness to simulate

**Supported deficiency types:**
- `"deutan"` or `"deuteranopia"` - Green color blindness
- `"protan"` or `"protanopia"` - Red color blindness
- `"tritan"` or `"tritanopia"` - Blue color blindness

## HTTP API

The web handler (`src/handlers/handle_image.rs`) provides a multipart form endpoint:

- **Endpoint**: POST with multipart/form-data
- **Fields**:
  - `upload`: Image file (PNG, JPEG, etc.)
  - `deficiency`: Color blindness type (optional, defaults to "deutan")
- **Response**: PNG image showing simulated color blindness view

## Scientific Accuracy

This implementation is based on the following research:

1. **Viénot, F., Brettel, H., & Mollon, J. D. (1999)**. "Digital video colourmaps for checking the legibility of displays by dichromats." Color Research & Application, 24(4), 243-252.

2. **Hunt-Pointer-Estevez LMS transformation** - Standard transformation matrices for converting between RGB and LMS color spaces.

The algorithm provides scientifically accurate simulation of complete dichromacy (total loss of one cone type). It does not simulate anomalous trichromacy (reduced sensitivity of one cone type), which is actually more common than complete dichromacy.

## Limitations

1. **Dichromacy Only**: Simulates complete loss of cone cells, not reduced sensitivity
2. **No Individual Variation**: Uses average population parameters
3. **Display Limitations**: Final output is limited by the display device's color gamut
4. **Lighting Conditions**: Assumes standard viewing conditions

## Testing

The implementation includes comprehensive unit tests covering:
- Basic functionality with single pixels
- Multiple pixel processing
- Error handling for invalid inputs
- All supported deficiency types
- Edge cases and validation

Run tests with:
```bash
cargo test simulate
```

## Performance Considerations

- **Memory Efficient**: Processes images in chunks to minimize memory usage
- **SIMD Potential**: Matrix operations could be optimized with SIMD instructions
- **Parallel Processing**: Could be parallelized for large images using rayon

## References

- [Viénot 1999 Original Paper](https://vision.psychol.cam.ac.uk/jdmollon/papers/colourmaps.pdf)
