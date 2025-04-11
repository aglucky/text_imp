from text_imp import (
    get_attachments,
    get_chat_handles,
    get_chats,
    get_handles,
    get_messages,
)

from .attachments import get_attachments_with_guid
from .contacts import get_contacts

# Re-export the functions from text_imp
get_attachments = get_attachments
get_chat_handles = get_chat_handles
get_chats = get_chats
get_handles = get_handles
get_messages = get_messages

__all__ = [
    "get_attachments",
    "get_chat_handles",
    "get_chats",
    "get_handles",
    "get_messages",
    "get_attachments_with_guid",
    "get_contacts",
]
