# `ico`

A simple command-line utility to generate Windows ICO files

## Features

- Generate ICO files from PNG or SVG images
- Create icons with multiple sizes in a single ICO file
- Extract frames from existing ICO files
- View detailed information about ICO files
- Support for both icon and cursor resources

## Usage

### Generate ICO file

cmd: `generate`, `gen`, `create`, `encode`, `new`, `pack`

- Generate ICO with default sizes (16,32,48,64,256)

    ```bash
    ico generate input.png --output icon.ico
    ```

- Generate ICO with custom sizes
    
    ```bash
    ico generate input.svg --output icon.ico --sizes 32,64,128
    ```

### View ICO information

cmd: `info`, `show`, `inspect`, `describe`, `details`

- View ICO info in human-readable format

    ```bash
    ico info icon.ico
    ```

- View ICO info in JSON format
    
    ```
    ico info icon.ico --json
    ```

### Extract ICO frames

cmd: `extract`, `unpack`, `decode`, `dump`

- Extract all frames as PNG files

    ```bash
    ico extract icon.ico --output frames/
    ```

---

## License

This project is licensed under the [MIT License](./LICENSE)
