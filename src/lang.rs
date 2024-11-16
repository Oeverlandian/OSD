#[derive(Debug, PartialEq, Eq)]
pub enum TokenKind {
    Inputs,
    Outputs,
    And,
    Or,
    Not,
    Identifier(String),
    In,
    Out,
    Comma,
    ParenOpen,
    ParenClose,
    Newline,
    EOF,
}

/// The main lexer struct that handles tokenization of source code
#[derive(Debug)]
pub struct Lexer {
    source: String,             // The complete source code to be tokenized
    position: usize,            // Current position in the source code
    line: usize,                // Current line number (for error reporting)
    column: usize,              // Current column number (for error reporting)
    current_char: Option<char>, // Current character being processed
}

/// Represents a location in the source code for error reporting
#[derive(Debug, Clone, Copy)]
pub struct Location {
    pub line: usize,        // Line number (1-based)
    pub column: usize,      // Column number (1-based)
}

impl Location {
    /// Creates a new Location instance
    fn new(line: usize, column: usize) -> Self {
        Location { line, column } 
    }
}

impl Lexer {
    /// Creates a new Lexer instance with the given source code
    pub fn new(source: String) -> Self {
        let mut lexer = Lexer {
            source,
            position: 0,
            line: 1,
            column: 1,
            current_char: None,
        };
        lexer.current_char = lexer.source.chars().nth(0);
        lexer
    }

    /// Returns the current location in the source code
    fn get_location(&self) -> Location {
        Location::new(self.line, self.column)
    }

    /// Advances the lexer position by one character
    /// Updates line and column numbers appropriately
    fn advance(&mut self) {
        if let Some(c) = self.current_char {
            if c == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
        }
        self.position += 1;
        self.current_char = self.source.chars().nth(self.position);
    }

    /// Skips over whitespace characters in the source code
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char {
            if c == '\n' {
                break; // Don't skip newlines
            } else if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    /// Main tokenization function that returns the next token from the source
    /// Returns Result<TokenKind, LexerError> to handle potential errors
    pub fn get_next_token(&mut self) -> Result<TokenKind, LexerError> {
        self.skip_whitespace();
        let location = self.get_location();

        if self.current_char.is_none() {
            return Ok(TokenKind::EOF);
        }

        match self.current_char {
            Some(c) if c.is_alphabetic() => Ok(self.identifier()),
            Some(',') => { self.advance(); Ok(TokenKind::Comma)},
            Some('(') => { self.advance(); Ok(TokenKind::ParenOpen)},
            Some(')') => { self.advance(); Ok(TokenKind::ParenClose)},
            Some('\n') => { // Newlines work now
                self.advance();
                Ok(TokenKind::Newline)
            },
            Some(c) => {
                self.advance();
                Err(LexerError::UnexpectedCharacter(c, location))
            },
            None => Ok(TokenKind::EOF),
        }
    }

    /// Processes identifiers and keywords
    /// Returns either a keyword token or an identifier token
    pub fn identifier(&mut self) -> TokenKind {
        let mut id_str = String::new();

        while let Some(c) = self.current_char {
            if c.is_alphanumeric() || c == '_' {
                id_str.push(c);
                self.advance();
            } else {
                break;
            }
        }

        // Match against known keywords, return Identifier if not a keyword
        match id_str.to_uppercase().as_str() {
            "INPUTS" => TokenKind::Inputs,
            "OUTPUTS" => TokenKind::Outputs,
            "AND" => TokenKind::And,
            "OR" => TokenKind::Or,
            "NOT" => TokenKind::Not,
            "IN" => TokenKind::In,
            "OUT" => TokenKind::Out,
            _ => TokenKind::Identifier(id_str),
        }
    }
}

/// Represents possible errors that can occur during lexical analysis 
#[derive(Debug)]
pub enum LexerError {
    UnexpectedCharacter(char, Location),
    UnterminatedStringLiteral(Location),
    UnterminatedCharLiteral(Location),
    InvalidNumberFormat(Location),
    UnexpectedToken(String, Location),
    UnterminatedMultilineComment(Location),
}

/// Implements Display trait for LexerError to provide human-readable error messages
impl std::fmt::Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexerError::UnexpectedCharacter(c, loc) => 
                write!(f, "Unexpected character '{}' at line {}, column {}", c, loc.line, loc.column),
            LexerError::UnterminatedStringLiteral(loc) => 
                write!(f, "Unterminated string literal at line {}, column {}", loc.line, loc.column),
            LexerError::UnterminatedCharLiteral(loc) => 
                write!(f, "Unterminated character literal at line {}, column {}", loc.line, loc.column),
            LexerError::InvalidNumberFormat(loc) => 
                write!(f, "Invalid number format at line {}, column {}", loc.line, loc.column),
            LexerError::UnexpectedToken(token, loc) => 
                write!(f, "Unexpected token '{}' at line {}, column {}", token, loc.line, loc.column),
            LexerError::UnterminatedMultilineComment(loc) =>
                write!(f, "Unterminated multi-line comment at line {}, column {}", loc.line, loc.column),
        }
    }
}

