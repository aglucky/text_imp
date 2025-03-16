# Text Imp

Python bindings for iMessage and Contacts database access.

## Installation

This package requires Python 3.8 or later. We recommend using [uv](https://github.com/astral-sh/uv) for package management.

```bash
uv pip install text-imp
```

## Development Setup

1. Clone the repository:
```bash
git clone https://github.com/yourusername/text-imp.git
cd text-imp
```

2. Set up a virtual environment and install dependencies:
```bash
uv venv
source .venv/bin/activate  # On Unix/macOS
# or
.venv\Scripts\activate  # On Windows
```

3. Install development dependencies:
```bash
uv pip install -e ".[dev]"
```

## Building from Source

This package uses Maturin for building the Rust extensions. To build from source:

```bash
uv pip install maturin
maturin develop
```

## Usage

Basic usage example:

```python
import text_imp

# Get recent messages
from datetime import datetime, timedelta
since = datetime.now() - timedelta(days=1)
messages = text_imp.get_messages(since=since)

# Print messages
for msg in messages:
    print(f"[{msg.timestamp}] {msg.sender}: {msg.text}")
```

For more examples, check out the [examples directory](examples/basic_usage.py) which includes:
- Listing recent messages
- Searching message content
- Working with contacts
- Managing group chats

## Requirements

- Python >= 3.8
- Rust (for building from source)
- macOS (for iMessage database access)

## License

[Insert License Information]

## Project Structure

```
text-imp/
├── src/           # Rust source code
├── text_imp/      # Python package directory
├── examples/      # Usage examples
├── tests/         # Test files
├── Cargo.toml     # Rust dependencies and configuration
└── pyproject.toml # Python package configuration
```
