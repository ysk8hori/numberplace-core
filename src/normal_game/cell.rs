use crate::normal_game::setting;

pub struct Cell {}

pub fn create_cells(setting: setting::GameSetting) -> Vec<Cell> {
    let mut ret = Vec::new();
    for _ in 1..=setting.side_size() {
        for _ in 1..=setting.side_size() {
            ret.push(Cell {});
        }
    }
    ret
}
