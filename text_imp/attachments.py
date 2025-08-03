import os

import polars as pl

from .text_imp import get_attachments

MESSAGE_DB_PATH = os.path.expanduser("~/Library/Messages/chat.db")


def get_attachments_with_guid():
    attachment_df = get_attachments()

    guid_query = """
        SELECT ROWID, guid 
        FROM attachment
    """
    guid_df = pl.read_database_uri(query=guid_query, uri=f"sqlite://{MESSAGE_DB_PATH}")

    return attachment_df.join(guid_df, left_on="rowid", right_on="ROWID", how="left")
