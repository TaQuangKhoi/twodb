/*! This file contains the UpdateEmptyTablesKnowledge use case service. */

use std::rc::Rc;
use log::info;
use crate::application::repositories::table_repository::TableRepository;
use crate::application::repositories::knowledge_repository::KnowledgeRepository;

/// Service for updating knowledge about empty tables
pub struct UpdateEmptyTablesKnowledgeService<T: TableRepository, K: KnowledgeRepository> {
    table_repository: Rc<T>,
    knowledge_repository: Rc<K>,
}

impl<T: TableRepository, K: KnowledgeRepository> UpdateEmptyTablesKnowledgeService<T, K> {
    /// Create a new UpdateEmptyTablesKnowledgeService
    pub fn new(table_repository: Rc<T>, knowledge_repository: Rc<K>) -> Self {
        Self {
            table_repository,
            knowledge_repository,
        }
    }

    /// Update knowledge about empty tables in a database
    pub fn update_empty_tables_knowledge(&self, database_name: &str) -> Result<(), String> {
        // Get all empty tables from the database
        let empty_tables = self.table_repository.get_empty_tables(database_name)?;
        
        info!("Found {} empty tables in database {}", empty_tables.len(), database_name);
        
        // Update knowledge about each empty table
        for mut table in empty_tables {
            // Check if table exists in knowledge base
            if self.table_repository.table_exists(&table)? {
                // Update row count
                self.knowledge_repository.update_row_count(&mut table)?;
            } else {
                // Insert new table
                self.table_repository.insert_table(&table)?;
            }
        }
        
        Ok(())
    }

    /// Update knowledge about all tables in a database
    pub fn update_all_tables_knowledge(&self, database_name: &str) -> Result<(), String> {
        // Get all tables from the database
        let all_tables = self.table_repository.get_all_tables(database_name)?;
        
        info!("Found {} tables in database {}", all_tables.len(), database_name);
        
        // Update knowledge about each table
        for mut table in all_tables {
            // Update self-referencing status
            self.knowledge_repository.update_self_referencing(&mut table, database_name)?;
            
            // Update row count
            self.knowledge_repository.update_row_count(&mut table)?;
            
            // Check if table exists in knowledge base
            if self.table_repository.table_exists(&table)? {
                // Update table
                self.knowledge_repository.update_table_knowledge(&table)?;
            } else {
                // Insert new table
                self.table_repository.insert_table(&table)?;
            }
        }
        
        Ok(())
    }

    /// Update knowledge about clean tables (tables without foreign keys)
    pub fn update_clean_tables_knowledge(&self, database_name: &str) -> Result<(), String> {
        // Get all clean tables from the database
        let clean_tables = self.table_repository.get_clean_tables(database_name)?;
        
        info!("Found {} clean tables in database {}", clean_tables.len(), database_name);
        
        // Update knowledge about each clean table
        for mut table in clean_tables {
            // Update row count
            self.knowledge_repository.update_row_count(&mut table)?;
            
            // Check if table exists in knowledge base
            if self.table_repository.table_exists(&table)? {
                // Skip if already exists
                continue;
            }
            
            // Insert new table
            self.table_repository.insert_table(&table)?;
        }
        
        Ok(())
    }

    /// Update knowledge about self-referencing tables
    pub fn update_self_referencing_tables_knowledge(&self, database_name: &str) -> Result<(), String> {
        // Get all self-referencing tables from the database
        let self_referencing_tables = self.table_repository.get_self_referencing_tables(database_name)?;
        
        info!("Found {} self-referencing tables in database {}", self_referencing_tables.len(), database_name);
        
        // Update knowledge about each self-referencing table
        for table in self_referencing_tables {
            // Check if table exists in knowledge base
            if self.table_repository.table_exists(&table)? {
                // Update table
                self.knowledge_repository.update_table_knowledge(&table)?;
            } else {
                // Insert new table
                self.table_repository.insert_table(&table)?;
            }
        }
        
        Ok(())
    }
}