#![allow(dead_code)]

extern crate nel;

use nel::tokenize::*;

fn main() {
   let mut builder = Builder::new();

   number(&mut builder);

   number_with_integer(&mut builder);

   number_fractional_only(&mut builder);

   let nodes = builder.destructure();

   nodes
      .iter()
      .enumerate()
      .for_each(|(i, node)| println!("[{:03}] {:?}", i, node));
}

#[cfg_attr(rustfmt, rustfmt_skip)]
fn number(builder: &mut Builder) {
   builder
      .element(Element::Number)
         .choice()
            .reference(Element::NumberWithInteger)
            .reference(Element::NumberFractionalOnly)
         .end()
      .end();
}

#[cfg_attr(rustfmt, rustfmt_skip)]
fn number_with_integer(builder: &mut Builder) {
   builder
      .element(Element::NumberWithInteger)
         .tok(Tok::Digits)
         .zero_or_one()
            .tok(Tok::FullStop)
            .zero_or_one()
               .tok(Tok::Digits)
            .end()
         .end()
      .end();
}

#[cfg_attr(rustfmt, rustfmt_skip)]
fn number_fractional_only(builder: &mut Builder) {
   builder
      .element(Element::NumberFractionalOnly)
         .tok(Tok::FullStop)
         .tok(Tok::Digits)
      .end();
}

#[derive(Debug)]
enum Element {
   Number,
   NumberWithInteger,
   NumberFractionalOnly,
   Space,
   Identifier,
   Single,
   Parens,
   Expression,
   NaryRight,
   Operators,
}

#[derive(Debug)]
enum Node {
   Element(Element),
   Reference(Element),
   Tok(Tok),
   Sequence,
   Choice,
   ZeroOrOne,
   End(usize),
}

struct Builder {
   nodes: Vec<Node>,
   starts: Vec<usize>,
}

impl Builder {
   fn new() -> Self {
      Builder {
         nodes: Vec::new(),
         starts: Vec::new(),
      }
   }

   fn destructure(self) -> Vec<Node> {
      let Self {
         nodes, ..
      } = self;

      nodes
   }

   fn element(&mut self, element: Element) -> &mut Self {
      self.start();
      self.nodes.push(Node::Element(element));
      self
   }

   fn reference(&mut self, element: Element) -> &mut Self {
      self.nodes.push(Node::Reference(element));
      self
   }

   fn tok(&mut self, tok: Tok) -> &mut Self {
      self.nodes.push(Node::Tok(tok));
      self
   }

   fn sequence(&mut self) -> &mut Self {
      self.start();
      self.nodes.push(Node::Sequence);
      self
   }

   fn choice(&mut self) -> &mut Self {
      self.start();
      self.nodes.push(Node::Choice);
      self
   }

   fn zero_or_one(&mut self) -> &mut Self {
      self.start();
      self.nodes.push(Node::ZeroOrOne);
      self
   }

   fn end(&mut self) -> &mut Self {
      debug_assert!(!self.starts.is_empty());
      let start = self.starts.pop().unwrap();
      self.nodes.push(Node::End(start));
      self
   }

   fn start(&mut self) {
      self.starts.push(self.nodes.len());
   }
}

/*

===---===---===---===---===


number = |
   number_with_integer
   number_fractional_only

number_with_integer = (
   one(^digits)
   zero_or_one(
      ^full_stop
      zero_or_one(^digits)
   )
)

number_fractional_only = one(^full_stop) one(^digits)

space = zero_or_one(^space)

identifier = one(^identifier)

single = |
   identifier
   number
   parens

parens = ~
   one(^paren_left)
   space
   expression
   space
   one(^paren_right)

expression = ~
   single
   zero_or_more(nary_right)

nary_right = ~
   space
   one(operators)
   space
   single

operators = |
   ^plus
   ^minus
   ^asterisk
   ^slash


TK = ^digits  | ^full_stop  |
   .          .             .
   0          1


OP = number  |          |
   .         .          .
   0         1


SQ = or-sq-1 | li-sq-2 | 1-tk-1 | 01-sq-4  | 1-tk-2  | 01-tk-1
   .         .         .        .          .         .
   0         1         2        3          4
   ^number
   :
   :
   |         |          |         |
   .         .          .         .
   10        11         12        13


*/
