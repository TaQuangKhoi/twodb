/*! This file contains the KnowledgePresenter implementation. */

use crate::domain::table::Table;

/// View model for knowledge about a table
pub struct TableKnowledgeViewModel {
    pub id: i64,
    pub name: String,
    pub database: String,
    pub row_count: i64,
    pub is_self_referencing: bool,
    pub self_referencing_column: String,
    pub is_exported: bool,
    pub export_order: i64,
    pub status: String,
}

/// View model for a summary of table knowledge
pub struct KnowledgeSummaryViewModel {
    pub total_tables: usize,
    pub exported_tables: usize,
    pub non_exported_tables: usize,
    pub self_referencing_tables: usize,
    pub empty_tables: usize,
}

/// Presenter for formatting knowledge-related data for the UI
pub struct KnowledgePresenter;

impl KnowledgePresenter {
    /// Format knowledge about a table for display in the UI
    pub fn present_table_knowledge(table: &Table) -> TableKnowledgeViewModel {
        TableKnowledgeViewModel {
            id: table.id,
            name: table.name.clone(),
            database: table.database.clone(),
            row_count: table.row_count,
            is_self_referencing: table.is_self_referencing,
            self_referencing_column: table.self_referencing_column.clone(),
            is_exported: table.is_exported,
            export_order: table.export_order,
            status: if table.is_exported { "Exported".to_string() } else { "Not Exported".to_string() },
        }
    }

    /// Format a list of tables for display in the UI
    pub fn present_tables_knowledge(tables: &[Table]) -> Vec<TableKnowledgeViewModel> {
        tables.iter().map(|table| Self::present_table_knowledge(table)).collect()
    }

    /// Create a summary of table knowledge for display in the UI
    pub fn present_knowledge_summary(tables: &[Table]) -> KnowledgeSummaryViewModel {
        let total_tables = tables.len();
        let exported_tables = tables.iter().filter(|table| table.is_exported).count();
        let non_exported_tables = total_tables - exported_tables;
        let self_referencing_tables = tables.iter().filter(|table| table.is_self_referencing).count();
        let empty_tables = tables.iter().filter(|table| table.row_count == 0).count();

        KnowledgeSummaryViewModel {
            total_tables,
            exported_tables,
            non_exported_tables,
            self_referencing_tables,
            empty_tables,
        }
    }
}