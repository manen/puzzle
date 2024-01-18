use iter_read_until::{IntoReader, Read, Reader, StrReader};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Token<'a> {
	Define,
	Ident(&'a str),
	Hex(&'a str),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct TokenizerImpl<'a> {
	readu: StrReader<'a>,
}
impl<'a> Iterator for TokenizerImpl<'a> {
	type Item = Token<'a>;

	fn next(&mut self) -> Option<Self::Item> {
		let word = self.readu.read_until_item(b' ');
		match word {
			Read::Condition(word) | Read::End(word) => match word {
				"#define" => Some(Token::Define),
				word => {
					if word.starts_with("0x") {
						Some(Token::Hex(word))
					} else {
						Some(Token::Ident(word))
					}
				}
			},
			Read::Finished => None,
		}
	}
}

/// what the fuck is this? why are there two borrows?
pub fn tokenizer<'a>(s: &'a &'a str) -> TokenizerImpl<'a> {
	TokenizerImpl { readu: s.reader() }
}
