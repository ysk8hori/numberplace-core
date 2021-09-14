use crate::normal_game::cell::Cell;
use crate::normal_game::cell::Position;
use crate::normal_game::setting::GameSetting;
use crate::normal_game::NormalGame;
use rand::prelude::*;

pub struct Generator {
    setting: GameSetting,
}

// impl Generator {
//     pub fn new(setting: GameSetting) -> Generator {
//         Generator { setting }
//     }
//     pub fn generate(&self) -> (NormalGame, NormalGame) {
//         let mut rng = rand::thread_rng();
//         let game = NormalGame::new(self.setting);
//         let mut answers = self.setting.answer_candidate();
//         answers.shuffle(&mut rng);
//         // game.groups().
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::normal_game::setting::BlockSize;
    use crate::solver::Solver;
}
