// A simple parser for a tiny subset of HTML
// Can parse basic opening and closing tags, and text nodes.
// Not yet supported:

//
// 1. Comments
// 2. Doctypes and processing instructions
// 3. Self closing tags
// 4. Non-well-formed markup
// 5. Character entities

use std::collections::HashMap;

use crate::dom;

pub struct Parser {
    pos: usize, // "usize" is an unsigned integer, similar to "size_t" in C
    input: String,
}

impl Parser {
    // Read the current character without consuming it
    pub fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    // Do the next characters start with the given string?
    pub fn start_with(&self, s: &str) -> bool {
        self.input[self.pos..].starts_with(s)
    }

    // Return true if all input is consumed
    pub fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    // Return the current character and advance self.pos to the next character
    pub fn consume_char(&mut self) -> char {
        let mut iter = self.input[self.pos..].char_indices();
        let (_, cur_char) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        self.pos += next_pos;

        return cur_char;
    }

    /// Consume characters until `test` returns false.
    pub fn consume_while<F>(&mut self, test: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut result = String::new();
        while !self.eof() && test(self.next_char()) {
            result.push(self.consume_char());
        }

        return result;
    }

    // Consume and discard zero or more whitespace characters
    pub fn consume_whitespace(&mut self) {
        self.consume_while(char::is_whitespace);
    }

    // Parse a tag or attribute name.
    pub fn parse_tag_name(&mut self) -> String {
        self.consume_while(|c| match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' => true,
            _ => false,
        })
    }

    // Parse a single node
    pub fn parse_node(&mut self) -> dom::Node {
        match self.next_char() {
            '<' => self.parse_element(),
            _ => self.parse_text(),
        }
    }

    // Parse a text node
    pub fn parse_text(&mut self) -> dom::Node {
        dom::text(self.consume_while(|c| c != '<'))
    }

    // Parse a single element, including its open tag, contents, and closing tag
    pub fn parse_element(&mut self) -> dom::Node {
        //  Opening tag
        assert!(self.consume_char() == '<');
        let tag_name = self.parse_tag_name();
        let attrs = self.parse_attributes();
        assert!(self.consume_char() == '>');

        // Contents.
        let children = self.parse_nodes();

        // Closing tag

        assert!(self.consume_char() == '<');
        assert!(self.consume_char() == '/');
        assert!(self.parse_tag_name() == tag_name);
        assert!(self.consume_char() == '>');

        return dom::elem(tag_name, attrs, children);
    }

    // Parse a single name="value" pair
    pub fn parse_attr(&mut self) -> (String, String) {
        let name = self.parse_tag_name();
        assert!(self.consume_char() == '=');
        let value = self.parse_attr_value();

        return (name, value);
    }

    // Parse a quoted value
    pub fn parse_attr_value(&mut self) -> String {
        let open_quote = self.consume_char();
        assert!(open_quote == '"' || open_quote == '\'');
        let value = self.consume_while(|c| c != open_quote);
        assert!(self.consume_char() == open_quote);
        return value;
    }

    // Parse a list of name="value" pairs, seperated by whitespace
    pub fn parse_attributes(&mut self) -> dom::AttrMap {
        let mut attributes = HashMap::new();

        loop {
            self.consume_whitespace();

            if self.next_char() == '>' {
                break;
            }

            let (name, value) = self.parse_attr();
            attributes.insert(name, value);
        }

        return attributes;
    }

    // Parse a sequence of sibling nodes
    pub fn parse_nodes(&mut self) -> Vec<dom::Node> {
        let mut nodes = Vec::new();

        loop {
            self.consume_whitespace();

            if self.eof() || self.start_with("</") {
                break;
            }
            nodes.push(self.parse_node());
        }

        return nodes;
    }

    // Parse an HTML document and return the root element
    pub fn parse(source: String) -> dom::Node {
        let mut nodes = Parser {
            pos: 0,
            input: source,
        }
        .parse_nodes();

        // if the document contains a root element, just return it. otherwise, create one
        if nodes.len() == 1 {
            nodes.swap_remove(0)
        } else {
            dom::elem("html".to_string(), HashMap::new(), nodes)
        }
    }
}
