use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, Token<'static>> = {
        let mut m = HashMap::new();
        m.insert("select", Token::Select);
        m.insert("from", Token::From);
        m.insert("where", Token::Where);
        m.insert("insert", Token::Insert);
        m.insert("into", Token::Into);
        m.insert("values", Token::Values);
        m.insert("update", Token::Update);
        m.insert("set", Token::Set);
        m.insert("delete", Token::Delete);
        m.insert("create", Token::Create);
        m.insert("table", Token::Table);
        m.insert("primary", Token::Primary);
        m.insert("key", Token::Key);
        m.insert("foreign", Token::Foreign);
        m.insert("references", Token::References);
        m.insert("drop", Token::Drop);
        m.insert("alter", Token::Alter);
        m.insert("add", Token::Add);
        m.insert("column", Token::Column);
        m.insert("constraint", Token::Constraint);
        m.insert("index", Token::Index);
        m.insert("join", Token::Join);
        m.insert("inner", Token::Inner);
        m.insert("left", Token::Left);
        m.insert("right", Token::Right);
        m.insert("full", Token::Full);
        m.insert("outer", Token::Outer);
        m.insert("on", Token::On);
        m.insert("group", Token::Group);
        m.insert("by", Token::By);
        m.insert("order", Token::Order);
        m.insert("asc", Token::Asc);
        m.insert("desc", Token::Desc);
        m.insert("union", Token::Union);
        m.insert("all", Token::All);
        m.insert("distinct", Token::Distinct);
        m.insert("limit", Token::Limit);
        m.insert("offset", Token::Offset);
        m.insert("having", Token::Having);
        m.insert("as", Token::As);
        m.insert("and", Token::And);
        m.insert("or", Token::Or);
        m.insert("not", Token::Not);
        m.insert("null", Token::Null);
        m.insert("is", Token::Is);
        m.insert("in", Token::In);
        m.insert("between", Token::Between);
        m.insert("like", Token::Like);
        m.insert("exists", Token::Exists);
        m.insert("any", Token::Any);
        m.insert("case", Token::Case);
        m.insert("when", Token::When);
        m.insert("then", Token::Then);
        m.insert("else", Token::Else);
        m.insert("end", Token::End);
        m.insert("default", Token::Default);
        m.insert("true", Token::BooleanLiteral(true));
        m.insert("false", Token::BooleanLiteral(false));
        m.insert("int", Token::Int);
        m.insert("integer", Token::Integer);
        m.insert("smallint", Token::SmallInt);
        m.insert("tinyint", Token::TinyInt);
        m.insert("bigint", Token::BigInt);
        m.insert("float", Token::Float);
        m.insert("real", Token::Real);
        m.insert("double", Token::Double);
        m.insert("decimal", Token::Decimal);
        m.insert("numeric", Token::Numeric);
        m.insert("varchar", Token::VarChar);
        m.insert("char", Token::Char);
        m.insert("text", Token::Text);
        m.insert("date", Token::Date);
        m.insert("datetime", Token::Datetime);
        m.insert("time", Token::Time);
        m.insert("timestamp", Token::Timestamp);
        m.insert("boolean", Token::Boolean);
        m
    };
}


#[derive(Debug, PartialEq, Clone)]
pub enum LexError {
    InvalidCharacter(char),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token<'a> {
    // Keywords
    Select,
    From,
    Where,
    Insert,
    Into,
    Values,
    Update,
    Set,
    Delete,
    Create,
    Table,
    Primary,
    Key,
    Foreign,
    References,
    Drop,
    Alter,
    Add,
    Column,
    Constraint,
    Index,
    Join,
    Inner,
    Left,
    Right,
    Full,
    Outer,
    On,
    Group,
    By,
    Order,
    Asc,
    Desc,
    Union,
    All,
    Distinct,
    Limit,
    Offset,
    Having,
    As,
    And,
    Or,
    Not,
    Null,
    Is,
    In,
    Between,
    Like,
    Exists,
    Any,
    Case,
    When,
    Then,
    Else,
    End,
    Default,
    // Data Types
    Int,
    Integer,
    SmallInt,
    TinyInt,
    BigInt,
    Float,
    Real,
    Double,
    Decimal,
    Numeric,
    VarChar,
    Char,
    Text,
    Date,
    Datetime,
    Time,
    Timestamp,
    Boolean,
    // Symbols and Operators
    Asterisk,
    Comma,
    Semicolon,
    OpenParen,
    CloseParen,
    Equals,
    NotEquals,
    LessThan,
    GreaterThan,
    LessThanOrEquals,
    GreaterThanOrEquals,
    Plus,
    Minus,
    Slash,
    Percent,
    Concat,
    SingleQuote,
    DoubleQuote,
    // Identifiers and Literals
    Identifier {
        first_name: &'a str,
        second_name: Option<&'a str>,
        third_name: Option<&'a str>,
    },
    StringLiteral(String),
    NumericLiteral(String),
    BooleanLiteral(bool),
    // Comments
    SingleLineComment(String),
    MultiLineComment(String),
}

impl<'a> Token<'a> {
    pub fn identifier(first_name: &'a str) -> Token<'a> {
        Token::Identifier {
            first_name,
            second_name: None,
            third_name: None,
        }
    }
}

struct Lexer<'a> {
    input: &'a str,
    input_iterator: std::str::Chars<'a>,
    current_position: usize,
    is_finished: bool,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token<'a>, LexError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let c = self.skip_whitespace()?;
            return match c {
                '*' => Some(self.single(Token::Asterisk)),
                ',' => Some(self.single(Token::Comma)),
                '=' => Some(self.single(Token::Equals)),
                '+' => Some(self.single(Token::Plus)),
                '%' => Some(self.single(Token::Percent)),
                '|' => Some(self.single(Token::Concat)),

                // may be longer
                '<' => Some(self.may_be_longer(Token::LessThan)),
                '>' => Some(self.may_be_longer(Token::GreaterThan)),
                '/' => Some(self.may_be_longer(Token::Slash)),
                '\'' => Some(self.may_be_longer(Token::SingleQuote)),
                '-' => Some(self.may_be_longer(Token::Minus)),
                '"' => Some(self.may_be_longer(Token::DoubleQuote)),

                ';' => Some(Ok(Token::Semicolon)),
                '(' => Some(Ok(Token::OpenParen)),
                ')' => Some(Ok(Token::CloseParen)),
                '!' => Some(Ok(Token::NotEquals)),
                c => {
                    if c.is_alphabetic() || c == '_' {
                        Some(self.word_started())
                    } else if c.is_numeric() {
                        Some(self.numeric_started(false))
                    } else {
                        Some(Err(LexError::InvalidCharacter(c)))
                    }
                }
            }
        }
    }
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            input_iterator: input.chars(),
            current_position: 0,
            is_finished: false,
        }
    }
    fn get_next_and_increment(&mut self) -> Option<char> {
        let c = self.input_iterator.next();
        if c.is_some() {
            self.current_position += 1;
        } else {
            self.is_finished = true;
        }
        c
    }

    fn get_last_token_end(&self) -> usize {
        if self.is_finished {
            return self.current_position;
        }
        self.current_position - 1
    }

    fn skip_whitespace(&mut self) -> Option<char> {
        while let Some(c) = self.get_next_and_increment() {
            if !c.is_whitespace() {
                return Some(c);
            }
        }
        None
    }

    fn numeric_started(&mut self, has_sign: bool) -> Result<Token<'a>, LexError> {
        let is_numeric = |c: Option<char>| -> bool {
            match c {
                Some(n) => n.is_numeric(),
                _ => false,
            }
        };
        let started_position = self.get_last_token_end();
        let mut seen_dot = false;
        loop {
            let c = self.get_next_and_increment();
            if is_numeric(c) {
                continue;
            } else if c == Some('.') {
                if seen_dot {
                    return Err(LexError::InvalidCharacter('.'));
                }
                seen_dot = true;
                continue;
            } else {
                break;
            }
        }
        let literal = &self.input[started_position..self.get_last_token_end()];
        Ok(Token::NumericLiteral(if has_sign { format!("-{}", literal) } else { literal.to_string() }))
    }

    fn identifier_dot_started(&mut self, started_position: usize) -> Result<Token<'a>, LexError> {
        let first_dot_position = self.current_position;
        let mut second_dot_position = 0;
        let mut third_dot_position = 0;
        let mut is_word = |c: Option<char>, cp: usize| -> bool {
            match c {
                Some('.') => {
                    if second_dot_position == 0 {
                        second_dot_position = cp;
                        return true;
                    }
                    if third_dot_position == 0 {
                        third_dot_position = cp;
                    }
                    return true;
                }
                Some(n) => n.is_alphabetic() || n.is_numeric() || n == '_',
                _ => false,
            }
        };
        loop {
            let c = self.get_next_and_increment();
            if is_word(c, self.current_position) {
                continue;
            } else {
                break;
            }
        }
        if third_dot_position > 0 {
            return Err(LexError::InvalidCharacter('.'));
        }
        let first_name = &self.input[started_position..first_dot_position - 1];
        let second_name = if second_dot_position > 0 {
            &self.input[first_dot_position..second_dot_position - 1]
        } else {
            &self.input[first_dot_position..self.get_last_token_end()]
        };
        let third_name = if second_dot_position > 0 {
            Some(&self.input[second_dot_position..self.get_last_token_end()])
        } else {
            None
        };
        return Ok(Token::Identifier {
            first_name,
            second_name: Some(second_name),
            third_name,
        });
    }
    fn word_started(&mut self) -> Result<Token<'a>, LexError> {
        let started_position = self.get_last_token_end();

        let is_word = |c: Option<char>| -> bool {
            match c {
                Some(n) => n.is_alphabetic() || n.is_numeric() || n == '_',
                _ => false,
            }
        };

        loop {
            let c = self.get_next_and_increment();
            if is_word(c) {
                continue;
            } else {
                if c == Some('.') {
                    return self.identifier_dot_started(started_position);
                }
                break;
            }
        }
        let word = &self.input[started_position..self.get_last_token_end()];
        let lower_case_word = word.to_lowercase();

        if let Some(token) = KEYWORDS.get(lower_case_word.as_str()) {
            Ok(token.clone())
        } else {
            Ok(Token::identifier(word))
        }
    }


    fn single(&mut self, token: Token<'a>) -> Result<Token<'a>, LexError> {
        let c = self.get_next_and_increment();
        match c {
            Some(c) => {
                if c.is_whitespace() {
                    Ok(token)
                } else {
                    Err(LexError::InvalidCharacter(c))
                }
            }
            None => Ok(token),
        }
    }

    fn quote_started(&mut self, quote: Token<'a>) -> Result<Token<'a>, LexError> {
        let same_quote = |c: Option<char>| -> bool {
            match c {
                Some('\'') => quote == Token::SingleQuote,
                Some('"') => quote == Token::DoubleQuote,
                _ => false,
            }
        };
        let started_position = self.current_position;
        loop {
            let c = self.get_next_and_increment();
            if !same_quote(c) {
                continue;
            } else {
                break;
            }
        }
        let literal = &self.input[started_position..self.get_last_token_end()];
        Ok(Token::StringLiteral(literal.to_string()))
    }

    fn single_line_comment_started(&mut self) -> Result<Token<'a>, LexError> {
        let started_position = self.current_position;
        loop {
            let c = self.get_next_and_increment();
            if c == Some('\n') || c == None {
                break;
            }
        }
        let comment = &self.input[started_position..self.get_last_token_end()];
        Ok(Token::SingleLineComment(comment.to_string()))
    }

    fn multi_line_comment(&mut self) -> Result<Token<'a>, LexError> {
        let started_position = self.current_position;
        loop {
            let c = self.get_next_and_increment();
            if c == Some('*') {
                let next = self.get_next_and_increment();
                if next == Some('/') {
                    break;
                }
            }
        }
        let comment = &self.input[started_position..self.current_position - 2];
        Ok(Token::MultiLineComment(comment.to_string()))
    }

    fn may_be_longer(&mut self, first: Token<'a>) -> Result<Token<'a>, LexError> {
        match first {
            Token::SingleQuote | Token::DoubleQuote => return self.quote_started(first),
            _ => (),
        }

        let second = self.get_next_and_increment();
        match first {
            Token::LessThan => match second {
                Some('=') => Ok(Token::LessThanOrEquals),
                Some('>') => Ok(Token::NotEquals),
                Some(' ') => Ok(Token::LessThan),
                Some(c) => Err(LexError::InvalidCharacter(c)),
                None => Ok(Token::LessThan),
            },
            Token::GreaterThan => match second {
                Some('=') => Ok(Token::GreaterThanOrEquals),
                Some(' ') => Ok(Token::GreaterThan),
                Some(c) => Err(LexError::InvalidCharacter(c)),
                None => Ok(Token::GreaterThan),
            },
            Token::Slash => match second {
                Some(' ') => Ok(Token::Slash),
                Some('*') => self.multi_line_comment(),
                Some(c) => Err(LexError::InvalidCharacter(c)),
                None => Ok(Token::Slash),
            },
            Token::Minus => match second {
                Some(' ') => Ok(Token::Minus),
                Some('-') => self.single_line_comment_started(),
                Some(c) => {
                    if c.is_numeric() {
                        self.numeric_started(true)
                    } else {
                        Err(LexError::InvalidCharacter(c))
                    }
                }
                None => Ok(Token::Minus),
            },
            _ => Err(LexError::InvalidCharacter(second.unwrap())),
        }
    }

    pub fn tokenize(&self) -> Vec<Token<'a>> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::lexer::LexError::InvalidCharacter;

    #[test]
    fn test_empty_input() {
        let input = "";
        let lexer = Lexer::new(input);
        let tokens: Vec<Result<Token, LexError>> = lexer.collect();
        println!("{:?}", tokens);
        let expected = vec![];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_double_asterisk() {
        let input = "* *  **";
        let lexer = Lexer::new(input);
        let tokens: Vec<Result<Token, LexError>> = lexer.collect();
        let expected = vec![
            Ok(Token::Asterisk),
            Ok(Token::Asterisk),
            Err(InvalidCharacter('*')),
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_invalid_character() {
        let input = "&";
        let lexer = Lexer::new(input);
        let tokens: Vec<Result<Token, LexError>> = lexer.collect();
        let expected = vec![Err(InvalidCharacter('&'))];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_tokenizer_str() {
        let input = "SELECT * FROM users WHERE age = 1;";
        let lexer = Lexer::new(input);
        let tokens: Vec<_> = lexer.collect();
        let expected = vec![
            Ok(Token::Select),
            Ok(Token::Asterisk),
            Ok(Token::From),
            Ok(Token::identifier("users")),
            Ok(Token::Where),
            Ok(Token::identifier("age")),
            Ok(Token::Equals),
            Ok(Token::NumericLiteral("1".to_string())),
            // Ok(Token::Semicolon),
            // Token::EOF,
        ];
        assert_eq!(tokens, expected);
    }
    #[test]
    fn lex_single_chars() {
        let input = "* ; ( ) = ! < > + - /";
        let lexer = Lexer::new(input);
        let tokens: Vec<Result<Token, LexError>> = lexer.collect();
        println!("{:?}", tokens);
        let expected = vec![
            Ok(Token::Asterisk),
            Ok(Token::Semicolon),
            Ok(Token::OpenParen),
            Ok(Token::CloseParen),
            Ok(Token::Equals),
            Ok(Token::NotEquals),
            Ok(Token::LessThan),
            Ok(Token::GreaterThan),
            Ok(Token::Plus),
            Ok(Token::Minus),
            Ok(Token::Slash),
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn lex_longer_tokens() {
        let input = "<= >= <>";
        let lexer = Lexer::new(input);
        let tokens: Vec<Result<Token, LexError>> = lexer.collect();
        println!("{:?}", tokens);
        let expected = vec![
            Ok(Token::LessThanOrEquals),
            Ok(Token::GreaterThanOrEquals),
            Ok(Token::NotEquals),
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn lex_invalid_longer_tokens() {
        let input = "<= >=";
        let lexer = Lexer::new(input);
        let tokens: Vec<Result<Token, LexError>> = lexer.collect();
        println!("{:?}", tokens);
        let expected = vec![Ok(Token::LessThanOrEquals), Ok(Token::GreaterThanOrEquals)];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn lex_keywords() {
        let input = "SELECT FROM WHERE INSERT INTO VALUES UPDATE SET DELETE CREATE TABLE PRIMARY KEY FOREIGN REFERENCES DROP ALTER ADD COLUMN CONSTRAINT INDEX JOIN INNER LEFT RIGHT FULL OUTER ON GROUP BY ORDER ASC DESC UNION ALL DISTINCT LIMIT OFFSET HAVING AS AND OR NOT NULL IS IN BETWEEN LIKE EXISTS ANY CASE WHEN THEN ELSE END DEFAULT";
        let lexer = Lexer::new(input);
        let tokens: Vec<Result<Token, LexError>> = lexer.collect();
        let expected = vec![
            Ok(Token::Select),
            Ok(Token::From),
            Ok(Token::Where),
            Ok(Token::Insert),
            Ok(Token::Into),
            Ok(Token::Values),
            Ok(Token::Update),
            Ok(Token::Set),
            Ok(Token::Delete),
            Ok(Token::Create),
            Ok(Token::Table),
            Ok(Token::Primary),
            Ok(Token::Key),
            Ok(Token::Foreign),
            Ok(Token::References),
            Ok(Token::Drop),
            Ok(Token::Alter),
            Ok(Token::Add),
            Ok(Token::Column),
            Ok(Token::Constraint),
            Ok(Token::Index),
            Ok(Token::Join),
            Ok(Token::Inner),
            Ok(Token::Left),
            Ok(Token::Right),
            Ok(Token::Full),
            Ok(Token::Outer),
            Ok(Token::On),
            Ok(Token::Group),
            Ok(Token::By),
            Ok(Token::Order),
            Ok(Token::Asc),
            Ok(Token::Desc),
            Ok(Token::Union),
            Ok(Token::All),
            Ok(Token::Distinct),
            Ok(Token::Limit),
            Ok(Token::Offset),
            Ok(Token::Having),
            Ok(Token::As),
            Ok(Token::And),
            Ok(Token::Or),
            Ok(Token::Not),
            Ok(Token::Null),
            Ok(Token::Is),
            Ok(Token::In),
            Ok(Token::Between),
            Ok(Token::Like),
            Ok(Token::Exists),
            Ok(Token::Any),
            Ok(Token::Case),
            Ok(Token::When),
            Ok(Token::Then),
            Ok(Token::Else),
            Ok(Token::End),
            Ok(Token::Default),
        ];
        println!("actual: {:?}", tokens);
        println!("expected: {:?}", expected);
        assert_eq!(tokens, expected);
    }

    #[test]
    fn string_literals() {
        let input = "'hello' 'world'";
        let lexer = Lexer::new(input);
        let tokens: Vec<Result<Token, LexError>> = lexer.collect();
        println!("{:?}", tokens);
        let expected = vec![
            Ok(Token::StringLiteral("hello".to_string())),
            Ok(Token::StringLiteral("world".to_string())),
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn string_literal_with_escape() {
        let input = "'hello \"world\"'";
        let lexer = Lexer::new(input);
        let tokens: Vec<Result<Token, LexError>> = lexer.collect();
        println!("{:?}", tokens);
        let expected = vec![Ok(Token::StringLiteral("hello \"world\"".to_string()))];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn numeric_literals() {
        let input = "1 2 3.45";
        let lexer = Lexer::new(input);
        let tokens: Vec<Result<Token, LexError>> = lexer.collect();
        println!("{:?}", tokens);
        let expected = vec![
            Ok(Token::NumericLiteral("1".to_string())),
            Ok(Token::NumericLiteral("2".to_string())),
            Ok(Token::NumericLiteral("3.45".to_string())),
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn boolean_literals() {
        let input = "true false";
        let lexer = Lexer::new(input);
        let tokens: Vec<Result<Token, LexError>> = lexer.collect();
        println!("{:?}", tokens);
        let expected = vec![
            Ok(Token::BooleanLiteral(true)),
            Ok(Token::BooleanLiteral(false)),
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn date_literals() {
        let input = "DATE '2021-01-01'";
        let lexer = Lexer::new(input);
        let tokens: Vec<Result<Token, LexError>> = lexer.collect();
        println!("{:?}", tokens);
        let expected = vec![
            Ok(Token::Date),
            Ok(Token::StringLiteral("2021-01-01".to_string())),
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn time_literals() {
        let input = "TIME '12:34:56'";
        let lexer = Lexer::new(input);
        let tokens: Vec<Result<Token, LexError>> = lexer.collect();
        println!("{:?}", tokens);
        let expected = vec![
            Ok(Token::Time),
            Ok(Token::StringLiteral("12:34:56".to_string())),
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn datetime_literals() {
        let input = "DATETIME '2021-01-01 12:34:56'";
        let lexer = Lexer::new(input);
        let tokens: Vec<Result<Token, LexError>> = lexer.collect();
        println!("{:?}", tokens);
        let expected = vec![
            Ok(Token::Datetime),
            Ok(Token::StringLiteral("2021-01-01 12:34:56".to_string())),
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn single_line_comment() {
        let input = "-- this is a comment";
        let lexer = Lexer::new(input);
        let tokens: Vec<Result<Token, LexError>> = lexer.collect();

        let expected = vec![Ok(Token::SingleLineComment(
            " this is a comment".to_string(),
        ))];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn single_line_comment_in_sql() {
        let input = "SELECT * FROM users -- this is a comment";
        let lexer = Lexer::new(input);
        let tokens: Vec<Result<Token, LexError>> = lexer.collect();

        let expected = vec![
            Ok(Token::Select),
            Ok(Token::Asterisk),
            Ok(Token::From),
            Ok(Token::identifier("users")),
            Ok(Token::SingleLineComment(" this is a comment".to_string())),
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn multi_line_comment() {
        let input = "/* this is a comment */";
        let lexer = Lexer::new(input);
        let tokens: Vec<Result<Token, LexError>> = lexer.collect();

        let expected = vec![Ok(Token::MultiLineComment(
            " this is a comment ".to_string(),
        ))];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn multi_line_comment_in_sql() {
        let input = "SELECT * FROM users /* this is a comment */";
        let lexer = Lexer::new(input);
        let tokens: Vec<Result<Token, LexError>> = lexer.collect();

        let expected = vec![
            Ok(Token::Select),
            Ok(Token::Asterisk),
            Ok(Token::From),
            Ok(Token::identifier("users")),
            Ok(Token::MultiLineComment(" this is a comment ".to_string())),
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn multi_line_comment_in_sql_with_newline() {
        let input = "SELECT * FROM users /* this is a comment\nwith newline */";
        let lexer = Lexer::new(input);
        let tokens: Vec<Result<Token, LexError>> = lexer.collect();

        let expected = vec![
            Ok(Token::Select),
            Ok(Token::Asterisk),
            Ok(Token::From),
            Ok(Token::identifier("users")),
            Ok(Token::MultiLineComment(
                " this is a comment\nwith newline ".to_string(),
            )),
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn multi_line_comment_in_sql_with_asterisk() {
        let input = "SELECT * FROM users /* this is a comment with * */";
        let lexer = Lexer::new(input);
        let tokens: Vec<Result<Token, LexError>> = lexer.collect();

        let expected = vec![
            Ok(Token::Select),
            Ok(Token::Asterisk),
            Ok(Token::From),
            Ok(Token::identifier("users")),
            Ok(Token::MultiLineComment(
                " this is a comment with * ".to_string(),
            )),
        ];
        assert_eq!(tokens, expected);
    }

    #[ignore]
    #[test]
    fn multi_line_comment_in_sql_with_nested_comment() {
        let input = "SELECT * FROM users /* this is a comment /* with nested */ */";
        let lexer = Lexer::new(input);
        let tokens: Vec<Result<Token, LexError>> = lexer.collect();

        let expected = vec![
            Ok(Token::Select),
            Ok(Token::Asterisk),
            Ok(Token::From),
            Ok(Token::identifier("users")),
            Ok(Token::MultiLineComment(
                " this is a comment /* with nested */ ".to_string(),
            )),
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    #[ignore]
    fn multi_line_comment_in_sql_with_nested_comment_and_newline() {
        let input = "SELECT * FROM users /* this is a comment /* with nested\n */ */";
        let lexer = Lexer::new(input);
        let tokens: Vec<Result<Token, LexError>> = lexer.collect();

        let expected = vec![
            Ok(Token::Select),
            Ok(Token::Asterisk),
            Ok(Token::From),
            Ok(Token::identifier("users")),
            Ok(Token::MultiLineComment(
                " this is a comment /* with nested\n */ ".to_string(),
            )),
        ];
        assert_eq!(tokens, expected);
    }

    #[ignore]
    #[test]
    fn multi_line_comment_in_sql_with_nested_comment_and_newline_and_asterisk() {
        let input = "SELECT * FROM users /* this is a comment /* with nested\n * */ */";
        let lexer = Lexer::new(input);
        let tokens: Vec<Result<Token, LexError>> = lexer.collect();

        let expected = vec![
            Ok(Token::Select),
            Ok(Token::Asterisk),
            Ok(Token::From),
            Ok(Token::identifier("users")),
            Ok(Token::MultiLineComment(
                " this is a comment /* with nested\n * ".to_string(),
            )),
            Err(InvalidCharacter('*')),
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn lex_identifiers() {
        let input = "table1 _table table_2";
        let lexer = Lexer::new(input);
        let tokens: Vec<Result<Token, LexError>> = lexer.collect();
        println!("{:?}", tokens);
        let expected = vec![
            Ok(Token::identifier("table1")),
            Ok(Token::identifier("_table")),
            Ok(Token::identifier("table_2")),
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn identifiers() {
        let input = "table1 column1 PUBLIC.table2 my_col_3 PUBLIC_4.table_5 public.t6able.column_7";
        let lexer = Lexer::new(input);
        let actual: Vec<Result<Token, LexError>> = lexer.collect();
        let expected = vec![
            Ok(Token::identifier("table1")),
            Ok(Token::identifier("column1")),
            Ok(Token::Identifier {
                first_name: "PUBLIC",
                second_name: Some("table2"),
                third_name: None,
            }),
            Ok(Token::identifier("my_col_3")),
            Ok(Token::Identifier {
                first_name: "PUBLIC_4",
                second_name: Some("table_5"),
                third_name: None,
            }),
            Ok(Token::Identifier {
                first_name: "public",
                second_name: Some("t6able"),
                third_name: Some("column_7"),
            }),
        ];
        assert_eq!(actual, expected);
    }
}
