/*! This file contains the MoveTableData use case service. */

use std::rc::Rc;
use log::info;
use crate::application::repositories::table_repository::TableRepository;
use crate::application::repositories::knowledge_repository::KnowledgeRepository;

/// Service for moving data from one table to another
pub struct MoveTableDataService<T: TableRepository, K: KnowledgeRepository> {
    table_repository: Rc<T>,
    knowledge_repository: Rc<K>,
}

impl<T: TableRepository, K: KnowledgeRepository> MoveTableDataService<T, K> {
    /// Create a new MoveTableDataService
    pub fn new(table_repository: Rc<T>, knowledge_repository: Rc<K>) -> Self {
        Self {
            table_repository,
            knowledge_repository,
        }
    }

    /// Move data from one table to another
    pub fn move_table_data(&self, source_database: &str, target_database: &str, table_name: &str) -> Result<(), String> {
        // Get source and target rows
        let source_rows = self.table_repository.get_rows(source_database, table_name)?;
        let target_rows = self.table_repository.get_rows(target_database, table_name)?;

        // Case: Both source and target databases are empty
        if source_rows.is_empty() && target_rows.is_empty() {
            self.knowledge_repository.update_export_status(table_name, true)?;
            info!("Both source and target databases are empty");
            return Ok(());
        }

        // Case: Data has been extracted
        if !target_rows.is_empty() && !source_rows.is_empty() && source_rows.len() == target_rows.len() {
            self.knowledge_repository.update_export_status(table_name, true)?;
            info!("Data has been extracted from source database");
            return Ok(());
        }

        // Check if table exists in target database
        let table = self.table_repository.get_table_by_name(source_database, table_name)?;
        if !self.table_repository.table_exists(&table)? {
            self.knowledge_repository.update_export_status(table_name, true)?;
            info!("Table: {} does not exist in the target database", table_name);
            return Ok(());
        }

        // Get columns from source and target databases
        let source_columns = self.table_repository.get_columns(source_database, table_name)?;
        let target_columns = self.table_repository.get_columns(target_database, table_name)?;

        // Find common columns
        let common_columns: Vec<_> = target_columns.iter()
            .filter(|c| source_columns.iter().any(|c2| c2.name == c.name))
            .collect();

        // Build insert queries
        for row in source_rows {
            // Build insert query
            let query = self.build_insert_query(table_name, &common_columns, &row)?;
            
            // Execute query
            match self.table_repository.execute_query(target_database, &query) {
                Ok(_) => {
                    info!("Query executed successfully");
                    self.knowledge_repository.update_export_status(table_name, true)?;
                },
                Err(err) => {
                    // Handle foreign key constraint errors by recursively moving referenced tables
                    if let Some(referenced_table) = self.extract_referenced_table_from_error(&err) {
                        self.move_table_data(source_database, target_database, &referenced_table)?;
                        // Retry the original query
                        self.table_repository.execute_query(target_database, &query)?;
                    } else {
                        return Err(err);
                    }
                }
            }
        }

        Ok(())
    }

    // Helper method to build an insert query
    fn build_insert_query(&self, table_name: &str, columns: &[&crate::domain::two_column::TwoColumn], row: &[String]) -> Result<String, String> {
        let columns_str = columns.iter().map(|c| c.name.clone()).collect::<Vec<_>>().join(", ");
        
        // Map row values to column values
        let mut values = Vec::new();
        for (i, column) in columns.iter().enumerate() {
            let value = if i < row.len() {
                if row[i].is_empty() {
                    "NULL".to_string()
                } else {
                    format!("'{}'", row[i])
                }
            } else {
                "NULL".to_string()
            };
            values.push(value);
        }
        
        let values_str = values.join(", ");
        Ok(format!("INSERT INTO {} ({}) VALUES ({})", table_name, columns_str, values_str))
    }

    // Helper method to extract referenced table from error message
    fn extract_referenced_table_from_error(&self, error: &str) -> Option<String> {
        // This is a simplified implementation
        // In a real application, you would need to parse the error message to extract the referenced table
        // For example: "Key (document_id)=(55) is not present in table \"materialflowresources_document\"."
        
        if let Some(start_idx) = error.find("table \"") {
            if let Some(end_idx) = error[start_idx..].find("\".") {
                return Some(error[start_idx + 7..start_idx + end_idx].to_string());
            }
        }
        
        None
    }
}