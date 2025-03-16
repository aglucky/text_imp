from .impy import get_attachments
import polars as pl
import os

MESSAGE_DB_PATH = os.path.expanduser("~/Library/Messages/chat.db")


def get_attachments_with_guid():
    attachmentDf = get_attachments()

    guidQuery = """
        SELECT ROWID, guid 
        FROM attachment
    """
    guidDf = pl.read_database_uri(query=guidQuery, uri=f"sqlite://{MESSAGE_DB_PATH}")

    return attachmentDf.join(guidDf, left_on="rowid", right_on="ROWID", how="left")
