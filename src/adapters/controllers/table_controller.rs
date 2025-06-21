/*! This file contains the TableController implementation. */

use crate::application::repositories::table_repository::TableRepository;
use crate::domain::table::Table;
use crate::domain::two_column::TwoColumn;

/// Controller for handling table-related operations
pub struct TableController<T: TableRepository> {
    table_repository: T,
}

impl<T: TableRepository> TableController<T> {
    /// Create a new TableController with the given repository
    pub fn new(table_repository: T) -> Self {
        Self { table_repository }
    }

    /// Get a table by its name
    pub fn get_table(&self, database_name: &str, table_name: &str) -> Result<Table, String> {
        self.table_repository.get_table_by_name(database_name, table_name)
    }

    /// Get all tables in a database
    pub fn get_all_tables(&self, database_name: &str) -> Result<Vec<Table>, String> {
        self.table_repository.get_all_tables(database_name)
    }

    /// Get all empty tables in a database
    pub fn get_empty_tables(&self, database_name: &str) -> Result<Vec<Table>, String> {
        self.table_repository.get_empty_tables(database_name)
    }

    /// Get all tables without foreign keys
    pub fn get_clean_tables(&self, database_name: &str) -> Result<Vec<Table>, String> {
        self.table_repository.get_clean_tables(database_name)
    }

    /// Get all self-referencing tables
    pub fn get_self_referencing_tables(&self, database_name: &str) -> Result<Vec<Table>, String> {
        self.table_repository.get_self_referencing_tables(database_name)
    }

    /// Update a table in the database
    pub fn update_table(&self, table: &Table) -> Result<(), String> {
        self.table_repository.update_table(table)
    }

    /// Insert a new table into the database
    pub fn insert_table(&self, table: &Table) -> Result<(), String> {
        self.table_repository.insert_table(table)
    }

    /// Check if a table exists
    pub fn table_exists(&self, table: &Table) -> Result<bool, String> {
        self.table_repository.table_exists(table)
    }

    /// Get columns of a table
    pub fn get_columns(&self, database_name: &str, table_name: &str) -> Result<Vec<TwoColumn>, String> {
        self.table_repository.get_columns(database_name, table_name)
    }

    /// Get rows of a table
    pub fn get_rows(&self, database_name: &str, table_name: &str) -> Result<Vec<Vec<String>>, String> {
        self.table_repository.get_rows(database_name, table_name)
    }

    /// Execute a query on a database
    pub fn execute_query(&self, database_name: &str, query: &str) -> Result<(), String> {
        self.table_repository.execute_query(database_name, query)
    }

    /// Move data from one table to another
    pub fn move_table_data(&self, source_db: &str, source_table: &str, target_db: &str, target_table: &str) -> Result<(), String> {
        // This is where we would call the move_table_data use case
        // In a real implementation, we would create a MoveTableDataUseCase and call it
        
        // For now, we'll just simulate the call
        let source = self.table_repository.get_table_by_name(source_db, source_table)?;
        let target = self.table_repository.get_table_by_name(target_db, target_table)?;
        
        // In a real implementation, we would pass the repositories to the use case
        // and let it handle the business logic
        Ok(())
    }
}