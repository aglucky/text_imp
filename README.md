# Text Imp

Python bindings for iMessage and Contacts database access.

## Installation

This package requires Python 3.8 or later. We use [uv](https://github.com/astral-sh/uv) for package management.

```bash
uv pip install text_imp
```

## Development Setup

1. Clone the repository:
```bash
git clone https://github.com/yourusername/text_imp.git
cd text_imp
```

1. Install uv if you haven't already:
```bash
curl -LsSf https://astral.sh/uv/install.sh | sh
```

1. Create a virtual environment and install dependencies:
```bash
uv venv
source .venv/bin/activate  # On Unix/macOS
# or
.venv\Scripts\activate  # On Windows

# Install the package in editable mode with all dependencies
uv pip install -e .
```

## Building from Source

The package uses Maturin for building the Rust extensions. To build from source in one line:

```bash
uv pip install -e .
```

Or if you want to build and verify the installation:

```bash
uv pip install -e . && uv run --with text_imp --no-project -- python -c "import text_imp"
```

## Usage

See `example.py`

## Requirements

- Python >= 3.8
- macOS (for iMessage database access)
- [uv](https://github.com/astral-sh/uv) package manager

## Project Structure

```txt
text_imp/
├── src/           # Rust source code
├── text_imp/      # Python package directory
├── examples/      # Usage examples
├── tests/         # Test files
├── Cargo.toml     # Rust dependencies and configuration
└── pyproject.toml # Python package configuration
```
