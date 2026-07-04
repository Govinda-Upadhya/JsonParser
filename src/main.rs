#[derive(PartialEq,Debug)]
enum Token{
    OpenBraces,
    CloseBraces,
    ListStart,
    ListEnd,
    StringToken(String),
    NumberToken(i64),
    Colon,
    Comma,
    Boolean(bool),
    Null
}
struct Lexer{
    input:Vec<char>,
    position:usize
}
impl Lexer {
    fn new(input:&str)->Self{
        Self { input: input.chars().collect(), position: 0 }
    }
    fn peek(&self)->Option<char>{
        if self.position < self.input.len() {
            Some(self.input[self.position])
        } else {
            None
        }
    }
    fn advance(&mut self){
        self.position+=1;
    }
    fn read_stringtoken(&mut self)->String{
        let mut stringval=String::new();
        while let Some(current)=self.peek(){
            if current.is_alphabetic() && current !='"'{
                stringval.push(current);
                self.advance();
            }else{
                break;
            }
        }
        self.advance();
        stringval
    }
    fn read_keyword(&mut self) -> String {
        let mut word = String::new();

        while let Some(ch) = self.peek() {
            if ch.is_ascii_alphabetic() {
                word.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        word
    }
    fn read_number(&mut self)->i64{
        let mut number=String::new();
        
        while let Some(current)=self.peek(){
            
            if current.is_ascii_digit() {
                number.push(current);
                self.advance(); 
            }else{
                break;
            }
        }
        number.parse().unwrap()
    }
    
    fn skip_whitespace(&mut self){
        while let Some(current)=self.peek() {
            if current.is_whitespace() {
                self.advance();
            }else{
                break;
            }
        }
    }
    fn next_token(&mut self)->Option<Token>{
        self.skip_whitespace();

        let ch = self.peek()?;
        match ch {
            '{'=>{
                self.advance();
                Some(Token::OpenBraces)
            },
            '}'=>{
                self.advance();
                Some(Token::CloseBraces)
            },
            '['=>{
                self.advance();
                Some(Token::ListStart)
            },
            ']'=>{
                self.advance();
                Some(Token::ListEnd)
            },
            ':'=>{
                self.advance();
                Some(Token::Colon)
            },
            ','=>{
                self.advance();
                Some(Token::Comma)
            },
            c if c.is_ascii_digit() => {
                Some(Token::NumberToken(self.read_number()))
            },
            '"' => {
                self.advance();
                
                Some(Token::StringToken(self.read_stringtoken()))
                
            },
            't'=>{
                match self.read_keyword().as_str() {
                    "true" => Some(Token::Boolean(true)),
                    "null" => Some(Token::Null),
                    _ => panic!("Unknown keyword"),
                }
            },
            'f'=>{
                match self.read_keyword().as_str() {
                    "false" => Some(Token::Boolean(false)),
                    "null" => Some(Token::Null),
                    _ => panic!("Unknown keyword"),
                }
            },
            
            _ => panic!("Unexpected character {}", ch),
        }
    }
    fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while let Some(token) = self.next_token() {
            tokens.push(token);
        }

        tokens
    }
}
fn main(){
    let raw_string=r#"{"name": "alice", "age": 30, "scores": [95, 87, 100], "active": true}"#;
    let mut lexer= Lexer::new(raw_string);
    let tokens= lexer.tokenize();
    println!("{:#?}",tokens);
}