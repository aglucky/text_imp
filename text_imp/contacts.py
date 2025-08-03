import sqlite3
from datetime import datetime, timedelta
from pathlib import Path
from typing import List, Union

import polars as pl

DEFAULT_CONTACTS_DB_PATH = [
    *Path.home()
    .joinpath("Library/Application Support/AddressBook/Sources")
    .rglob("AddressBook-v22.abcddb"),
    Path.home().joinpath(
        "Library/Application Support/AddressBook/AddressBook-v22.abcddb"
    ),
]


def normalize_id(contact_id: str) -> str:
    """
    Normalize contact IDs (phone numbers and email addresses).

    Args:
        contact_id: Raw contact ID (phone number or email)

    Returns:
        Normalized contact ID:
        - Email addresses are lowercased and stripped
        - 10-digit phone numbers get +1 prefix
        - 11-digit phone numbers get + prefix
        - Other numbers are returned as-is
    """
    if "@" in contact_id:
        return contact_id.strip().lower()  # Email addresses

    numbers = "".join([c for c in contact_id if c.isdigit()])
    if len(numbers) == 10:
        return f"+1{numbers}"  # Assumes country code is +1 if not present
    elif len(numbers) == 11:
        return f"+{numbers}"
    else:
        return numbers  # Non traditional phone numbers


def coredata_to_datetime(coredata_timestamp):
    coredata_epoch = datetime(2001, 1, 1)
    return coredata_epoch + timedelta(seconds=coredata_timestamp)


def get_contacts(
    db_paths: Union[
        str, Path, List[Union[str, Path]], List[Path]
    ] = DEFAULT_CONTACTS_DB_PATH,
) -> pl.DataFrame:
    """
    Get contacts information from the AddressBook database(s).

    Args:
        db_paths: Path or list of paths to AddressBook database files

    Returns:
        DataFrame containing contact information with columns:
        - normalized_contact_id: Normalized phone number or email address
        - first_name: Contact's first name
        - last_name: Contact's last name
        - state: State from postal address
        - city: City from postal address
    """
    if isinstance(db_paths, (str, Path)):
        db_paths = [db_paths]

    dfs = []
    for db_path in db_paths:
        conn = None
        try:
            conn = sqlite3.connect(db_path)
            fields = """
                r.Z_PK as primary_key,
                COALESCE(r.ZFIRSTNAME, '') as first_name,
                COALESCE(r.ZLASTNAME, '') as last_name,
                COALESCE(a.ZSTATE, '') as state,
                COALESCE(a.ZCITY, '') as city,
                r.ZCREATIONDATE as creation_date,
                r.ZMODIFICATIONDATE as modification_date
                FROM ZABCDRECORD as r
                LEFT JOIN ZABCDPOSTALADDRESS as a on r.Z_PK = a.ZOWNER
            """
            query = f"""
            SELECT 
                COALESCE(p.ZFULLNUMBER, '') as contact_id,
                {fields}
            LEFT JOIN ZABCDPHONENUMBER as p on r.Z_PK = p.ZOWNER

            UNION ALL

            SELECT 
                COALESCE(e.ZADDRESS, '') as contact_id,
                {fields}
            LEFT JOIN ZABCDEMAILADDRESS as e on r.Z_PK = e.ZOWNER
            """
            df = pl.read_database(query=query, connection=conn)
            dfs.append(df)
        except sqlite3.OperationalError as e:
            print(f"Error reading from {db_path}: {e}")
        except Exception as e:
            print(f"Unexpected error processing {db_path}: {e}")
        finally:
            if conn:
                conn.close()

    if dfs:
        return (
            pl.concat(dfs)
            .unique()
            .filter((pl.col("contact_id").str.len_chars() > 3))
            .with_columns(
                pl.col("contact_id")
                .map_elements(normalize_id, return_dtype=pl.Utf8)
                .alias("normalized_contact_id"),
                pl.col("creation_date")
                .map_elements(coredata_to_datetime)
                .dt.strftime("%Y-%m-%d %H:%M:%S")
                .alias("creation_date"),
                pl.col("modification_date")
                .map_elements(coredata_to_datetime)
                .dt.strftime("%Y-%m-%d %H:%M:%S")
                .alias("modification_date"),
            )
        )
    return pl.DataFrame()
