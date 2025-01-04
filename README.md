# tage-nr-v1-050125
Database in Rust

# High level design 
High level design of the database and development plan for this database.:
## SQL

### Parser
- Takes a string
- Checks if the string is a valid SQL statement
- Returns a parse tree

### Analyser
- Takes a parse tree
- Checks if the parse tree is valid
- Returns a query plan

### Optimizer
- Takes a query plan
- Optimizes the query plan
- Returns an optimized query plan

## Execution Engine
- Takes an optimized query plan
- Executes the query plan
- Returns the result

## Network

## Storage
- Key-Value store for cache and Relational store for data
- Declares methods:
    - `get_by_key(key: String) -> Result<String, Error>`
    - `put(key: String, value: String) -> Result<(), Error>`
    - `delete(key: String) -> Result<(), Error>`
    - `scan(start_key: String, end_key: String) -> Result<Vec<String>, Error>`
## Transaction Manager
- ACID
## Configuration
- Configuration file for the database
