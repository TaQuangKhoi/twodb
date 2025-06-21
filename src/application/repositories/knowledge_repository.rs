/*! This file contains the KnowledgeRepository trait. */

use crate::domain::table::Table;

/// Repository trait for accessing and updating knowledge about tables
pub trait KnowledgeRepository {
    /// Get knowledge about a table
    fn get_table_knowledge(&self, database_name: &str, table_name: &str) -> Result<Table, String>;
    
    /// Update knowledge about a table
    fn update_table_knowledge(&self, table: &Table) -> Result<(), String>;
    
    /// Update knowledge about table row count
    fn update_row_count(&self, table: &mut Table) -> Result<(), String>;
    
    /// Update knowledge about table self-referencing status
    fn update_self_referencing(&self, table: &mut Table, database_name: &str) -> Result<(), String>;
    
    /// Update knowledge about table export status
    fn update_export_status(&self, table_name: &str, is_exported: bool) -> Result<(), String>;
    
    /// Get all tables that have been exported
    fn get_exported_tables(&self) -> Result<Vec<Table>, String>;
    
    /// Get all tables that have not been exported
    fn get_non_exported_tables(&self) -> Result<Vec<Table>, String>;
}