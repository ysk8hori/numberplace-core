// // use crate::normal_game::group::Group;
// use crate::normal_game::{group::Group, *};

// pub fn solving(game: NormalGame) {}

// /// If there is only one possible answer, confirm it.
// pub fn fill_lonely(group: &Group) {
//     for candidate in group.unanswerd_candidate.iter() {
//         let asdf: Vec<Rc<RefCell<cell::Cell>>> = group
//             .cells
//             .filter(|c| c.borrow().has_answer_candidate(*candidate));
//         if asdf.len() == 1 {
//             asdf.get(0).unwrap().borrow_mut().set_answer(*candidate);
//         }
//     }
// }
