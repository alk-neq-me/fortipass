# FortiPass - A Rust-Based Password Manager

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

FortiPass is a secure and reliable password manager implemented in Rust. It provides a command-line interface (CLI) for managing your passwords with a strong focus on security and ease of use.

## Features

- **Strong Encryption:** FortiPass uses industry-standard encryption algorithms to keep your passwords and sensitive data secure.

- **Command-Line Interface:** Manage your passwords from the command line, making it easy to integrate with other tools and scripts.

- **MIT License:** FortiPass is open-source and released under the MIT License, allowing you to use, modify, and distribute it freely.

## Getting Started

### Prerequisites

Before using FortiPass, ensure that you have Rust installed on your system. You can download and install Rust from the official website: [Rust Programming Language](https://www.rust-lang.org/).

### Installation

1. Clone the FortiPass repository:

   ```bash
   git clone https://github.com/alk-neq-me/fortipass.git
   ```

2. Navigate to the project directory:

   ```bash
   cd fortipass
   ```

3. Build the FortiPass CLI:

   ```bash
   make build
   ```

### Usage

#### Running FortiPass

To run FortiPass, use the following command:

```bash
make run
```

This will start the FortiPass CLI, allowing you to interact with your password manager.

#### Running Tests

To run the test suite, use the following command:

```bash
make test
```

This will execute the test cases to ensure the integrity of the code.

### Cleaning

To clean the build artifacts and dependencies, you can use the following command:

```bash
make clean
```

## License

FortiPass is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.

## Acknowledgments

We would like to thank the open-source community for their valuable contributions and support.

## Disclaimer

FortiPass is provided "as is" without warranty of any kind. Use it at your own risk.
