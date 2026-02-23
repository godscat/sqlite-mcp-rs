## ADDED Requirements

### Requirement: Auxiliary tables for comments storage
The system SHALL create two auxiliary tables to store table and column comments if they do not exist.

#### Scenario: Create auxiliary tables on first use
- **WHEN** get_table_schema is called for the first time
- **THEN** system creates _table_comment table with columns: id, table_name, table_desc, ctime, utime
- **AND** system creates _table_column_comment table with columns: id, table_name, column_name, column_desc
- **AND** _table_comment has unique constraint on table_name
- **AND** _table_column_comment has unique index on (table_name, column_name)

### Requirement: Default comment initialization
The system SHALL initialize default comments for tables and columns when they are first queried.

#### Scenario: Initialize table default comment
- **WHEN** get_table_schema is called for a table that has no entry in _table_comment
- **THEN** system inserts a record into _table_comment with table_name equal to the table name
- **AND** table_desc is set to the table name (as default description)

#### Scenario: Initialize column default comments
- **WHEN** get_table_schema is called for a table that has no entries in _table_column_comment
- **THEN** system inserts records into _table_column_comment for each column
- **AND** each record contains: table_name, column_name, and column_desc set to the column name (as default description)

### Requirement: Comment query from auxiliary tables
The system SHALL query comments from auxiliary tables when returning table schema.

#### Scenario: Query existing table comment
- **WHEN** get_table_schema is called for a table with existing entry in _table_comment
- **THEN** system retrieves table_desc from _table_comment where table_name matches

#### Scenario: Query existing column comments
- **WHEN** get_table_schema is called for a table with existing entries in _table_column_comment
- **THEN** system retrieves column_desc for each column from _table_column_comment where table_name and column_name match