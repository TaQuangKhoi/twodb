/*! This file contains the KnowledgeController implementation. */

use crate::application::repositories::knowledge_repository::KnowledgeRepository;
use crate::application::use_cases::update_empty_tables_knowledge;
use crate::domain::table::Table;

/// Controller for handling knowledge-related operations
pub struct KnowledgeController<T: KnowledgeRepository> {
    knowledge_repository: T,
}

impl<T: KnowledgeRepository> KnowledgeController<T> {
    /// Create a new KnowledgeController with the given repository
    pub fn new(knowledge_repository: T) -> Self {
        Self { knowledge_repository }
    }

    /// Get knowledge about a table
    pub fn get_table_knowledge(&self, database_name: &str, table_name: &str) -> Result<Table, String> {
        self.knowledge_repository.get_table_knowledge(database_name, table_name)
    }

    /// Update knowledge about a table
    pub fn update_table_knowledge(&self, table: &Table) -> Result<(), String> {
        self.knowledge_repository.update_table_knowledge(table)
    }

    /// Update knowledge about table row count
    pub fn update_row_count(&self, table: &mut Table) -> Result<(), String> {
        self.knowledge_repository.update_row_count(table)
    }

    /// Update knowledge about table self-referencing status
    pub fn update_self_referencing(&self, table: &mut Table, database_name: &str) -> Result<(), String> {
        self.knowledge_repository.update_self_referencing(table, database_name)
    }

    /// Update knowledge about table export status
    pub fn update_export_status(&self, table_name: &str, is_exported: bool) -> Result<(), String> {
        self.knowledge_repository.update_export_status(table_name, is_exported)
    }

    /// Get all tables that have been exported
    pub fn get_exported_tables(&self) -> Result<Vec<Table>, String> {
        self.knowledge_repository.get_exported_tables()
    }

    /// Get all tables that have not been exported
    pub fn get_non_exported_tables(&self) -> Result<Vec<Table>, String> {
        self.knowledge_repository.get_non_exported_tables()
    }

    /// Update knowledge about empty tables
    pub fn update_empty_tables_knowledge(&self, database_name: &str) -> Result<(), String> {
        // This is where we would call the update_empty_tables_knowledge use case
        // In a real implementation, we would create an UpdateEmptyTablesKnowledgeUseCase and call it
        
        // For now, we'll just simulate the call
        let tables = self.knowledge_repository.get_non_exported_tables()?;
        
        // In a real implementation, we would pass the repositories to the use case
        // and let it handle the business logic
        Ok(())
    }
}