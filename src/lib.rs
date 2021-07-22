pub mod game;

#[cfg(test)]
mod tests {
    use super::game;
    #[test]
    fn it_works() {
        let game = game::Game::new(game::setting::GameSetting {
            block_height: 3,
            block_width: 3,
        });
        println!("{:?}", game);
        assert_eq!(2 + 2, 4);
    }
}
