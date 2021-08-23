use crate::normal_game::cell;
use crate::normal_game::setting;
use std::rc::Rc;

pub struct Group {
    cells: Vec<Rc<cell::Cell>>,
}

pub fn create_groups(cells: &cell::Cells, setting: &setting::GameSetting) -> Vec<Group> {
    let vg: Vec<Group> = create_vertical_groups(&cells, &setting);
    // let c2: Vec<Rc<cell::Cell>> = cells.iter().map(|x| x.clone()).collect();
    vg
    // vec![vg, Group { cells: c2 }]
}

fn create_vertical_groups(cells: &cell::Cells, setting: &setting::GameSetting) -> Vec<Group> {
    let rows: Vec<u8> = (0..setting.side_size()).collect();
    rows.iter()
        .map(|n| Group {
            cells: cells.filter_by_row(*n),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create_groups_test() {
        assert_eq!(true, true);
    }
    #[test]
    fn test_create_vertical_groups() {
        let setting = setting::GameSetting {
            block_height: 2,
            block_width: 3,
        };
        let vg = create_vertical_groups(&cell::Cells::create_cells(&setting), &setting);
        assert_eq!(vg.len(), 6);
    }
}
