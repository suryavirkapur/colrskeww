# Colrskeww

## Overview

The implementation uses the **Viénot 1999** algorithm, which is a widely accepted and scientifically validated method for simulating dichromacy (complete loss of one type of cone cell). This approach transforms colors through the LMS color space to accurately simulate the visual experience of people with color blindness.



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

## TODO

- **Memory Efficient**: Processes images in chunks to minimize memory usage
- **SIMD Potential**: Matrix operations could be optimized with SIMD instructions
- **Parallel Processing**: Could be parallelized for large images using rayon

## References

- [Viénot 1999 Original Paper](https://vision.psychol.cam.ac.uk/jdmollon/papers/colourmaps.pdf)
