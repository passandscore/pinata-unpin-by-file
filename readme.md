## Program Installation and Execution Guide

This guide provides instructions on how to install and run the program.

### Prerequisites

- Rust programming language: Make sure you have Rust installed on your system. You can download and install Rust from the [official website](https://www.rust-lang.org/tools/install).

### Installation Steps

1. Clone the repository: Open a terminal or command prompt and execute the following command to clone the repository:

```js
git clone https://github.com/passandscore/pinata-unpin-by-file.git
cd pinata-unpin-by-file
cargo build
```

## Execution Steps

Prepare the input file: Create a file containing the hashes you want to process. Each hash should be on a separate line, and the file should be in CSV format.

Run the program: Execute the following command to run the program, providing the path to the input file:

```js
cargo run
```

The program will prompt you to enter the path to the input file. Enter the path and press Enter. The program will process the hashes and output the results to the terminal.
