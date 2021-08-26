use crate::normal_game::cell;
use crate::normal_game::setting;
use std::rc::Rc;

#[derive(Debug)]
pub struct Group {
    pub cells: cell::Cells,
}

pub fn create_groups(cells: &cell::Cells, setting: &setting::GameSetting) -> Vec<Rc<Group>> {
    let vg = create_vertical_groups(&cells, &setting);
    let hg = create_horizontal_groups(&cells, &setting);
    let gg = create_block_groups(&cells, &setting);
    vec![vg, hg, gg]
        .iter()
        .flatten()
        .map(|g| g.clone())
        .collect()
}

fn create_vertical_groups(cells: &cell::Cells, setting: &setting::GameSetting) -> Vec<Rc<Group>> {
    let cols: Vec<u8> = (0..setting.side_size()).collect();
    cols.iter()
        .map(|n| {
            Rc::new(Group {
                cells: cells.filter_by_column(*n),
            })
        })
        .collect()
}

fn create_horizontal_groups(cells: &cell::Cells, setting: &setting::GameSetting) -> Vec<Rc<Group>> {
    let rows: Vec<u8> = (0..setting.side_size()).collect();
    rows.iter()
        .map(|n| {
            Rc::new(Group {
                cells: cells.filter_by_row(*n),
            })
        })
        .collect()
}

fn create_block_groups(cells: &cell::Cells, setting: &setting::GameSetting) -> Vec<Rc<Group>> {
    let block_start_positions = create_block_start_positions(setting);
    let mut vec: Vec<Rc<Group>> = vec![];
    for start_pos in block_start_positions {
        let mut one_group_cells: Vec<Rc<cell::Cell>> = vec![];
        for row in (0..setting.block_height).collect::<Vec<u8>>() {
            for col in (0..setting.block_width).collect::<Vec<u8>>() {
                let pos = start_pos.add_row(row).add_col(col);
                one_group_cells.push(cells.find_by_position(&pos).unwrap().clone());
            }
        }
        vec.push(Rc::new(Group {
            cells: cell::Cells::new(one_group_cells),
        }))
    }
    return vec;
}

fn create_block_start_positions(setting: &setting::GameSetting) -> Vec<cell::Position> {
    let side_num_list: Vec<u8> = (0..setting.side_size()).collect();
    let block_start_rows: Vec<u8> = side_num_list
        .iter()
        .filter(|n| *n % setting.block_height == 0)
        .map(|n| *n)
        .collect();
    let block_start_cols: Vec<u8> = side_num_list
        .iter()
        .filter(|n| *n % setting.block_width == 0)
        .map(|n| *n)
        .collect();
    block_start_rows
        .iter()
        .map(|row| {
            block_start_cols
                .iter()
                .map(|col| cell::Position::new(*row, *col))
                .collect::<Vec<cell::Position>>()
        })
        .flatten()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_create_vertical_groups() {
        let setting = setting::GameSetting {
            block_height: 2,
            block_width: 3,
        };
        let vg = create_vertical_groups(&cell::create_cells(&setting), &setting);
        assert_eq!(vg.len(), 6);
        assert_eq!(vg[0].cells.len(), 6);
        assert_eq!(
            vg[0].cells.find_by_index(0).unwrap().pos(),
            cell::Position::new(0, 0)
        );
        assert_eq!(
            vg[0].cells.find_by_index(5).unwrap().pos(),
            cell::Position::new(5, 0)
        );
        assert_eq!(
            vg[5].cells.find_by_index(0).unwrap().pos(),
            cell::Position::new(0, 5)
        );
        assert_eq!(
            vg[5].cells.find_by_index(5).unwrap().pos(),
            cell::Position::new(5, 5)
        );
    }
    #[test]
    fn test_create_horizontal_groups() {
        let setting = setting::GameSetting {
            block_height: 2,
            block_width: 3,
        };
        let hg = create_horizontal_groups(&cell::create_cells(&setting), &setting);
        assert_eq!(hg.len(), 6);
        assert_eq!(hg[0].cells.len(), 6);
        assert_eq!(
            hg[0].cells.find_by_index(0).unwrap().pos(),
            cell::Position::new(0, 0)
        );
        assert_eq!(
            hg[0].cells.find_by_index(5).unwrap().pos(),
            cell::Position::new(0, 5)
        );
        assert_eq!(
            hg[5].cells.find_by_index(0).unwrap().pos(),
            cell::Position::new(5, 0)
        );
        assert_eq!(
            hg[5].cells.find_by_index(5).unwrap().pos(),
            cell::Position::new(5, 5)
        );
    }
    #[test]
    fn test_create_block_start_positions() {
        let setting = setting::GameSetting {
            block_height: 2,
            block_width: 3,
        };
        let block_start_positions = create_block_start_positions(&setting);
        assert_eq!(
            block_start_positions,
            vec![
                cell::Position::new(0, 0),
                cell::Position::new(0, 3),
                cell::Position::new(2, 0),
                cell::Position::new(2, 3),
                cell::Position::new(4, 0),
                cell::Position::new(4, 3)
            ]
        )
    }
    mod test_create_block_groups {
        use super::*;
        #[test]
        fn block_group_count() {
            let setting = setting::GameSetting {
                block_height: 2,
                block_width: 3,
            };
            let groups = create_block_groups(&cell::create_cells(&setting), &setting);
            assert_eq!(groups.len(), 6);
        }
        #[test]
        fn block_group_cell_count() {
            let setting = setting::GameSetting {
                block_height: 2,
                block_width: 3,
            };
            let groups = create_block_groups(&cell::create_cells(&setting), &setting);
            assert!(groups.iter().all(|g| g.cells.len() == 6));
        }
        #[test]
        fn first_block_group_cells() {
            let setting = setting::GameSetting {
                block_height: 2,
                block_width: 3,
            };
            let groups = create_block_groups(&cell::create_cells(&setting), &setting);
            assert_eq!(
                groups[0].cells.positions(),
                vec![
                    cell::Position::new(0, 0),
                    cell::Position::new(0, 1),
                    cell::Position::new(0, 2),
                    cell::Position::new(1, 0),
                    cell::Position::new(1, 1),
                    cell::Position::new(1, 2),
                ]
            );
        }
        #[test]
        fn last_block_group_cells() {
            let setting = setting::GameSetting {
                block_height: 2,
                block_width: 3,
            };
            let groups = create_block_groups(&cell::create_cells(&setting), &setting);
            assert_eq!(
                groups[5].cells.positions(),
                vec![
                    cell::Position::new(4, 3),
                    cell::Position::new(4, 4),
                    cell::Position::new(4, 5),
                    cell::Position::new(5, 3),
                    cell::Position::new(5, 4),
                    cell::Position::new(5, 5),
                ]
            );
        }
    }
}
