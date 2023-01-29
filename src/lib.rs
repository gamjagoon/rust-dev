const FSM_COLUMN_SIZE : usize= 130;
const FSM_NEW_LINE : usize= 129;

#[derive(Default, Clone, Copy)]
struct FsmAction {
    next: usize,
    offset: i32
}

#[derive(Clone, Copy)]
pub struct FsmColumn {
    ts : [FsmAction;FSM_COLUMN_SIZE]
}

impl FsmColumn {
    fn new() -> Self { Self { ts: [Default::default(); FSM_COLUMN_SIZE], } }
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
                print!("({} {})", self.cs[col].ts[row].next
                                , self.cs[col].ts[row].offset);
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
                    for i in 'A'..='z' {
                        tmp.ts[i as usize] = FsmAction{
                            next : fsm.cs.len() + 1,
                            offset: 1
                        };
                    }
                    fsm.cs.push(tmp);
                }
                '$' => {
                    tmp.ts[FSM_NEW_LINE] = FsmAction{
                        next : fsm.cs.len() + 1,
                        offset: 1,
                    };
                    fsm.cs.push(tmp);
                }
                '+' => {
                    let n = fsm.cs.len();

                    fsm.cs.push(fsm.cs.last().unwrap().clone());

                    for t in fsm.cs.last_mut().unwrap().ts.iter_mut() {
                        if t.next == n { // Just leave it as it is. It's already looped 
                            continue;
                        } else if t.next == 0{
                            t.next = n + 1;
                            t.offset = 0;
                        } else {
                            unreachable!();
                        }
                    }
                }
                '*' => {
                    let n = fsm.cs.len();

                    for t in fsm.cs.last_mut().unwrap().ts.iter_mut() {
                        if t.next == n {
                            t.next = n - 1;
                        } else if t.next == 0{
                            t.next = n;
                            t.offset = 0;
                        } else {
                            unreachable!();
                        }
                    }
                }
                _ => {
                    tmp.ts[c as usize] = FsmAction{
                        next : fsm.cs.len() + 1,
                        offset: 1,
                    };
                    fsm.cs.push(tmp);
                }
            }
        }
        fsm
    }


    /// check input str is equal regex
    /// ### Examples
    /// ```
    /// use rust_dev::FsmRegex;
    ///
    /// let fsm_regex = FsmRegex::comp_regex("abc");
    /// assert_eq!(fsm_regex.match_str("abc"), true);
    /// ```
    pub fn match_str(&self, input:&str) -> bool {
        let n = input.len();
        let mut state = 1;
        let mut head = 0;
        let chars = input.chars().collect::<Vec<_>>();

        while 0 < state && state < self.cs.len() && head < n{
            dbg!(head, state);
            let action = self.cs[state].ts[chars[head as usize] as usize];

            dbg!(action.next, action.offset);
            state = action.next;
            head = (head as i32  + action.offset) as usize;
        }

        if state == 0 {
            return false;
        }

        if state < self.cs.len() {
            let action = self.cs[state].ts[FSM_NEW_LINE];
            state = action.next;
        }

        return state >= self.cs.len();
    }
}

#[cfg(test)]
mod tests {
    use crate::FsmRegex;

    #[test]
    fn basic_test () {
        let fsm = FsmRegex::comp_regex("a*bc$");
        fsm.dump();
        let test_list: Vec<(&str, bool)> = vec![
            ("abc" ,true ),
            ("abcd",false),
            ("bcd",false),
            ("bc",true),
            ("aabc",true)
        ];
        for test in test_list {
            assert_eq!(fsm.match_str(test.0), test.1);
            println!("{} PASS", test.0);
        }
    }

    #[test]
    fn basic_test3 () {
        let fsm = FsmRegex::comp_regex("a+bc$");
        fsm.dump();
        let test_list: Vec<(&str, bool)> = vec![
            ("abc" ,true ),
            ("abcd",false),
            ("bcd",false),
            ("bc",false),
            ("aabc",true)
        ];
        for test in test_list {
            assert_eq!(fsm.match_str(test.0), test.1);
            println!("{} PASS", test.0);
        }
    }

    #[test]
    fn match_all_test() {
        let fsm = FsmRegex::comp_regex(".*d$");
        fsm.dump();
        let test_list: Vec<(&str, bool)> = vec![
            ("abc" ,false),
            ("abcd",true),
            ("bcd",true),
            ("bc",false),
            ("aabc",false)
        ];
        for test in test_list {
            assert_eq!(fsm.match_str(test.0), test.1);
            println!("{} PASS", test.0);
        }
    }

    #[test]
    fn match_all_test2() {
        let fsm = FsmRegex::comp_regex("a*bc$");
        fsm.dump();
        let test_list: Vec<(&str, bool)> = vec![
            ("aabc" ,true ),
            ("abcd",false),
            ("bcd",false),
            ("bc",true),
            ("aacc",false)
        ];
        for test in test_list {
            assert_eq!(fsm.match_str(test.0), test.1);
            println!("{} PASS", test.0);
        }
    }
}
