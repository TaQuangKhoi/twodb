/*! This file contains the TableRepository trait. */

use crate::domain::table::Table;
use crate::domain::two_column::TwoColumn;

/// Repository trait for accessing and manipulating tables
pub trait TableRepository {
    /// Get a table by its name
    fn get_table_by_name(&self, database_name: &str, table_name: &str) -> Result<Table, String>;
    
    /// Get all tables in a database
    fn get_all_tables(&self, database_name: &str) -> Result<Vec<Table>, String>;
    
    /// Get all empty tables in a database
    fn get_empty_tables(&self, database_name: &str) -> Result<Vec<Table>, String>;
    
    /// Get all tables without foreign keys
    fn get_clean_tables(&self, database_name: &str) -> Result<Vec<Table>, String>;
    
    /// Get all self-referencing tables
    fn get_self_referencing_tables(&self, database_name: &str) -> Result<Vec<Table>, String>;
    
    /// Update a table in the database
    fn update_table(&self, table: &Table) -> Result<(), String>;
    
    /// Insert a new table into the database
    fn insert_table(&self, table: &Table) -> Result<(), String>;
    
    /// Check if a table exists
    fn table_exists(&self, table: &Table) -> Result<bool, String>;
    
    /// Get columns of a table
    fn get_columns(&self, database_name: &str, table_name: &str) -> Result<Vec<TwoColumn>, String>;
    
    /// Get rows of a table
    fn get_rows(&self, database_name: &str, table_name: &str) -> Result<Vec<Vec<String>>, String>;
    
    /// Execute a query on a database
    fn execute_query(&self, database_name: &str, query: &str) -> Result<(), String>;
}