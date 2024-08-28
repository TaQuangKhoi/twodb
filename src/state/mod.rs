
#[derive(serde::Deserialize, serde::Serialize)]
pub struct WindowsState {
    pub window_move_one_table_open: bool,
    pub window_move_all_tables_open: bool,
    pub window_reset_open: bool,
}