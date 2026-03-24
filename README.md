# Colrskeww

## Overview

The implementation now follows the **Viénot 1999** display model directly for **protanopia** and **deuteranopia**. It uses the paper's gamma model, deficiency-specific gamut reduction step, BT.709/Judd-Vos/Smith-Pokorny RGB-to-LMS transform, and the published projection matrices.



## Algorithm Flow

```
Input RGB Image
       ↓
Decode Gamma (power 2.2)
       ↓
Apply Deficiency-Specific Gamut Reduction
       ↓
Transform to LMS Color Space
       ↓
Apply Dichromacy Simulation Matrix
       ↓
Transform back to RGB
       ↓
Encode Gamma (power 1/2.2)
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
- **Supported types**: `deutan`, `deuteranopia`, `protan`, `protanopia`

## Scientific Accuracy

This implementation is based on the following research:

1. **Viénot, F., Brettel, H., & Mollon, J. D. (1999)**. "Digital video colourmaps for checking the legibility of displays by dichromats." Color Research & Application, 24(4), 243-252.

The algorithm provides a source-backed simulation of complete **protanopia** and **deuteranopia** for the display model described in the paper. It does not simulate anomalous trichromacy, and it does not currently include a source-backed tritanopia path.

## Limitations

1. **Protan/Deutan Only**: This build intentionally supports only the Viénot 1999 paths that are fully implemented and tested
2. **Dichromacy Only**: Simulates complete loss of cone cells, not reduced sensitivity
3. **No Individual Variation**: Uses average population parameters
4. **Display Model Bound**: The transform matches the display assumptions in the paper rather than every possible image pipeline

## TODO

- **Memory Efficient**: Processes images in chunks to minimize memory usage
- **SIMD Potential**: Matrix operations could be optimized with SIMD instructions
- **Parallel Processing**: Could be parallelized for large images using rayon

## References

- [Viénot 1999 Original Paper](https://vision.psychol.cam.ac.uk/jdmollon/papers/colourmaps.pdf)
