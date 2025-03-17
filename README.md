# Text Imp

Python bindings for iMessage and Contacts database access.

## Requirements

- Python >= 3.8
- macOS (for iMessage database access)
- [uv](https://github.com/astral-sh/uv) package manager (recommended) or pip

## Installation

This package requires Python 3.8 or later. We recommend using [uv](https://github.com/astral-sh/uv) for package management.

### Using uv (Recommended)

```bash
uv pip install text_imp
```

### Using pip

```bash
pip install text_imp
```

## Development Setup

1. Clone the repository:
```bash
git clone https://github.com/yourusername/text_imp.git
cd text_imp
```

2. Install uv if you haven't already:
```bash
curl -LsSf https://astral.sh/uv/install.sh | sh
```

3. Create a virtual environment and install dependencies:
```bash
uv venv
source .venv/bin/activate  # On Unix/macOS
# or
.venv\Scripts\activate  # On Windows

# Install the package in editable mode with all dependencies
uv pip install -e .
```

## Usage Example

```python
import text_imp

# Get messages
messages = text_imp.get_messages()
print(messages)

# Get contacts
contacts = text_imp.get_contacts()
print(contacts)

# Get attachments
attachments = text_imp.get_attachments()
print(attachments)

# Get chats
chats = text_imp.get_chats()
print(chats)

# Get chat handles
handles = text_imp.get_chat_handles()
print(handles)
```

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

## Building from Source

The package uses Maturin for building the Rust extensions. To build from source:

```bash
# Using uv
uv pip install -e .

# Or verify the installation
uv run --with text_imp --no-project -- python -c "import text_imp"
```

## Troubleshooting

If you encounter the error `AttributeError: module 'text_imp' has no attribute 'get_messages'`, try the following:

1. Make sure you're on macOS (this package only works on macOS)
2. Reinstall the package:
```bash
uv pip uninstall text_imp
uv pip install text_imp
```

3. If installing from source, rebuild the package:
```bash
uv pip install -e .
```

## License

MIT License - see LICENSE file for details.
