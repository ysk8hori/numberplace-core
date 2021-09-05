use crate::normal_game::cell;
use crate::normal_game::setting;
use std::cell::RefCell;
use std::rc::Rc;

// #[derive(Debug)]
pub struct Group {
    cells: Vec<Rc<RefCell<cell::Cell>>>,
    unanswerd_candidate: Vec<u8>,
}

impl Group {
    /// Get lonely, in the Group.
    ///
    /// Returns the answer_candidate and the position of the cell that is held by only one of the cells that belong to the group.
    ///
    /// Group に所属する Cell の うち 1 つの Cell のみが保有している answer_candidate とその Cell の Position を返却する。
    pub fn get_lonely(&self) -> Vec<(cell::Position, u8)> {
        let mut lonelies: Vec<(cell::Position, u8)> = vec![];
        for candidate in self.unanswerd_candidate.iter() {
            let asdf: Vec<cell::Position> = self
                .cells
                .iter()
                .filter(|c| c.borrow().has_answer_candidate(*candidate))
                .map(|c| c.borrow().pos())
                .collect();
            if asdf.len() == 1 {
                lonelies.push((asdf[0], *candidate));
            }
        }
        lonelies
    }

    /// Remove the specified answer from the unanswerd_candidate.
    pub fn remove_unanswerd_candidate(&mut self, answer: u8) {
        self.unanswerd_candidate = self
            .unanswerd_candidate
            .iter()
            .filter(|n| **n != answer)
            .map(|n| *n)
            .collect();
    }
}

pub fn create_groups(
    cells: &Vec<Rc<RefCell<cell::Cell>>>,
    setting: &setting::GameSetting,
) -> Vec<Rc<RefCell<Group>>> {
    let vg = create_vertical_groups(&cells, &setting);
    let hg = create_horizontal_groups(&cells, &setting);
    let gg = create_block_groups(&cells, &setting);
    vec![vg, hg, gg]
        .iter()
        .flatten()
        .map(|g| g.clone())
        .collect()
}

fn create_vertical_groups(
    cells: &Vec<Rc<RefCell<cell::Cell>>>,
    setting: &setting::GameSetting,
) -> Vec<Rc<RefCell<Group>>> {
    let x_pos: Vec<u8> = (0..setting.side_size()).collect();
    x_pos
        .iter()
        .map(|x| {
            Rc::new(RefCell::new(Group {
                cells: cells
                    .iter()
                    .filter(|c| c.borrow().pos().x() == *x)
                    .map(|c| c.clone())
                    .collect(),
                unanswerd_candidate: setting.answer_candidate().clone(),
            }))
        })
        .collect()
}

fn create_horizontal_groups(
    cells: &Vec<Rc<RefCell<cell::Cell>>>,
    setting: &setting::GameSetting,
) -> Vec<Rc<RefCell<Group>>> {
    let y_pos: Vec<u8> = (0..setting.side_size()).collect();
    y_pos
        .iter()
        .map(|y| {
            Rc::new(RefCell::new(Group {
                cells: cells
                    .iter()
                    .filter(|c| c.borrow().pos().y() == *y)
                    .map(|c| c.clone())
                    .collect(),
                unanswerd_candidate: setting.answer_candidate().clone(),
            }))
        })
        .collect()
}

fn create_block_groups(
    cells: &Vec<Rc<RefCell<cell::Cell>>>,
    setting: &setting::GameSetting,
) -> Vec<Rc<RefCell<Group>>> {
    let block_start_positions = create_block_start_positions(setting);
    let mut vec: Vec<Rc<RefCell<Group>>> = vec![];
    for start_pos in block_start_positions {
        let mut one_group_cells: Vec<Rc<RefCell<cell::Cell>>> = vec![];
        for y in (0..setting.block_height).collect::<Vec<u8>>() {
            for x in (0..setting.block_width).collect::<Vec<u8>>() {
                let pos = start_pos.move_y(y).move_x(x);
                one_group_cells.push(
                    cells
                        .iter()
                        .find(|c| c.borrow().pos() == pos)
                        .unwrap()
                        .clone(),
                );
            }
        }
        vec.push(Rc::new(RefCell::new(Group {
            cells: one_group_cells,
            unanswerd_candidate: setting.answer_candidate().clone(),
        })))
    }
    return vec;
}

fn create_block_start_positions(setting: &setting::GameSetting) -> Vec<cell::Position> {
    let side_num_list: Vec<u8> = (0..setting.side_size()).collect();
    let block_start_y_list: Vec<u8> = side_num_list
        .iter()
        .filter(|n| *n % setting.block_height == 0)
        .map(|n| *n)
        .collect();
    let block_start_x_list: Vec<u8> = side_num_list
        .iter()
        .filter(|n| *n % setting.block_width == 0)
        .map(|n| *n)
        .collect();
    block_start_y_list
        .iter()
        .map(|y| {
            block_start_x_list
                .iter()
                .map(|x| cell::Position::new(*x, *y))
                .collect::<Vec<cell::Position>>()
        })
        .flatten()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    const SETTING: setting::GameSetting = setting::GameSetting {
        block_height: 2,
        block_width: 3,
    };
    #[test]
    fn test_create_vertical_groups() {
        let vg = create_vertical_groups(&cell::create_cells(&SETTING), &SETTING);
        assert_eq!(vg.len(), 6);
        assert_eq!(vg[0].borrow().cells.len(), 6);
        assert_eq!(
            vg[0].borrow().cells.get(0).unwrap().borrow().pos(),
            cell::Position::new(0, 0)
        );
        assert_eq!(
            vg[0].borrow().cells.get(5).unwrap().borrow().pos(),
            cell::Position::new(0, 5)
        );
        assert_eq!(
            vg[5].borrow().cells.get(0).unwrap().borrow().pos(),
            cell::Position::new(5, 0)
        );
        assert_eq!(
            vg[5].borrow().cells.get(5).unwrap().borrow().pos(),
            cell::Position::new(5, 5)
        );
    }
    #[test]
    fn test_create_horizontal_groups() {
        let hg = create_horizontal_groups(&cell::create_cells(&SETTING), &SETTING);
        assert_eq!(hg.len(), 6);
        assert_eq!(hg[0].borrow().cells.len(), 6);
        assert_eq!(
            hg[0].borrow().cells.get(0).unwrap().borrow().pos(),
            cell::Position::new(0, 0)
        );
        assert_eq!(
            hg[0].borrow().cells.get(5).unwrap().borrow().pos(),
            cell::Position::new(5, 0)
        );
        assert_eq!(
            hg[5].borrow().cells.get(0).unwrap().borrow().pos(),
            cell::Position::new(0, 5)
        );
        assert_eq!(
            hg[5].borrow().cells.get(5).unwrap().borrow().pos(),
            cell::Position::new(5, 5)
        );
    }
    mod test_create_block_groups {
        use super::*;
        #[test]
        fn test_create_block_start_positions() {
            let block_start_positions = create_block_start_positions(&SETTING);
            assert_eq!(
                block_start_positions,
                vec![
                    cell::Position::new(0, 0),
                    cell::Position::new(3, 0),
                    cell::Position::new(0, 2),
                    cell::Position::new(3, 2),
                    cell::Position::new(0, 4),
                    cell::Position::new(3, 4)
                ]
            )
        }
        #[test]
        fn block_group_count() {
            let groups = create_block_groups(&cell::create_cells(&SETTING), &SETTING);
            assert_eq!(groups.len(), 6);
        }
        #[test]
        fn block_group_cell_count() {
            let groups = create_block_groups(&cell::create_cells(&SETTING), &SETTING);
            assert!(groups.iter().all(|g| g.borrow().cells.len() == 6));
        }
        #[test]
        fn first_block_group_cells() {
            let groups = create_block_groups(&cell::create_cells(&SETTING), &SETTING);
            assert_eq!(
                groups[0]
                    .borrow()
                    .cells
                    .iter()
                    .map(|c| c.borrow().pos())
                    .collect::<Vec<cell::Position>>(),
                vec![
                    cell::Position::new(0, 0),
                    cell::Position::new(1, 0),
                    cell::Position::new(2, 0),
                    cell::Position::new(0, 1),
                    cell::Position::new(1, 1),
                    cell::Position::new(2, 1),
                ]
            );
        }
        #[test]
        fn last_block_group_cells() {
            let groups = create_block_groups(&cell::create_cells(&SETTING), &SETTING);
            assert_eq!(
                groups[5]
                    .borrow()
                    .cells
                    .iter()
                    .map(|c| c.borrow().pos())
                    .collect::<Vec<cell::Position>>(),
                vec![
                    cell::Position::new(3, 4),
                    cell::Position::new(4, 4),
                    cell::Position::new(5, 4),
                    cell::Position::new(3, 5),
                    cell::Position::new(4, 5),
                    cell::Position::new(5, 5),
                ]
            );
        }
    }
    #[test]
    fn get_lonely_returns_lonely() {
        let g = create_horizontal_groups(&cell::create_cells(&SETTING), &SETTING);
        // index が 2 以外の cell の解答候補から 3 を除去
        for (i, cell) in g[0].borrow().cells.iter().enumerate() {
            if i == 2 {
                continue;
            };
            cell.borrow_mut().remove_answer_candidate(3);
        }
        // index が 4 以外の cell の解答候補から 5 を除去
        for (i, cell) in g[0].borrow().cells.iter().enumerate() {
            if i == 4 {
                continue;
            };
            cell.borrow_mut().remove_answer_candidate(5);
        }
        assert_eq!(
            g[0].borrow().get_lonely(),
            vec![
                (cell::Position::new(2, 0), 3),
                (cell::Position::new(4, 0), 5)
            ]
        );
    }
}
