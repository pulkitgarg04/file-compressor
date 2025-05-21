# File Compressor

A simple command-line utility to compress files using the Gzip format. This tool allows you to easily compress files to save disk space or prepare them for sharing.

## Features
1. Compresses files using the Gzip compression algorithm.
2. Easy-to-use command-line interface.
3. Provides meaningful error messages for invalid file paths or permissions.

## Requirements
- **Rust**: Ensure you have Rust installed. You can download it from [rust-lang.org](https://www.rust-lang.org/tools/install).

## Installation
1. Clone the repository:
```bash
git clone https://github.com/pulkitgarg04/file-compressor.git
cd file-compressor
```

2. Build the project:
```bash
cargo build --release
```

3. The executable will be available in the target/release directory.

## Usage
Run the file compressor with the following syntax:
```bash
./target/release/file_compressor --input <input_file> --output <output_file>
```

#### Options
|Argument|Description|Required|
|--------|-----------|--------|
|-i|--input|The path to the input file to compress|Yes|
|-o|--output|The path to the output compressed file|Yes|

## Contributing
Contributions are welcome! Feel free to fork this repository and submit a pull request with your improvements or new features.

## License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.