use std::cell::Ref;
use super::GroupType;
use super::{ASTElement, Node, Tree}; 

/// Struct for generation of HTML strings.
pub struct HTMLConstructor {
	output_string: String,
	pretty_print: bool,
}
impl HTMLConstructor {
	/// Creates a new HTMLConstructor.
	pub fn new (out_len: usize, pretty_print: bool) -> HTMLConstructor {
		let output_string = String::with_capacity(out_len + out_len/2);
		HTMLConstructor {
			output_string, 
			pretty_print,
		}
	}

	/// Generates an HTML string from an ASTElement
	pub fn construct(&mut self, ast: &Tree<Node<ASTElement>>) -> String {

		self.output_string.clone()
	}

	
}