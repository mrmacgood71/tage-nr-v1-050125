#[derive(Debug, PartialEq, Clone)]
pub enum LexError {
    InvalidCharacter(char)
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
    Identifier(&'a str),
    StringLiteral(String),
    NumericLiteral(String),
    // Comments
    SingleLineComment(String),
    MultiLineComment(String),
    // Whitespace
    Whitespace,
    // EOF
    EOF,
}

struct Lexer<'a> {
    input: &'a str,
    input_iterator: std::str::Chars<'a>,
    current_position: usize
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token<'a>, LexError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let c = self.skip_whitespace()?;
            match c {
                '*' => return Some(self.single(Token::Asterisk)),
                ',' => return Some(self.single(Token::Comma)),
                '=' => return Some(self.single(Token::Equals)),
                '+' => return Some(self.single(Token::Plus)),
                '-' => return Some(self.single(Token::Minus)),
                '%' => return Some(self.single(Token::Percent)),
                '|' => return Some(self.single(Token::Concat)),


                // may be longer
                '<' => return Some(self.may_be_longer(Token::LessThan)),
                '>' => return Some(self.may_be_longer(Token::GreaterThan)),
                '/' => return Some(self.may_be_longer(Token::Slash)),
                '\'' => return Some(self.may_be_longer(Token::SingleQuote)),
                '"' => return Some(self.literal_started()),



                ';' => return  Some(Ok(Token::Semicolon)),
                '(' => return Some(Ok(Token::OpenParen)),
                ')' => return Some(Ok(Token::CloseParen)),
                '!' => return Some(Ok(Token::NotEquals)),
                c => {
                    if c.is_alphabetic() {
                        return Some(self.keyword_started())
                    } else if c.is_numeric() {
                        return Some(Ok(Token::NumericLiteral(c.to_string())))
                    } else {
                        return Some(Err(LexError::InvalidCharacter(c)))
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
            current_position: 0
        }
    }

    fn skip_whitespace(&mut self) -> Option<char>{
        while let Some(c) = self.input_iterator.next() {
            if !c.is_whitespace() {
                self.current_position += 1;
                return Some(c)
            }
        }
        None
    }

    fn keyword_started(&mut self) -> Result<Token<'a>, LexError> {
        let started_position = self.current_position - 1;
        loop {
            let c = self.input_iterator.next();
            self.current_position += 1;
            if c.is_some() && c.unwrap().is_alphabetic() {
                continue;
            } else {
                break;
            }
        }
        let word = &self.input[started_position..self.current_position - 1];
        let lower_case_word = word.to_lowercase();

        match lower_case_word.as_str() {
            "select" => Ok(Token::Select),
            "from" => Ok(Token::From),
            "where" => Ok(Token::Where),
            "insert" => Ok(Token::Insert),
            "into" => Ok(Token::Into),
            "values" => Ok(Token::Values),
            "update" => Ok(Token::Update),
            "set" => Ok(Token::Set),
            "delete" => Ok(Token::Delete),
            "create" => Ok(Token::Create),
            "table" => Ok(Token::Table),
            "primary" => Ok(Token::Primary),
            "key" => Ok(Token::Key),
            "foreign" => Ok(Token::Foreign),
            "references" => Ok(Token::References),
            "drop" => Ok(Token::Drop),
            "alter" => Ok(Token::Alter),
            "add" => Ok(Token::Add),
            "column" => Ok(Token::Column),
            "constraint" => Ok(Token::Constraint),
            "index" => Ok(Token::Index),
            "join" => Ok(Token::Join),
            "inner" => Ok(Token::Inner),
            "left" => Ok(Token::Left),
            "right" => Ok(Token::Right),
            "full" => Ok(Token::Full),
            "outer" => Ok(Token::Outer),
            "on" => Ok(Token::On),
            "group" => Ok(Token::Group),
            "by" => Ok(Token::By),
            "order" => Ok(Token::Order),
            "asc" => Ok(Token::Asc),
            "desc" => Ok(Token::Desc),
            "union" => Ok(Token::Union),
            "all" => Ok(Token::All),
            "distinct" => Ok(Token::Distinct),
            "limit" => Ok(Token::Limit),
            "offset" => Ok(Token::Offset),
            "having" => Ok(Token::Having),
            "as" => Ok(Token::As),
            "and" => Ok(Token::And),
            "or" => Ok(Token::Or),
            "not" => Ok(Token::Not),
            "null" => Ok(Token::Null),
            "is" => Ok(Token::Is),
            "in" => Ok(Token::In),
            "between" => Ok(Token::Between),
            "like" => Ok(Token::Like),
            "exists" => Ok(Token::Exists),
            "any" => Ok(Token::Any),
            "case" => Ok(Token::Case),
            "when" => Ok(Token::When),
            "then" => Ok(Token::Then),
            "else" => Ok(Token::Else),
            "end" => Ok(Token::End),
            "default" => Ok(Token::Default),
            _ => Ok(Token::Identifier(word))
        }
    }

        fn single(&mut self, token: Token<'a>) -> Result<Token<'a>, LexError> {
        let c = self.input_iterator.next();
        match c {
            Some(c) => {
                self.current_position += 1;
                if c.is_whitespace() {
                    Ok(token)
                } else {
                    Err(LexError::InvalidCharacter(c))
                }
            }
            None => Ok(token)
        }
    }

    fn may_be_longer(&mut self, first: Token<'a>) -> Result<Token<'a>, LexError> {
        let second = self.input_iterator.next();
        self.current_position += 1;
        match first {
            Token::LessThan =>
                match second {
                    Some('=') => Ok(Token::LessThanOrEquals),
                    Some('>') => Ok(Token::NotEquals),
                    Some(' ')  => Ok(Token::LessThan),
                    Some(c) => Err(LexError::InvalidCharacter(c)),
                    None => Ok(Token::LessThan)
                }
            Token::GreaterThan =>
                match second {
                    Some('=') => Ok(Token::GreaterThanOrEquals),
                    Some(' ') => Ok(Token::GreaterThan),
                    Some(c) => Err(LexError::InvalidCharacter(c)),
                    None => Ok(Token::GreaterThan)
                }
            Token::Slash =>
                match second {
                    Some(' ') => Ok(Token::Slash),
                    Some(c) => Err(LexError::InvalidCharacter(c)),
                    None => Ok(Token::Slash)
                },
            _ => Err(LexError::InvalidCharacter(' '))

        }
    }
    fn literal_started(&self) -> Result<Token<'a>, LexError> {
        todo!()
    }
    pub fn tokenize(&self) -> Vec<Token<'a>> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::lexer::LexError::InvalidCharacter;
    use super::*;


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
    // names of table like 'table1' or 'table_1' etc and CREATE/ALTER TABLE parses as keyword
    // string literals is not implemented yet
    // quotes are not implemented yet
    #[test]
    fn test_tokenizer_str() {
        let input = "SELECT * FROM users WHERE age = 1;";
        let lexer = Lexer::new(input);
        let tokens: Vec<_> = lexer.collect();
        let expected = vec![
            Ok(Token::Select),
            Ok(Token::Asterisk),
            Ok(Token::From),
            Ok(Token::Identifier("users")),
            Ok(Token::Where),
            Ok(Token::Identifier("age")),
            Ok(Token::Equals),
            Ok(Token::NumericLiteral("1".to_string())),
            Ok(Token::Semicolon),
            // Token::EOF,
        ];
        ;
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
            Ok(Token::Slash)
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
            Ok(Token::NotEquals)
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn lex_invalid_longer_tokens() {
        let input = "<= >=";
        let lexer = Lexer::new(input);
        let tokens: Vec<Result<Token, LexError>> = lexer.collect();
        println!("{:?}", tokens);
        let expected = vec![
            Ok(Token::LessThanOrEquals),
            Ok(Token::GreaterThanOrEquals),
        ];
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
}


