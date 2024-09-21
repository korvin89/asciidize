Application aims to provide ability to generate ascii art from images independently of target fonts and symbols.

Should consist of two main parts:

1. Alphabet Mapper - generates mapping data for any font from image.
2. Image Converter - converts image to ascii art using mapping data.

## Alphabet Mapper

MVP should do the following:

1. Print alphabet symbols in console.
2. Generate mapping data for screenshot of alphabet symbols.
3. Save mapping data to file.

Proposed format for mapping data:

```json
{
  "symbol_width": 3,
  "symbol_height": 3,
  "symbol_map": {
    "A": [[1,1,1],[1,0,1],[1,1,1]],
    ...
  }
}
```

## Image Converter

MVP should do the following:

1. Load mapping data from file.
2. Convert image to grayscale.
3. Split image to blocks of symbol size.
4. Find the most similar symbol for each block.
5. Print ascii art in console.
