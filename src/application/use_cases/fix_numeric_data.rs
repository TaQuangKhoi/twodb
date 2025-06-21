/*! This file contains the FixNumericData use case service. */

use std::rc::Rc;
use log::info;
use crate::application::repositories::table_repository::TableRepository;
use crate::application::repositories::knowledge_repository::KnowledgeRepository;

/// Service for fixing numeric data errors
pub struct FixNumericDataService<T: TableRepository, K: KnowledgeRepository> {
    table_repository: Rc<T>,
    knowledge_repository: Rc<K>,
}

impl<T: TableRepository, K: KnowledgeRepository> FixNumericDataService<T, K> {
    /// Create a new FixNumericDataService
    pub fn new(table_repository: Rc<T>, knowledge_repository: Rc<K>) -> Self {
        Self {
            table_repository,
            knowledge_repository,
        }
    }

    /// Fix numeric data errors in a table
    pub fn fix_numeric_data(&self, database_name: &str, table_name: &str) -> Result<(), String> {
        // Get columns of the table
        let columns = self.table_repository.get_columns(database_name, table_name)?;
        
        // Find numeric columns
        let numeric_columns: Vec<_> = columns.iter()
            .filter(|c| self.is_numeric_column(&c.data_type))
            .collect();
        
        if numeric_columns.is_empty() {
            info!("No numeric columns found in table {}", table_name);
            return Ok(());
        }
        
        info!("Found {} numeric columns in table {}", numeric_columns.len(), table_name);
        
        // For each numeric column, fix data errors
        for column in numeric_columns {
            // Build query to fix numeric data errors
            let query = self.build_fix_numeric_data_query(table_name, &column.name)?;
            
            // Execute query
            self.table_repository.execute_query(database_name, &query)?;
            
            info!("Fixed numeric data errors in column {} of table {}", column.name, table_name);
        }
        
        Ok(())
    }

    /// Fix numeric data errors in all tables
    pub fn fix_all_numeric_data(&self, database_name: &str) -> Result<(), String> {
        // Get all tables
        let tables = self.table_repository.get_all_tables(database_name)?;
        
        // For each table, fix numeric data errors
        for table in tables {
            self.fix_numeric_data(database_name, &table.name)?;
        }
        
        Ok(())
    }

    // Helper method to check if a column is numeric
    fn is_numeric_column(&self, data_type: &str) -> bool {
        let numeric_types = [
            "smallint", "integer", "bigint", "decimal", "numeric", 
            "real", "double precision", "smallserial", "serial", "bigserial"
        ];
        
        numeric_types.iter().any(|&t| data_type.contains(t))
    }

    // Helper method to build a query to fix numeric data errors
    fn build_fix_numeric_data_query(&self, table_name: &str, column_name: &str) -> Result<String, String> {
        // This is a simplified implementation
        // In a real application, you would need to build a more complex query based on the specific numeric data errors
        
        // Example: Replace commas with periods in numeric values
        let query = format!(
            "UPDATE {} SET {} = REPLACE({}, ',', '.') WHERE {} ~ ',' AND {} !~ '[^0-9,\\.]'",
            table_name, column_name, column_name, column_name, column_name
        );
        
        Ok(query)
    }
}