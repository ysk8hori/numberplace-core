pub mod normal_game;

#[cfg(test)]
mod tests {
    use super::normal_game;
    #[test]
    fn it_works() {
        let game = normal_game::NormalGame::new(normal_game::setting::GameSetting {
            block_height: 3,
            block_width: 3,
        });
        assert_eq!(2 + 2, 4);
    }
}
