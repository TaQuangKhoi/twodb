/*! This file contains the TablePresenter implementation. */

use crate::domain::table::Table;
use crate::domain::two_column::TwoColumn;

/// View model for a table
pub struct TableViewModel {
    pub id: i64,
    pub name: String,
    pub table_type: String,
    pub export_complexity_type: String,
    pub database: String,
    pub export_order: i64,
    pub is_self_referencing: bool,
    pub self_referencing_column: String,
    pub row_count: i64,
    pub is_exported: bool,
    pub status: String,
}

/// View model for a column
pub struct ColumnViewModel {
    pub name: String,
    pub data_type: String,
}

/// View model for a row
pub struct RowViewModel {
    pub values: Vec<String>,
}

/// Presenter for formatting table-related data for the UI
pub struct TablePresenter;

impl TablePresenter {
    /// Format a table entity for display in the UI
    pub fn present_table(table: &Table) -> TableViewModel {
        TableViewModel {
            id: table.id,
            name: table.name.clone(),
            table_type: table.table_type.name().to_string(),
            export_complexity_type: table.export_complexity_type.name().to_string(),
            database: table.database.clone(),
            export_order: table.export_order,
            is_self_referencing: table.is_self_referencing,
            self_referencing_column: table.self_referencing_column.clone(),
            row_count: table.row_count,
            is_exported: table.is_exported,
            status: if table.is_exported { "Exported".to_string() } else { "Not Exported".to_string() },
        }
    }

    /// Format a list of table entities for display in the UI
    pub fn present_tables(tables: &[Table]) -> Vec<TableViewModel> {
        tables.iter().map(|table| Self::present_table(table)).collect()
    }

    /// Format a column entity for display in the UI
    pub fn present_column(column: &TwoColumn) -> ColumnViewModel {
        ColumnViewModel {
            name: column.name.clone(),
            data_type: column.data_type.clone(),
        }
    }

    /// Format a list of column entities for display in the UI
    pub fn present_columns(columns: &[TwoColumn]) -> Vec<ColumnViewModel> {
        columns.iter().map(|column| Self::present_column(column)).collect()
    }

    /// Format a row for display in the UI
    pub fn present_row(row: &[String]) -> RowViewModel {
        RowViewModel {
            values: row.to_vec(),
        }
    }

    /// Format a list of rows for display in the UI
    pub fn present_rows(rows: &[Vec<String>]) -> Vec<RowViewModel> {
        rows.iter().map(|row| Self::present_row(row)).collect()
    }
}