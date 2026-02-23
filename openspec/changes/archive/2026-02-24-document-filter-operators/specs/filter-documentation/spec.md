## ADDED Requirements

### Requirement: Filter JSON Schema must list all supported operators
The system SHALL include all comparison operators in the query_records tool's filters parameter inputSchema.

#### Scenario: JSON Schema includes comparison operators
- **WHEN** MCP client requests tools/list
- **THEN** query_records filters inputSchema documents all supported operators: $eq, $gt, $gte, $lt, $lte, $ne, $like

### Requirement: Filter JSON Schema must provide complete structure
The system SHALL provide a complete JSON Schema structure for the filters parameter that shows the nested operator-value format.

#### Scenario: Filters schema shows operator structure
- **WHEN** MCP client inspects query_records tool definition
- **THEN** the filters inputSchema shows the nested structure: field -> operator -> value

### Requirement: Filter JSON Schema must explain logical relationship
The system SHALL document the default logical relationship (AND) when multiple filter conditions are specified in the schema description.

#### Scenario: Multiple conditions use AND logic
- **WHEN** user specifies multiple filter conditions
- **THEN** the JSON Schema description explains that conditions are combined using AND logic
- **AND** all conditions must be satisfied for a record to match

### Requirement: README must provide filter usage guide
The system SHALL provide a centralized filter parameter usage guide in the README with practical examples.

#### Scenario: README contains filter guide
- **WHEN** user reads the project README
- **THEN** a "Filter Parameter Usage" section exists
- **AND** it explains supported operators, syntax, and examples

#### Scenario: Filter guide shows JSON examples
- **WHEN** user reads the filter usage guide
- **THEN** it includes JSON examples for each operator type
- **AND** it includes complex examples with multiple conditions
