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
        while let Some(current) = self.peek() {
            if current == '"' {
                break;
            }

            stringval.push(current);
            self.advance();
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
#[derive(Debug)]
pub enum JsonValue {
    Object(Vec<(String, JsonValue)>),

    Array(Vec<JsonValue>),

    String(String),

    Number(i64),

    Boolean(bool),

    Null,
}
impl JsonValue {
    fn get(&self, key: &str) -> Option<&JsonValue> {
        match self {
            JsonValue::Object(pairs) => {
                for (k, v) in pairs {
                    if k == key {
                        return Some(v);
                    }
                }
                None
            }

            _ => None,
        }
    }
}
pub struct Parser{
    input:Vec<Token>,
    position:usize
}
impl Parser{
    fn new(input:Vec<Token>)->Self{
        Self { input, position: 0 }
    }
    fn peek(&self)->&Token{
        if self.position < self.input.len() {
            &self.input[self.position]
        } else {
            panic!("Unexpected end of token stream");
        }
    }
    fn advance(&mut self){
        self.position+=1;
    }
    fn expect(&mut self, description: &str) -> &Token {
        if self.position < self.input.len() {
            let token = &self.input[self.position];
            self.position += 1;
            token
        } else {
            panic!("Expected {} but reached end of input", description);
        }
    }
    fn parse_value(&mut self)->JsonValue{
        match self.peek(){
            Token::OpenBraces    => self.parse_object(),
            Token::ListStart  => self.parse_array(),
            Token::Boolean(true)         => { self.advance(); JsonValue::Boolean(true)  }
            Token::Boolean(false)        => { self.advance(); JsonValue::Boolean(false) }
            Token::Null         => { self.advance(); JsonValue::Null        }
            Token::NumberToken(n) => {
                let value = *n;
                self.advance();
                JsonValue::Number(value)
            }
            Token::StringToken(s) => {
                let value = s.clone();
                self.advance();
                JsonValue::String(value)
            }
            Token::CloseBraces   => panic!("Unexpected '}}'"),
            Token::ListEnd => panic!("Unexpected ']'"),
            Token::Colon        => panic!("Unexpected ':'"),
            Token::Comma        => panic!("Unexpected ','"),
        }
    }
    fn parse_object(&mut self)->JsonValue{
        self.advance();
        let mut pairs: Vec<(String, JsonValue)> = Vec::new();
        if *self.peek() == Token::CloseBraces {
            self.advance();
            return JsonValue::Object(pairs);
        }
        loop {
            let key = match self.expect("object key") {
                Token::StringToken(s) => s.clone(),
                _other => panic!("Expected string key, got something else"),
            };

            
            match self.expect("colon") {
                Token::Colon => {}
                _ => panic!("Expected ':' after object key"),
            }

            
            let value = self.parse_value();

            pairs.push((key, value));
            match self.peek() {
                Token::Comma => {
                    self.advance(); 
                }
                Token::CloseBraces => {
                    self.advance(); 
                    break;
                }
                _ => panic!("Expected ',' or '}}' in object"),
            }
        }
        return JsonValue::Object(pairs);
    }
    fn parse_array(&mut self) -> JsonValue {
        self.advance(); 

        let mut elements: Vec<JsonValue> = Vec::new();

       
        if let Token::ListEnd = self.peek() {
            self.advance();
            return JsonValue::Array(elements);
        }

        loop {
            let element = self.parse_value(); // recursive call
            elements.push(element);

            match self.peek() {
                Token::Comma => {
                    self.advance();
                }
                Token::ListEnd => {
                    self.advance(); 
                    break;
                }
                _ => panic!("Expected ',' or ']' in array"),
            }
        }

        JsonValue::Array(elements)
    }
    fn parse(tokens: Vec<Token>) -> JsonValue {
        let mut parser = Parser::new(tokens);
        let value = parser.parse_value();
        value
    }
}
fn main(){
    let raw_string=r#"{"name": "alice", "age": 30, "scores": [95, 87, 100], "active": true}"#;
    let mut lexer= Lexer::new(raw_string);
    let tokens= lexer.tokenize();
    println!("{:#?}",tokens);
    let json= Parser::parse(tokens);
    if let Some(age) = json.get("age") {
        println!("{:?}", age);
    }
   
   
    
}