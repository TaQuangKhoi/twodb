/*! This file contains the Table entity. */

#[derive(Debug)]
pub struct Table {
    pub id: i64,
    pub name: String,
    pub table_type: TableType,
    pub export_complexity_type: ExportComplexityType,
    pub database: String,
    pub export_order: i64,
    pub is_self_referencing: bool,
    pub self_referencing_column: String,
    pub row_count: i64,
    pub is_exported: bool,
}

impl Default for Table {
    fn default() -> Self {
        Self {
            id: 0,
            name: String::from(""),
            table_type: TableType::BaseTable,
            export_complexity_type: ExportComplexityType::SIMPLE,
            database: String::from(""),
            export_order: 0,
            is_self_referencing: false,
            self_referencing_column: String::from(""),
            row_count: 0,
            is_exported: false,
        }
    }
}

impl Table {
    pub fn get_table_name_as_str(&self) -> String {
        self.name.clone()
    }

    pub fn increase_export_order(&mut self) {
        self.export_order += 1;
    }
}

pub const BASE_TABLE_STR: &str = "BASE TABLE";

#[derive(Debug)]
pub enum TableType {
    BaseTable,
    VIEW,
}

impl TableType {
    pub fn name(&self) -> &str {
        match self {
            TableType::BaseTable => BASE_TABLE_STR,
            TableType::VIEW => "VIEW",
        }
    }
}

#[derive(Debug)]
pub enum ExportComplexityType {
    SIMPLE,
    COMPLEX,
}

impl ExportComplexityType {
    pub fn name(&self) -> &str {
        match self {
            ExportComplexityType::SIMPLE => "SIMPLE",
            ExportComplexityType::COMPLEX => "COMPLEX",
        }
    }
}