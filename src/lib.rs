const FSM_COLUMN_SIZE : usize= 130;
const FSM_NEW_LINE : usize= 129;

pub struct FsmColumn {
    ts : [usize;FSM_COLUMN_SIZE]
}

impl FsmColumn {
    fn new() -> Self { Self { ts: [0; FSM_COLUMN_SIZE], } }
}

pub struct FsmRegex {
    cs : Vec<FsmColumn>
}

impl FsmRegex {
    pub fn new() -> Self { Self { cs : Vec::new() } }

    pub fn dump(&self) {
        for row in 0..FSM_COLUMN_SIZE {
            print!("value {:03} : ", row);
            for col in 0..self.cs.len(){
                print!("{} ", self.cs[col].ts[row]);
            }
            println!("");

        }
    }

    /// .
    pub fn comp_regex(src:&str) -> FsmRegex {
        let mut fsm: FsmRegex = FsmRegex::new();
        fsm.cs.push(FsmColumn::new());
        for c in src.chars() {
            let mut tmp = FsmColumn::new();
            match c {
                '.' => {
                    for i in 'a'..='Z' {
                        tmp.ts[i as usize] = fsm.cs.len() + 1;
                    }
                }
                '&' => {
                    tmp.ts[FSM_NEW_LINE] = fsm.cs.len() + 1;
                }
                '*' => {
                    todo!(r#"something todo !"#);
                }
                _ => {
                    tmp.ts[c as usize] = fsm.cs.len() + 1;
                }
            }
            fsm.cs.push(tmp);
        }
        fsm
    }


    /// check input str is equal regex
    /// ### Examples
    /// ```
    /// use rust_dev::FsmRegex;
    ///
    /// let fsm_regex = FsmRegex::comp_regex("abc");
    /// assert_eq!(fsm_regex.match_str(input), true);
    /// ```
    pub fn match_str(&self, input:&str) -> bool {
        let mut state : usize = 1;
        for (i, c) in input.char_indices() {
            if state == 0 && state >= self.cs.len() {
                break;
            }
            state = self.cs[state].ts[c as usize];
        }

        if state < self.cs.len() {
            state = self.cs[state].ts[FSM_NEW_LINE];
        }

        return state >= self.cs.len();
    }
}

#[cfg(test)]
mod tests {
    use crate::FsmRegex;

    #[test]
    fn test () {
        let fsm = FsmRegex::comp_regex("abc&");
        fsm.dump();
        let test_list: Vec<(&str, bool)> = vec![
            ("abc" ,true ),
            ("abcd",false),
            ("bcd",false),
            ("bc",false),
            ("aabc",false)
        ];
        for test in test_list {
            assert_eq!(fsm.match_str(test.0), test.1);
            println!("{} PASS", test.0);
        }
    }
}
