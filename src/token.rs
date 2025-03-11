use crate::token_type::TokenType;
/*
    Since token_type.rs is already defined at the crate root (via main.rs), you need to refer to it globally using crate:::
    •	Look at the root (crate::)
	•	Find token_type, which was already declared in main.rs
	•	Use TokenType from token_type
 */
#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<String>, //  Since Java’s Object could hold any type (nullable), we use Option<String> in Rust, where None means no value.
    pub line: usize,

    
}

/*
	•	This associates methods with the Token struct.
	•	These methods can only be called on Token instances.
	•	This is similar to defining instance methods in a Java class.
*/
impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Option<String>, line: usize) -> Self {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{} {} {:?}", self.token_type, self.lexeme, self.literal)
    }
}