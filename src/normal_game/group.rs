use crate::normal_game::cell;
use crate::normal_game::setting;
use std::rc::Rc;

#[derive(Debug)]
pub struct Group {
    pub cells: Vec<Rc<cell::Cell>>,
}

pub fn create_groups(cells: &cell::Cells, setting: &setting::GameSetting) -> Vec<Group> {
    let vg: Vec<Group> = create_vertical_groups(&cells, &setting);
    let hg: Vec<Group> = create_horizontal_groups(&cells, &setting);
    vg
    // vec![vg, Group { cells: c2 }]
}

fn create_vertical_groups(cells: &cell::Cells, setting: &setting::GameSetting) -> Vec<Group> {
    let cols: Vec<u8> = (1..=setting.side_size()).collect();
    cols.iter()
        .map(|n| Group {
            cells: cells.filter_by_column(*n),
        })
        .collect()
}

fn create_horizontal_groups(cells: &cell::Cells, setting: &setting::GameSetting) -> Vec<Group> {
    let rows: Vec<u8> = (1..=setting.side_size()).collect();
    rows.iter()
        .map(|n| Group {
            cells: cells.filter_by_row(*n),
        })
        .collect()
}

// fn create_block_groups(cells: &cell::Cells, setting: &setting::GameSetting) -> Vec<Group> {
//     let block_rows_list: Vec<u8> = ()
// }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_create_vertical_groups() {
        let setting = setting::GameSetting {
            block_height: 2,
            block_width: 3,
        };
        let vg = create_vertical_groups(&cell::Cells::create_cells(&setting), &setting);
        assert_eq!(vg.len(), 6);
        assert_eq!(vg[0].cells.len(), 6);
        assert_eq!(*vg[0].cells[0].pos(), cell::Position::new(1, 1));
        assert_eq!(*vg[0].cells[5].pos(), cell::Position::new(6, 1));
        assert_eq!(*vg[5].cells[0].pos(), cell::Position::new(1, 6));
        assert_eq!(*vg[5].cells[5].pos(), cell::Position::new(6, 6));
    }
    #[test]
    fn test_create_horizontal_groups() {
        let setting = setting::GameSetting {
            block_height: 2,
            block_width: 3,
        };
        let hg = create_horizontal_groups(&cell::Cells::create_cells(&setting), &setting);
        assert_eq!(hg.len(), 6);
        assert_eq!(hg[0].cells.len(), 6);
        assert_eq!(*hg[0].cells[0].pos(), cell::Position::new(1, 1));
        assert_eq!(*hg[0].cells[5].pos(), cell::Position::new(1, 6));
        assert_eq!(*hg[5].cells[0].pos(), cell::Position::new(6, 1));
        assert_eq!(*hg[5].cells[5].pos(), cell::Position::new(6, 6));
    }
}
