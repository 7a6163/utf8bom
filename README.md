utf8bom - UTF-8 BOM (Byte Order Mark) Tool

utf8bom is a command-line tool written in Rust for adding or removing UTF-8 BOM (Byte Order Mark) from text files.

Usage

To add the UTF-8 BOM to a file, use the following command:

$ utf8bom input.txt output.txt

Installation

To install utf8bom, download the binary from the releases page and add it to your system’s PATH. Alternatively, you can build it from source by following these steps:

1. Install Rust if you haven’t already: https://www.rust-lang.org/tools/install.
2. Clone the utf8bom repository.
3. Navigate to the repository’s directory.
4. Build the binary using the following command:
$ cargo build --release

5. The compiled binary will be located at target/release/utf8bom.

Contributing

Contributions, bug reports, and feature requests are welcome! Please feel free to open issues or submit pull requests on the GitHub repository.

License

utf8bom is distributed under the MIT License. See the LICENSE file for more information.

If you have any questions, please don’t hesitate to ask. Thank you for your interest and support!