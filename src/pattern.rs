use crate::normal_game::setting::BlockSize;

pub struct AnswerPattern {}

static P_1_3: &str = "a,b,c|b,c,a|c,a,b";
static P_2_2: &str = "a,b,c,d|c,d,a,b|b,a,d,c|d,c,b,a";
static P_2_3: &str = "a,b,c,d,e,f|d,e,f,a,b,c|b,c,a,e,f,d|e,f,d,b,c,a|c,a,b,f,d,e|f,d,e,c,a,b";
static P_3_2: &str = "a,b,c,d,e,f|c,d,e,f,a,b|e,f,a,b,c,d|b,a,d,c,f,e|d,c,f,e,b,a|f,e,b,a,d,c";
static P_4_2: &str = "a,b,c,d,e,f,g,h|c,d,g,h,a,b,e,f|e,f,a,b,g,h,c,d|g,h,e,f,c,d,a,b|b,a,d,c,f,e,h,g|d,c,h,g,b,a,f,e|f,e,b,a,h,g,d,c|h,g,f,e,d,c,b,a";
static P_2_4: &str = "a,b,c,d,e,f,g,h|e,f,g,h,a,b,c,d|b,d,a,c,f,g,h,e|f,h,e,g,b,d,a,c|c,a,f,b,h,e,d,g|d,g,h,e,c,a,b,f|g,c,b,f,d,h,e,a|h,e,d,a,g,c,f,b";
static P_3_3: &str = "a,b,c,d,e,f,g,h,i|d,e,f,g,h,i,a,b,c|g,h,i,a,b,c,d,e,f|b,c,a,f,g,d,h,i,e|h,g,e,i,a,b,c,f,d|f,i,d,e,c,h,b,a,g|c,a,g,b,f,e,i,d,h|e,d,b,h,i,g,f,c,a|i,f,h,c,d,a,e,g,b";
static P_2_5: &str = "a,b,c,d,e,f,g,h,i,j|f,g,h,i,j,a,b,c,d,e|b,e,a,c,d,i,h,j,g,f|g,j,f,h,i,b,e,a,c,d|c,a,d,e,b,h,i,f,j,g|h,f,i,j,g,c,a,d,e,b|d,c,g,b,a,j,f,e,h,i|e,i,j,f,h,d,c,g,b,a|i,d,e,a,c,g,j,b,f,h|j,h,b,g,f,e,d,i,a,c";
static P_5_2: &str = "a,b,c,d,e,f,g,h,i,j|c,d,i,j,a,b,e,f,g,h|e,f,a,b,g,h,i,j,c,d|g,h,e,f,i,j,c,d,a,b|i,j,g,h,c,d,a,b,e,f|b,a,d,c,f,e,h,g,j,i|d,c,j,i,b,a,f,e,h,g|f,e,b,a,h,g,j,i,d,c|h,g,f,e,j,i,d,c,b,a|j,i,h,g,d,c,b,a,f,e";
static P_4_3: &str = "a,b,c,d,e,f,g,h,i,j,k,l|d,e,f,j,k,l,a,b,c,g,h,i|g,h,i,a,b,c,j,k,l,d,e,f|j,k,l,g,h,i,d,e,f,a,b,c|b,c,a,h,f,d,k,i,e,l,j,g|i,j,g,k,c,a,b,l,h,f,d,e|f,l,h,e,i,g,c,d,j,k,a,b|e,d,k,l,j,b,f,a,g,i,c,h|c,f,d,b,g,h,i,j,a,e,l,k|h,a,b,f,l,j,e,g,k,c,i,d|k,g,j,i,d,e,l,c,b,h,f,a|l,i,e,c,a,k,h,f,d,b,g,j";
static P_3_4: &str = "a,b,c,d,e,f,g,h,i,j,k,l|e,f,g,h,i,j,k,l,a,b,c,d|i,j,k,l,a,b,c,d,e,f,g,h|b,g,a,c,j,i,d,e,k,l,h,f|j,k,l,e,g,h,b,f,c,a,d,i|d,h,i,f,k,l,a,c,g,e,b,j|c,a,f,j,b,d,i,k,h,g,l,e|l,e,d,g,h,c,j,a,f,k,i,b|h,i,b,k,f,e,l,g,d,c,j,a|f,c,h,b,l,g,e,i,j,d,a,k|g,l,e,a,d,k,h,j,b,i,f,c|k,d,j,i,c,a,f,b,l,h,e,g";
static P_4_4: &str = "a,b,c,d,e,f,g,h,i,j,k,l,m,n,o,p|e,f,g,h,m,n,o,p,a,b,c,d,i,j,k,l|i,j,k,l,a,b,c,d,m,n,o,p,e,f,g,h|m,n,o,p,i,j,k,l,e,f,g,h,a,b,c,d|b,d,a,c,h,i,j,e,n,k,l,f,p,o,m,g|o,k,l,n,p,d,a,f,b,m,e,g,h,c,i,j|g,m,p,i,n,k,b,o,h,c,j,a,f,d,l,e|h,e,j,f,c,g,l,m,o,p,d,i,k,a,b,n|c,a,h,b,f,e,i,g,k,d,p,m,j,l,n,o|l,g,d,m,k,o,n,a,c,i,h,j,b,p,e,f|n,p,i,j,d,l,m,b,f,e,a,o,c,g,h,k|f,o,e,k,j,h,p,c,g,l,b,n,d,i,a,m|d,h,f,a,l,c,e,k,j,g,n,b,o,m,p,i|j,c,m,o,g,p,h,i,l,a,f,e,n,k,d,b|k,l,n,e,b,m,d,j,p,o,i,c,g,h,f,a|p,i,b,g,o,a,f,n,d,h,m,k,l,e,j,c";
static P_4_5: &str = "a,b,c,d,e,f,g,h,i,j,k,l,m,n,o,p,q,r,s,t|f,g,h,i,j,p,q,r,s,t,a,b,c,d,e,k,l,m,n,o|k,l,m,n,o,a,b,c,d,e,p,q,r,s,t,f,g,h,i,j|p,q,r,s,t,k,l,m,n,o,f,g,h,i,j,a,b,c,d,e|b,i,a,c,d,l,m,p,e,f,r,n,q,j,g,t,o,s,k,h|q,r,s,p,f,t,h,k,b,g,l,o,d,m,c,i,a,e,j,n|n,t,j,m,g,o,s,i,a,r,e,p,k,f,h,d,c,b,l,q|e,k,l,o,h,j,d,q,c,n,b,s,t,a,i,g,m,f,p,r|c,a,k,j,b,i,n,e,m,d,t,h,l,q,r,s,p,o,g,f|s,m,p,f,r,b,o,j,t,h,g,d,i,c,k,l,n,q,e,a|o,d,t,g,i,c,k,f,l,q,n,a,p,e,s,h,r,j,m,b|h,e,n,l,q,g,p,a,r,s,m,j,o,b,f,c,d,i,t,k|d,o,b,h,m,n,f,g,p,c,s,e,a,t,l,q,j,k,r,i|l,p,e,q,s,m,t,o,j,i,c,k,f,r,d,b,h,n,a,g|r,j,f,t,k,s,a,l,h,b,q,i,g,o,n,m,e,p,c,d|i,c,g,a,n,e,r,d,q,k,j,m,b,h,p,o,s,t,f,l|g,h,o,b,c,d,e,n,f,p,i,t,j,l,m,r,k,a,q,s|j,n,i,k,a,r,c,b,g,l,h,f,s,p,q,e,t,d,o,m|m,s,d,e,p,q,i,t,k,a,o,r,n,g,b,j,f,l,h,c|t,f,q,r,l,h,j,s,o,m,d,c,e,k,a,n,i,g,b,p";
static P_5_4: &str = "a,b,c,d,e,f,g,h,i,j,k,l,m,n,o,p,q,r,s,t|e,f,g,h,q,r,s,t,a,b,c,d,i,j,k,l,m,n,o,p|i,j,k,l,a,b,c,d,m,n,o,p,q,r,s,t,e,f,g,h|m,n,o,p,i,j,k,l,q,r,s,t,e,f,g,h,a,b,c,d|q,r,s,t,m,n,o,p,e,f,g,h,a,b,c,d,i,j,k,l|b,d,a,c,k,h,j,e,o,p,l,f,r,q,m,g,s,t,n,i|r,o,q,n,s,t,d,g,b,c,e,m,p,i,f,a,j,l,h,k|p,k,h,e,f,i,l,n,s,q,j,r,o,t,b,c,d,g,a,m|j,m,l,i,b,a,q,r,t,d,h,g,n,k,e,s,f,c,p,o|s,t,f,g,c,p,m,o,k,a,i,n,d,h,l,j,b,e,q,r|c,a,m,o,p,k,h,b,d,i,q,e,g,l,t,n,r,s,j,f|g,p,j,b,o,q,e,i,l,s,n,a,f,m,h,r,t,k,d,c|h,q,n,f,l,m,a,c,g,t,r,b,j,s,d,k,o,p,i,e|d,e,i,r,j,g,t,s,f,k,m,c,b,a,p,o,h,q,l,n|k,l,t,s,n,d,r,f,h,o,p,j,c,e,i,q,g,m,b,a|f,c,d,m,g,o,b,q,n,l,a,i,s,p,r,e,k,h,t,j|l,i,p,k,d,c,f,m,j,e,t,q,h,g,a,b,n,o,r,s|n,s,e,a,h,l,i,k,r,g,b,o,t,c,j,f,p,d,m,q|o,g,r,j,t,s,n,a,p,h,f,k,l,d,q,m,c,i,e,b|t,h,b,q,r,e,p,j,c,m,d,s,k,o,n,i,l,a,f,g";
static P_5_5: &str = "a,b,c,d,e,f,g,h,i,j,k,l,m,n,o,p,q,r,s,t,u,v,w,x,y|f,g,h,i,j,u,v,w,x,y,a,b,c,d,e,k,l,m,n,o,p,q,r,s,t|k,l,m,n,o,a,b,c,d,e,p,q,r,s,t,u,v,w,x,y,f,g,h,i,j|p,q,r,s,t,k,l,m,n,o,u,v,w,x,y,f,g,h,i,j,a,b,c,d,e|u,v,w,x,y,p,q,r,s,t,f,g,h,i,j,a,b,c,d,e,k,l,m,n,o|b,e,a,c,d,j,k,l,m,f,r,u,n,o,g,v,w,s,p,h,y,x,t,q,i|q,t,u,v,w,x,n,e,a,g,y,c,f,h,i,l,m,k,o,b,j,p,d,r,s|j,p,s,f,i,o,w,u,y,d,v,m,l,t,x,n,r,q,a,c,g,e,b,h,k|x,y,n,l,r,b,h,q,p,v,d,s,k,j,w,g,i,t,e,f,c,m,a,o,u|h,o,g,k,m,s,t,i,c,r,b,a,e,q,p,x,j,u,y,d,l,n,f,v,w|c,a,o,p,b,n,i,s,q,h,m,t,v,u,d,j,e,g,r,w,x,k,y,l,f|v,w,x,t,s,d,e,f,l,a,o,r,p,k,c,y,n,b,h,m,i,j,u,g,q|y,d,q,h,l,m,p,g,r,b,n,i,j,e,f,o,x,v,k,u,t,w,s,a,c|m,n,e,u,k,v,x,j,t,c,g,w,y,a,s,q,d,i,f,l,b,h,o,p,r|g,j,i,r,f,w,y,o,u,k,x,h,q,b,l,s,t,a,c,p,m,d,n,e,v|d,k,b,e,n,i,m,a,f,u,l,j,g,w,q,c,s,o,t,x,v,r,p,y,h|i,m,j,g,v,r,c,n,b,w,e,o,t,p,a,d,h,y,u,q,s,f,l,k,x|o,s,p,y,x,h,d,t,v,q,c,k,u,f,b,r,a,j,l,n,w,i,e,m,g|r,c,f,w,a,l,j,p,g,s,h,y,x,m,n,b,k,e,v,i,o,t,q,u,d|l,h,t,q,u,e,o,y,k,x,i,d,s,r,v,m,f,p,w,g,n,a,j,c,b|e,f,v,j,c,q,r,b,h,m,t,n,a,y,k,i,o,l,g,s,d,u,x,w,p|n,r,d,a,h,t,s,v,o,i,q,x,b,c,u,w,p,f,m,k,e,y,g,j,l|s,i,k,m,p,y,a,d,w,l,j,f,o,g,h,e,u,x,b,r,q,c,v,t,n|t,u,y,o,q,g,f,x,e,n,w,p,i,l,m,h,c,d,j,v,r,s,k,b,a|w,x,l,b,g,c,u,k,j,p,s,e,d,v,r,t,y,n,q,a,h,o,i,f,m";
const PATTERN_CHARS: [char; 25] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y',
];

impl AnswerPattern {
    pub fn create_solved_string_from_pattern(
        block_size: &BlockSize,
        answer_candidate: &Vec<u8>,
    ) -> Option<String> {
        let size = (block_size.height, block_size.width);
        let pattern = match size {
            (1, 3) => Some(P_1_3),
            (2, 2) => Some(P_2_2),
            (2, 3) => Some(P_2_3),
            (3, 2) => Some(P_3_2),
            (4, 2) => Some(P_4_2),
            (2, 4) => Some(P_2_4),
            (3, 3) => Some(P_3_3),
            (2, 5) => Some(P_2_5),
            (5, 2) => Some(P_5_2),
            (4, 3) => Some(P_4_3),
            (3, 4) => Some(P_3_4),
            (4, 4) => Some(P_4_4),
            (4, 5) => Some(P_4_5),
            (5, 4) => Some(P_5_4),
            (5, 5) => Some(P_5_5),
            _ => return None,
        };

        let ziped = answer_candidate.iter().zip(PATTERN_CHARS.iter());
        let mut pattern = String::from(pattern.unwrap());
        for pair in ziped {
            pattern = String::from(pattern).replace(*pair.1, &pair.0.to_string());
        }
        Some(pattern)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    pub fn patternify(solved: String) -> String {
        solved
            .replace("25", "y")
            .replace("24", "x")
            .replace("23", "w")
            .replace("22", "v")
            .replace("21", "u")
            .replace("20", "t")
            .replace("19", "s")
            .replace("18", "r")
            .replace("17", "q")
            .replace("16", "p")
            .replace("15", "o")
            .replace("14", "n")
            .replace("13", "m")
            .replace("12", "l")
            .replace("11", "k")
            .replace("10", "j")
            .replace("9", "i")
            .replace("8", "h")
            .replace("7", "g")
            .replace("6", "f")
            .replace("5", "e")
            .replace("4", "d")
            .replace("3", "c")
            .replace("2", "b")
            .replace("1", "a")
    }
    #[test]
    fn it_returns_pattern_string_by_blocksize() {
        let pattern = AnswerPattern::create_solved_string_from_pattern(
            &BlockSize {
                height: 2,
                width: 3,
            },
            &vec![1, 2, 3, 4, 5, 6],
        );
        assert_eq!(
            pattern.unwrap(),
            "1,2,3,4,5,6|4,5,6,1,2,3|2,3,1,5,6,4|5,6,4,2,3,1|3,1,2,6,4,5|6,4,5,3,1,2"
        );
    }
    #[test]
    fn it_returns_none_string_by_unexpected_blocksize() {
        let maybe_none = AnswerPattern::create_solved_string_from_pattern(
            &BlockSize {
                height: 1,
                width: 1,
            },
            &vec![1],
        );
        assert_eq!(maybe_none, None);
    }
    mod create_patterns {
        use super::*;
        use crate::normal_game::setting::GameSetting;
        use crate::normal_game::NormalGame;
        #[test]
        #[ignore]
        fn create_2x3() {
            let game = NormalGame::new(GameSetting::new(BlockSize {
                height: 2,
                width: 3,
            }));
            let solved_game = game.solve();

            assert_eq!(
                patternify(solved_game.unwrap().to_string_with_comma()),
                "a,b,c,d,e,f|d,e,f,a,b,c|b,c,a,e,f,d|e,f,d,b,c,a|c,a,b,f,d,e|f,d,e,c,a,b"
            )
        }
    }
}
