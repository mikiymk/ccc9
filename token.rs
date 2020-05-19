#[derive(Debug)]
enum Token {
    RESERVED(String),
    IDENT(String),
    NUM(i64),
}

#[derive(Debug)]
pub struct TokenList {
    list : Vec<Token>,
}

pub fn tokenize(code: &String) -> Option<TokenList> {
    let mut vect = Vec::new();
    let codev = code.chars().collect::<Vec<char>>();
    let len = codev.len();
    let mut cur = 0;

    while cur < len {
        match codev[cur] {
            ' ' => {
                cur += 1;
            }
            '+' | '-' | '*' | '/' | '(' | ')' | ';' => {
                vect.push(Token::RESERVED(codev[cur].to_string()));
                cur += 1;
            }
            '0' ..= '9' => {
                let (lo, c) = str_to_long(code, cur);
                cur = c;
                vect.push(Token::NUM(lo));
            },
            '=' | '!' | '<' | '>' => {
                let next = codev[cur + 1];
                if next == '=' {
                    vect.push(Token::RESERVED(format!("{}{}", codev[cur], codev[cur + 1])));
                    cur += 2;
                } else {
                    vect.push(Token::RESERVED(codev[cur].to_string()));
                    cur += 1;
                }
            },
            'a' ..= 'z' => {
                vect.push(Token::IDENT(codev[cur].to_string()));
                cur += 1;
            }
            _ => {
                error_at(code, cur, "トークナイズ出来ません。");
                return None;
            }
        }
    }
    Some(TokenList{ list: vect })
}

fn str_to_long(code: &String, cursor: usize) -> (i64, usize) {
    let mut len = cursor;
    while len + 1 <= code.len() && code[cursor..len + 1].parse::<i64>().is_ok() {
      len += 1
    }
    (code[cursor..len].parse().unwrap(), len)
}

fn error_at(code: &str, pos: usize, error: &str) {
    eprintln!("{}", error);
    eprintln!("{}", code);
    eprintln!("{}^", " ".repeat(pos));
}

impl TokenList {
    fn get(&self) -> Option<&Token> {
        if self.at_eof() {
            None
        } else {
            Some(&self.list[0])
        }
    }

    fn pop(&mut self) -> Option<Token> {
        if self.at_eof() {
            None
        } else {
            Some(self.list.remove(0))
        }
    }

    pub fn at_eof(&self) -> bool {
        self.list.len() == 0
    }

    pub fn consume(&mut self, stri: &str) -> bool {
        if let Some(Token::RESERVED(ref s)) = self.get() {
            if s == stri {
                self.pop();
                return true;
            }
        }
        false
    }

    pub fn consume_ident(&self) -> bool {
        matches!(self.get(), Some(Token::IDENT(_)))
    }

    pub fn expect(&mut self, stri: &str) -> bool {
        if let Token::RESERVED(s) = self.pop().unwrap() {
            stri == s
        } else {
            false
        }
    }

    pub fn expect_num(&mut self) -> Option<i64> {
        match self.pop() {
            Some(Token::NUM(i)) => Some(i),
            _ => None,
        }
    }

    pub fn expect_ident(&mut self) -> Option<i64> {
        match self.pop() {
            Some(Token::IDENT(s)) => {
                let c = s.chars().nth(0).unwrap();
                Some(c as i64 - 96)
            },
            _ => None,
        }
    }
}