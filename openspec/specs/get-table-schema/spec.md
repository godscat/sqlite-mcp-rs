## MODIFIED Requirements

### Requirement: Table schema JSON structure includes comments
The system SHALL return table schema JSON with comment information for both table and columns.

#### Scenario: Return table schema with table and column comments
- **WHEN** get_table_schema is called with a valid table name
- **THEN** response JSON includes desc field at table level containing table comment
- **AND** response JSON includes desc field for each column containing column comment
- **AND** desc field values are retrieved from _table_comment and _table_column_comment tables
- **AND** desc field defaults to table name or column name if no custom comment is set

#### Scenario: Table schema JSON response format with comments
- **WHEN** get_table_schema is called for table "products" with comments
- **THEN** JSON response structure is:
```json
{
  "name": "products",
  "desc": "产品表",
  "columns": [
    {
      "name": "id",
      "desc": "主键",
      "data_type": "INTEGER",
      "not_null": false,
      "default_value": null,
      "is_primary_key": true
    },
    {
      "name": "name",
      "desc": "产品名称",
      "data_type": "TEXT",
      "not_null": true,
      "default_value": null,
      "is_primary_key": false
    }
  ],
  "primary_key": "id"
}
```