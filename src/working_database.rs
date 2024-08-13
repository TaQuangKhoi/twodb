use std::time::SystemTime;

pub fn get_cells(row: &postgres::Row) -> Vec<String> {
    let columns = row.columns();
    let cells: Vec<String> = columns.iter().map(|column| {
        let name = column.name();
        let type_ = column.type_();
        match type_.name() {
            "int8" => {
                let value: Option<i64> = row.try_get(name).unwrap_or(None);
                format!("{}: {}", name, value.unwrap_or(0))
            },
            "int4" => {
                let value: Option<i32> = row.try_get(name).unwrap_or(None);
                format!("{}: {}", name, value.unwrap_or(0))
            },

            "varchar" => {
                let value: Option<&str> = row.try_get(name).unwrap_or(None);
                format!("{}: {}", name, value.unwrap_or("None"))
            }
            "bool" => {
                let value: Option<bool> = row.try_get(name).unwrap_or(None);
                format!("{}: {}", name, value.unwrap_or(false))
            }
            "timestamp" => {
                let value: Option<SystemTime> = row.try_get(name).unwrap_or(None);
                format!("{}: {}", name, value.unwrap_or(SystemTime::UNIX_EPOCH)
                    .duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs())
            }
            _ => {
                println!("Unknown type: {:?}", type_.name());
                format!("{}: {}", name, "Unknown")
            }
        }
    }).collect();
    cells
}