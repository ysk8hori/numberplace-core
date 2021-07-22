pub mod game;

#[cfg(test)]
mod tests {
    use super::game::setting;
    #[test]
    fn it_works() {
        let setting = setting::GameSetting::new(3, 3);
        println!("{:?}", setting);
        assert_eq!(2 + 2, 4);
    }
}
