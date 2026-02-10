# env-parser

A command-line tool written in Rust for validating and formatting `.env` files. It helps maintain clean, consistent environment variable files by removing unnecessary quotes and validating configuration values.

## Features

- ✅ **Format .env files**: Automatically removes unnecessary quotes (single and double) from values
- ✅ **Validate configuration**: Checks for valid ports, URLs, and required fields
- ✅ **Preserve comments**: Keeps comments and empty lines intact
- ✅ **Safe formatting**: Only removes quotes that wrap the entire value, preserving quotes within values

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) 1.70 or higher
- Cargo (comes with Rust)

### Build from source

```bash
git clone https://github.com/victorcesc/env-parser.git
cd env-parser
cargo build --release
```

The binary will be located at `target/release/env-parser`.

## Usage

### Format a .env file

Format and display the result (without modifying the file):

```bash
cargo run -- .env --format
```

Format and save the changes back to the file:

```bash
cargo run -- .env --format --write
```

Or if installed:

```bash
env-parser .env --format --write
```

### Validate a .env file

```bash
cargo run -- .env
```

## Examples

### Before formatting

```env
KEY="value"
ANOTHER_KEY='value with spaces'
DATABASE_URL="mongodb://user:pass@host/db"
# This is a comment
EMPTY_KEY=""
```

### After formatting

```env
KEY=value
ANOTHER_KEY=value with spaces
DATABASE_URL=mongodb://user:pass@host/db
# This is a comment
EMPTY_KEY=
```

### What gets preserved

- Quotes within values: `KEY=value"with"quotes` → stays as is
- Comments: `# Comment` → preserved
- Empty lines: preserved
- Values with quotes in the middle: `KEY=start"middle"end` → preserved

## Validation Rules

The validator checks:

- **PORT**: Must be greater than 1024
- **HOST**: Cannot be empty
- **DATABASE_URL**: Must be a valid URL format

## Project Structure

```
env-parser/
├── src/
│   ├── main.rs          # CLI entry point and argument parsing
│   ├── config.rs        # Configuration structure
│   ├── errors.rs        # Error types using thiserror
│   ├── env_loader.rs    # Environment variable loading
│   ├── formatter.rs    # .env file formatting logic
│   └── rules.rs        # Validation rules
├── Cargo.toml          # Project dependencies
└── README.md           # This file
```

## Dependencies

- **clap**: Command-line argument parsing
- **dotenvy**: Loading `.env` files
- **thiserror**: Error handling
- **url**: URL validation

## Development

### Run tests

```bash
cargo test
```

### Check code

```bash
cargo check
```

### Build

```bash
cargo build
```

### Run in debug mode

```bash
cargo run -- .env --format
```

## Error Handling

The tool uses Rust's `Result` type for error handling and provides clear error messages:

- `LoadError`: File read/write errors
- `MissingEnvironmentVariable`: Required environment variable not found
- `InvalidValue`: Invalid value format or validation failure

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

[Add your license here]

## Author

**Victor Cesconetto**

- Email: victorcesconetto@gmail.com

