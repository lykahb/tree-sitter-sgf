//! This crate provides SGF language support for the [tree-sitter][] parsing
//! library.
//!
//! Typically, you will use the [LANGUAGE][] constant to add this language to a
//! tree-sitter [Parser][], and then parse SGF source code:
//!
//! ```
//! let code = "(;GM[1]FF[4]SZ[19];B[pd];W[dd])";
//! let mut parser = tree_sitter::Parser::new();
//! let language = tree_sitter_sgf::LANGUAGE;
//! parser
//!     .set_language(&language.into())
//!     .expect("Error loading SGF parser");
//! let tree = parser.parse(code, None).unwrap();
//! assert!(!tree.root_node().has_error());
//! ```
//!
//! [Parser]: https://docs.rs/tree-sitter/*/tree_sitter/struct.Parser.html
//! [tree-sitter]: https://tree-sitter.github.io/tree-sitter/

use tree_sitter_language::LanguageFn;

extern "C" {
    fn tree_sitter_sgf() -> *const ();
}

/// The tree-sitter [`LanguageFn`][LanguageFn] for this grammar.
///
/// [LanguageFn]: https://docs.rs/tree-sitter-language/*/tree_sitter_language/struct.LanguageFn.html
pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_sgf) };

/// The content of the [`node-types.json`][] file for this grammar.
///
/// [`node-types.json`]: https://tree-sitter.github.io/tree-sitter/using-parsers#static-node-types
pub const NODE_TYPES: &str = include_str!("../../src/node-types.json");

/// The syntax highlighting query for this language.
pub const HIGHLIGHTS_QUERY: &str = include_str!("../../queries/sgf/highlights.scm");

#[cfg(test)]
mod tests {
    #[test]
    fn can_load_and_parse_sgf() {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&super::LANGUAGE.into())
            .expect("Error loading SGF parser");

        let tree = parser
            .parse("(;GM[1]FF[4]SZ[19];B[pd];W[dd])", None)
            .expect("Failed to parse SGF");
        assert!(!tree.root_node().has_error());
    }
}
