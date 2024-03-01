# Hash It!

`hashit` is a command-line utility designed for processing columns of text into hash values. It supports both SHA3 and SHA256 hashing algorithms, making it a versatile tool for developers, security professionals, and anyone in need of reliable hash generation from text inputs.

## Installation

To install `hashit`, you will need Rust installed on your machine. If you don't have Rust, you can install it by following the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).

Once Rust is installed, you can install `hashit` by cloning the repository and using Cargo, Rust's package manager and build system:

```bash
git clone https://github.com/jac18281828/hashit.git
cd hashit
cargo install --path .
```

## Building

Building `hashit` from source is straightforward with Cargo. After cloning the repository and navigating into the project directory, run the following command to compile the project:

```bash
cargo build --release
```

The compiled binary will be located in `./target/release/hashit`. You can move this binary to a directory in your PATH for easier access.

## Testing

To run the tests for `hashit`, navigate to the project directory and use the following command:

```bash
cargo build --release
```

## Build using Docker

To build the project using Docker invoke the following command

```bash
docker build . -t hashit:1
```

## Contributing

Contributions to `hashit` are welcome! Whether you're fixing bugs, adding new features, or improving documentation, your help is appreciated. To contribute:

1. Fork the repository on GitHub.
2. Clone your forked repository to your local machine.
3. Create a new branch for your feature or bug fix.
4. Make your changes and commit them with clear, descriptive messages.
5. Push your changes to your fork on GitHub.
6. Submit a pull request to the main `hashit` repository.

Before contributing, please read through any existing issues or discussions to see if someone else is already working on something similar.

## License

`hashit` is licensed under the [BSD-3-Clause](LICENSE). This license allows you to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the software and to permit persons to whom the software is furnished to do so, subject to certain limitations.