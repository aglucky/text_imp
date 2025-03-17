# Text Imp

Python bindings for iMessage and Contacts database access. Requires MacOS and full file system access to run.

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

# Get  handles
handles = text_imp.get_handles()
print(handles)

# Get chat handles
chat_handles = text_imp.get_chat_handles()
print(handles)
```

## Example Data

Each function returns a Polars DataFrame with structured data. Here are examples of what the returned data looks like:

### Messages DataFrame

**Columns:** `date, text, is_from_me, handle_id, chat_id, guid, thread_originator_guid, thread_originator_part, service_type, variant, expressive, announcement_type, num_attachments, is_deleted, group_title, is_edited, is_tapback, is_reply, num_replies, date_delivered, date_read, is_url, has_replies, body`

![Message DataFrame Example](examples/screenshots/message.png)

### Contacts DataFrame

**Columns:** `contact_id, first_name, last_name, state, city, normalized_contact_id`

![Contacts DataFrame Example](examples/screenshots/contacts.png)

### Attachments DataFrame

**Columns:** `rowid, filename, uti, mime_type, transfer_name, emoji_description, is_sticker, path, extension, display_filename, file_size`

![Attachments DataFrame Example](examples/screenshots/attach.png)

### Chats DataFrame

**Columns:** `rowid, chat_identifier, service_name, display_name, name, resolved_display_name`

![Chats DataFrame Example](examples/screenshots/chat.png)

### Handles DataFrame

**Columns:** `rowid, id, person_centric_id`

![Handles DataFrame Example](examples/screenshots/handle.png)

### Chat Handles DataFrame

**Columns:** `chat_id, handle_id`

![Chat Handles DataFrame Example](examples/screenshots/chat_handle.png)

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
