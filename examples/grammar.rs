#![allow(dead_code)]

extern crate nel;

use std::usize;

use nel::tokenize::*;

fn main() {
   let mut builder = Builder::new();

   for f in &[
      expression,
      nary_right,
      nary_operator,
      single,
      parens,
      identifier,
      number,
      number_with_integer,
      number_fractional_only,
      space,
   ] {
      f(&mut builder);
   }

   let (nodes, elements) = builder.destructure();

   nodes
      .iter()
      .enumerate()
      .for_each(|(i, node)| println!("[{:03}] {:?}", i, node));

   println!("---");

   elements
      .iter()
      .enumerate()
      .for_each(|(i, pos)| println!("[{:03}] {}", i, pos));

   println!("---");

   let source = "10 + 10";

   let chars: Vec<_> = source.chars().collect();

   let (toks, _) = tokenize(&chars);

   toks
      .iter()
      .enumerate()
      .for_each(|(i, tok)| println!("[{:03}] {:?}", i, tok));

   println!("================");

   parse_toks(&nodes, &elements, &toks);
}

#[cfg_attr(rustfmt, rustfmt_skip)]
fn expression(b: &mut Builder) {
   b.element(Element::Expression)
      .sequence()
         .reference(Element::Single)
//         .zero_or_more()
         .zero_or_one()
            .reference(Element::NaryRight)
         .end()
      .end();
}

#[cfg_attr(rustfmt, rustfmt_skip)]
fn nary_right(b: &mut Builder) {
   b.element(Element::NaryRight)
      .sequence()
         .reference(Element::Space)
         .reference(Element::NaryOperator)
         .reference(Element::Space)
         .reference(Element::Single)
      .end();
}

#[cfg_attr(rustfmt, rustfmt_skip)]
fn nary_operator(b: &mut Builder) {
   b.element(Element::NaryOperator)
      .choice()
         .tok(Tok::Plus)
         .tok(Tok::Minus)
         .tok(Tok::Asterisk)
         .tok(Tok::Slash)
      .end();
}

#[cfg_attr(rustfmt, rustfmt_skip)]
fn single(b: &mut Builder) {
   b.element(Element::Single)
      .choice()
         .reference(Element::Identifier)
         .reference(Element::Number)
         .reference(Element::Parens)
      .end();
}

#[cfg_attr(rustfmt, rustfmt_skip)]
fn parens(b: &mut Builder) {
   b.element(Element::Parens)
      .sequence()
         .tok(Tok::ParenLeft)
         .reference(Element::Space)
         .reference(Element::Expression)
         .reference(Element::Space)
         .tok(Tok::ParenRight)
      .end();
}

#[cfg_attr(rustfmt, rustfmt_skip)]
fn identifier(b: &mut Builder) {
   b.element(Element::Identifier)
      .sequence()
         .tok(Tok::Identifier)
      .end();
}

#[cfg_attr(rustfmt, rustfmt_skip)]
fn number(b: &mut Builder) {
   b.element(Element::Number)
      .choice()
         .reference(Element::NumberWithInteger)
         .reference(Element::NumberFractionalOnly)
      .end();
}

#[cfg_attr(rustfmt, rustfmt_skip)]
fn number_with_integer(b: &mut Builder) {
   b.element(Element::NumberWithInteger)
      .sequence()
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
fn number_fractional_only(b: &mut Builder) {
   b.element(Element::NumberFractionalOnly)
      .sequence()
         .tok(Tok::FullStop)
         .tok(Tok::Digits)
      .end();
}

#[cfg_attr(rustfmt, rustfmt_skip)]
fn space(b: &mut Builder) {
   b.element(Element::Space)
      .zero_or_one()
         .tok(Tok::Space)
      .end();
}

#[derive(Debug, Clone, Copy)]
enum Element {
   Expression = 0,
   NaryRight,
   NaryOperator,
   Single,
   Parens,
   Identifier,
   Number,
   NumberWithInteger,
   NumberFractionalOnly,
   Space,
}

const ELEMENTS_COUNT: usize = Element::Space as usize + 1;

#[derive(Debug)]
enum Node {
   Element(Element),
   Reference(Element),
   Tok(Tok),
   Sequence(usize),
   Choice(usize),
   ZeroOrOne(usize),
}

struct Builder {
   nodes: Vec<Node>,
   starts: Vec<usize>,
   elements: [usize; ELEMENTS_COUNT],
}

impl Builder {
   fn new() -> Self {
      Builder {
         nodes: Vec::new(),
         starts: Vec::new(),
         elements: [0; ELEMENTS_COUNT],
      }
   }

   fn destructure(self) -> (Vec<Node>, [usize; ELEMENTS_COUNT]) {
      let Self {
         nodes, elements, ..
      } = self;
      (nodes, elements)
   }

   fn element(&mut self, element: Element) -> &mut Self {
      self.elements[element as usize] = self.nodes.len();
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
      self.start(Node::Sequence(0));
      self
   }

   fn choice(&mut self) -> &mut Self {
      self.start(Node::Choice(0));
      self
   }

   fn zero_or_one(&mut self) -> &mut Self {
      self.start(Node::ZeroOrOne(0));
      self
   }

   fn start(&mut self, parent: Node) -> &mut Self {
      self.starts.push(self.nodes.len());
      self.nodes.push(parent);
      self
   }

   fn end(&mut self) -> &mut Self {
      debug_assert!(!self.starts.is_empty());

      let start = self.starts.pop().unwrap();

      debug_assert!(self.nodes.len() > start);

      let end = self.nodes.len();

      match *unsafe { self.nodes.get_unchecked_mut(start) } {
         Node::Sequence(ref mut i) | Node::Choice(ref mut i) | Node::ZeroOrOne(ref mut i) => {
            *i = end
         }
         _ => unreachable!(),
      }

      self
   }
}

macro_rules! dsp_elm {
   ($elm_pos:expr, $path:expr, $what:expr) => {
      println!("[{:03}]{} {}", $elm_pos, "..".repeat($path.len() + 1), $what);
   };
}

macro_rules! dbg_elm {
   ($elm_pos:expr, $path:expr, $what:expr) => {
      println!("[{:03}]{} {:?}", $elm_pos, "..".repeat($path.len() + 1), $what);
   };
}

fn parse_toks(nodes: &[Node], elements: &[usize; ELEMENTS_COUNT], toks: &[Tok]) {
   let mut path: Vec<usize> = Vec::new();

   let mut elm_pos = elements[Element::Expression as usize];

   let mut tok_pos = 0;
   let mut tok_pos_stack: Vec<usize> = Vec::new();

   loop {
      dbg_elm!(elm_pos, path, nodes[elm_pos]);

      match nodes[elm_pos] {
         Node::Element(ref _element) => {
            path.push(elm_pos);
            elm_pos += 1;
         }
         Node::Sequence(end) | Node::ZeroOrOne(end) => {
            debug_assert!(elm_pos < end);
            path.push(elm_pos);
            elm_pos += 1;
         }
         Node::Choice(end) => {
            debug_assert!(elm_pos < end);
            path.push(elm_pos);
            tok_pos_stack.push(tok_pos);
            elm_pos += 1;
         }
         Node::Reference(ref element) => {
            path.push(elm_pos);
            elm_pos = elements[*element as usize];
         }
         Node::Tok(ref tok) => {
            let mut matched = if let Some(tok_src) = toks.get(tok_pos) {
               tok == tok_src
            } else {
               false
            };

            dsp_elm!(elm_pos, path, format!("matched is {}", matched));

            if matched {
               tok_pos += 1;
            }

            elm_pos += 1;

            loop {
               let pop;

               if let Some(pos) = path.last() {
                  dsp_elm!(*pos, path, format!("?? {:?}", nodes[*pos]));

                  match nodes[*pos] {
                     Node::Element(ref _element) => {
                        pop = true;
                     }
                     Node::Sequence(end) => {
                        if !matched {
                           dsp_elm!(pos, path, format!("{} -> {}", elm_pos, end));
                           elm_pos = end;
                           pop = true;
                        } else if elm_pos == end {
                           pop = true;
                        } else {
                           break;
                        }
                     }
                     Node::Choice(end) => {
                        if matched {
                           dsp_elm!(pos, path, format!("{} |> {}", elm_pos, end));
                           elm_pos = end;
                           pop = true;
                           tok_pos_stack.pop();
                        } else if elm_pos == end {
                           pop = true;
                        } else {
                           if let Some(choice_tok_pos) = tok_pos_stack.last() {
                              tok_pos = *choice_tok_pos;
                           } else {
                              unreachable!();
                           }
                           break;
                        }
                     }
                     Node::ZeroOrOne(end) => {
                        if !matched {
                           dsp_elm!(elm_pos, path, format!("{} 0> {}", elm_pos, end));
                           elm_pos = end;
                           pop = true;
                        } else if elm_pos == end {
                           matched = true;
                           pop = true;
                        } else {
                           break;
                        }
                     }
                     Node::Reference(ref _element) => {
                        dsp_elm!(elm_pos, path, format!("{} &> {}", elm_pos, pos + 1));
                        elm_pos = pos + 1;
                        pop = true;
                     }
                     _ => unreachable!()
                  }
               } else {
                  dsp_elm!(elm_pos, path, "DONE");
                  return;
               }

               if pop {
                  path.pop();
               }
            }
         }
      }
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
