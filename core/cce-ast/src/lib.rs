mod stream;
mod lexer;

pub use stream::{InputStream, Streamable, InputStreamError};
pub use lexer::{Lexer, Token, LexerError};