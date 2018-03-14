extern crate lax;
extern crate termion;

use std::usize;
use std::fs::File;
use std::io::prelude::*;

use termion::color;

use lax::tokenize::*;
use lax::indentation::estimate_indentation;

const C_RESET: color::Fg<color::Reset> = color::Fg(color::Reset);
const C_HIGHLIGHT: color::Fg<color::Rgb> = color::Fg(color::Rgb(255, 116, 79));
const C_DOTS: color::Fg<color::Rgb> = color::Fg(color::Rgb(115, 55, 100));
const C_INDEX: color::Fg<color::Rgb> = color::Fg(color::Rgb(177, 65, 149));
const C_PUNCT: color::Fg<color::Rgb> = color::Fg(color::Rgb(216, 46, 0));
const C_TEXT: color::Fg<color::Rgb> = color::Fg(color::Rgb(121, 166, 169));

macro_rules! dsp_elm {
   ($elm_pos:expr, $path:tt, $fmt:expr, $($arg:tt)*) => {
      printi!(concat!("{}.{} {}", $fmt, "{}"), $elm_pos, C_DOTS, "..".repeat($path.len()), C_TEXT, $($arg)*, C_RESET);
   };
}

macro_rules! printi {
   ($fmt:expr, $pos:tt, $($arg:tt)*) => {
      println!(concat!("{}[{}{:03}{}]{} ", $fmt), C_DOTS, C_INDEX, $pos, C_DOTS, C_RESET, $($arg)*);
   };
}

fn main() {
   let mut builder = Builder::new();

   for f in &[
      module,
      if_,
      block,
      statement,
      expression,
      nary_right,
      nary_operator,
      nullary,
      parens,
      not,
      identifier,
      boolean,
      number,
   ] {
      f(&mut builder);
   }

   let (nodes, elements) = builder.destructure();

   println!("{}----------------{}", C_DOTS, C_RESET);

   nodes
      .iter()
      .enumerate()
      .for_each(|(i, node)| printi!("{}{:?}{}", i, C_TEXT, node, C_RESET));

   println!("{}----------------{}", C_DOTS, C_RESET);

   elements
      .iter()
      .enumerate()
      .for_each(|(i, pos)| printi!("{}{}{}", i, C_TEXT, pos, C_RESET));

   println!("{}----------------{}", C_DOTS, C_RESET);

   let mut f = File::open("lax/block.lax").expect("file not found");

   let mut source = String::new();
   f.read_to_string(&mut source)
      .expect("something went wrong reading the file");

   let chars: Vec<_> = source.chars().collect();

   println!(
      "{}Length: {}{}{}",
      C_TEXT,
      C_HIGHLIGHT,
      chars.len(),
      C_RESET
   );

   println!("{}----------------{}", C_DOTS, C_RESET);

   let (toks, toks_meta) = tokenize(&chars);

   toks
      .iter()
      .enumerate()
      .for_each(|(i, tok)| printi!("{}{:?}{}", i, C_TEXT, tok, C_RESET));

   println!("{}----------------{}", C_DOTS, C_RESET);

   toks_meta
      .iter()
      .enumerate()
      .for_each(|(i, tok_meta)| printi!("{}{:?}{}", i, C_TEXT, tok_meta, C_RESET));

   println!("{}----------------{}", C_DOTS, C_RESET);

   let step = match estimate_indentation(&toks, &toks_meta) {
      Some(step) => step,
      None => 1,
   };

   println!("{}Indentation: {}{}{}", C_TEXT, C_HIGHLIGHT, step, C_RESET);

   println!("{}================{}", C_DOTS, C_RESET);

   parse_toks(&nodes, &elements, &toks, &toks_meta, step);
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Element {
   Module = 0,
   If,
   Block,
   Statement,
   Expression,
   NaryRight,
   NaryOperator,
   Nullary,
   Parens,
   Not,
   Identifier,
   Boolean,
   Number,
}

const ELEMENTS_COUNT: usize = Element::Number as usize + 1;

#[cfg_attr(rustfmt, rustfmt_skip)]
fn module(b: &mut Builder) {
   b.element(Element::Module)
      .zero_or_more()
         .choice()
            .reference(Element::Statement)
            .tok(Tok::LineEnd)
         .end()
      .end();
}

#[cfg_attr(rustfmt, rustfmt_skip)]
fn if_(b: &mut Builder) {
   b.element(Element::If)
      .sequence()
         .tok(Tok::If)
         .skip_space()
         .reference(Element::Expression)
         .tok(Tok::LineEnd)
         .reference(Element::Block)
      .end();
}

#[cfg_attr(rustfmt, rustfmt_skip)]
fn block(b: &mut Builder) {
   b.element(Element::Block)
      .sequence()
         .zero_or_more()
            .tok(Tok::LineEnd)
         .end()
         .reference(Element::Statement)
         .zero_or_more()
            .choice()
               .reference(Element::Statement)
               .tok(Tok::LineEnd)
            .end()
         .end()
      .end();
}

#[cfg_attr(rustfmt, rustfmt_skip)]
fn statement(b: &mut Builder) {
   b.element(Element::Statement)
      .sequence()
         .indentation(0)
         .choice()
            .reference(Element::If)
            .sequence()
               .reference(Element::Expression)
               .tok(Tok::LineEnd)
            .end()
         .end()
      .end();
}

#[cfg_attr(rustfmt, rustfmt_skip)]
fn expression(b: &mut Builder) {
   b.element(Element::Expression)
      .sequence()
         .reference(Element::Nullary)
         .zero_or_more()
            .reference(Element::NaryRight)
         .end()
      .end();
}

#[cfg_attr(rustfmt, rustfmt_skip)]
fn nary_right(b: &mut Builder) {
   b.element(Element::NaryRight)
      .sequence()
         .skip_space()
         .reference(Element::NaryOperator)
         .skip_space()
         .reference(Element::Nullary)
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
         .tok(Tok::And)
         .tok(Tok::Or)
      .end();
}

#[cfg_attr(rustfmt, rustfmt_skip)]
fn nullary(b: &mut Builder) {
   b.element(Element::Nullary)
      .choice()
         .reference(Element::Parens)
         .reference(Element::Not)
         .reference(Element::Identifier)
         .reference(Element::Boolean)
         .reference(Element::Number)
      .end();
}

#[cfg_attr(rustfmt, rustfmt_skip)]
fn not(b: &mut Builder) {
   b.element(Element::Not)
      .sequence()
         .tok(Tok::Not)
         .skip_space()
         .reference(Element::Nullary)
      .end();
}

#[cfg_attr(rustfmt, rustfmt_skip)]
fn parens(b: &mut Builder) {
   b.element(Element::Parens)
      .sequence()
         .tok(Tok::ParenLeft)
         .skip_space()
         .reference(Element::Expression)
         .skip_space()
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
fn boolean(b: &mut Builder) {
   b.element(Element::Boolean)
      .choice()
         .tok(Tok::True)
         .tok(Tok::False)
      .end();
}

#[cfg_attr(rustfmt, rustfmt_skip)]
fn number(b: &mut Builder) {
   b.element(Element::Number)
      .choice()
         .sequence()
            .tok(Tok::Digits)
            .zero_or_one()
               .tok(Tok::FullStop)
               .zero_or_one()
                  .tok(Tok::Digits)
               .end()
            .end()
         .end()
         .sequence()
            .tok(Tok::FullStop)
            .tok(Tok::Digits)
         .end()
      .end();
}

#[derive(Debug)]
enum Node {
   Element(Element),
   Reference(Element),
   Tok(Tok),
   Sequence(usize),
   Choice(usize),
   ZeroOrOne(usize),
   ZeroOrMore(usize),
   Indentation(usize),
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

   #[cfg_attr(rustfmt, rustfmt_skip)]
   fn skip_space(&mut self) -> &mut Self {
      self
         .zero_or_one()
            .choice()
               .tok(Tok::Space)
               .sequence()
                  .tok(Tok::LineEnd)
                  .indentation(2)
               .end()
            .end()
         .end()
   }

   fn indentation(&mut self, offset: usize) -> &mut Self {
      self.nodes.push(Node::Indentation(offset));
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

   fn zero_or_more(&mut self) -> &mut Self {
      self.start(Node::ZeroOrMore(0));
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
         Node::Sequence(ref mut i)
         | Node::Choice(ref mut i)
         | Node::ZeroOrOne(ref mut i)
         | Node::ZeroOrMore(ref mut i) => *i = end,
         _ => unreachable!(),
      }

      self
   }
}

#[cfg_attr(feature = "cargo-clippy", allow(cyclomatic_complexity))]
fn parse_toks(
   nodes: &[Node],
   elements: &[usize; ELEMENTS_COUNT],
   toks: &[Tok],
   toks_meta: &[TokMeta],
   step: usize,
) {
   let mut path: Vec<usize> = Vec::new();

   let mut elm_pos = elements[Element::Module as usize];

   let mut tok_pos = 0;
   let mut tok_pos_stack: Vec<usize> = Vec::new();

   let mut indentation = 0;

   loop {
      dsp_elm!(elm_pos, path, "{:?}", nodes[elm_pos]);

      let mut matched;

      match nodes[elm_pos] {
         Node::Element(ref element) => {
            dsp_elm!(
               elm_pos,
               path,
               "{}{:?} [{:03}] >>",
               C_PUNCT,
               element,
               tok_pos
            );
            if element == &Element::Block {
               indentation += 1;
            }
            path.push(elm_pos);
            tok_pos_stack.push(tok_pos);
            elm_pos += 1;
            continue;
         }
         Node::Sequence(end) | Node::ZeroOrOne(end) | Node::ZeroOrMore(end) | Node::Choice(end) => {
            debug_assert!(elm_pos < end);
            path.push(elm_pos);
            tok_pos_stack.push(tok_pos);
            elm_pos += 1;
            continue;
         }
         Node::Reference(ref element) => {
            if tok_pos == toks.len() {
               matched = false;
               elm_pos += 1;
            } else {
               path.push(elm_pos);
               elm_pos = elements[*element as usize];
               continue;
            }
         }
         Node::Tok(ref tok) => {
            matched = if let Some(tok_src) = toks.get(tok_pos) {
               let equal = tok == tok_src;
               if equal {
                  dsp_elm!(
                     elm_pos,
                     path,
                     "{}TOK {:?} [{:03}]",
                     C_HIGHLIGHT,
                     tok_src,
                     tok_pos
                  );

                  tok_pos += 1;
               } else {
                  dsp_elm!(elm_pos, path, "TOK {:?} [{:03}]", tok_src, tok_pos);
               }
               equal
            } else {
               false
            };

            elm_pos += 1;
         }
         Node::Indentation(offset) => {
            let spaces = (indentation + offset) * step;

            matched = if spaces == 0 {
               dsp_elm!(
                  elm_pos,
                  path,
                  "{}Indentation 0",
                  C_HIGHLIGHT
               );
               true
            } else if let Some(tok_src) = toks.get(tok_pos) {
               let span = toks_meta[tok_pos].span;
               let equal = tok_src == &Tok::Space && span == spaces;
               if equal {
                  dsp_elm!(
                     elm_pos,
                     path,
                     "{}Indentation {} [{:03}]",
                     C_HIGHLIGHT,
                     span,
                     tok_pos
                  );

                  tok_pos += 1;
               } else {
                  dsp_elm!(elm_pos, path, "Indentation {} [{:03}]", span, tok_pos);
               }

               equal
            } else {
               false
            };

            elm_pos += 1;
         }
      }

      loop {
         if !path.is_empty() {
            let pos = *path.last().unwrap();

            dsp_elm!(
               pos,
               path,
               "?? {:?} [{:03}] {}",
               nodes[pos],
               elm_pos,
               matched
            );

            match nodes[pos] {
               Node::Element(ref element) => {
                  if element == &Element::Block {
                     indentation -= 1;
                  }
                  path.pop();
                  let element_tok_pos = pop_tok_pos(&mut tok_pos_stack);
                  if matched {
                     dsp_elm!(
                        pos,
                        path,
                        "{}{:?} {}[{:03}-{:03}] <<",
                        C_HIGHLIGHT,
                        element,
                        C_PUNCT,
                        element_tok_pos,
                        tok_pos
                     );
                  } else {
                     debug_assert!(element_tok_pos == tok_pos);
                     dsp_elm!(
                        pos,
                        path,
                        "{}{:?} [{:03}-{:03}] <<",
                        C_PUNCT,
                        element,
                        element_tok_pos,
                        tok_pos
                     );
                  }
               }
               Node::Sequence(end) => {
                  if !matched {
                     dsp_elm!(pos, path, "{} -> {}", elm_pos, end);
                     elm_pos = end;
                     path.pop();
                     tok_pos = pop_tok_pos(&mut tok_pos_stack);
                  } else if elm_pos == end {
                     path.pop();
                     tok_pos_stack.pop();
                  } else {
                     break;
                  }
               }
               Node::Choice(end) => {
                  if matched {
                     dsp_elm!(pos, path, "{} |> {}", elm_pos, end);
                     elm_pos = end;
                     path.pop();
                     tok_pos_stack.pop();
                  } else if elm_pos == end {
                     path.pop();
                     tok_pos = pop_tok_pos(&mut tok_pos_stack);
                  } else {
                     tok_pos = last_tok_pos(&tok_pos_stack);
                     break;
                  }
               }
               Node::ZeroOrOne(end) => {
                  if !matched {
                     dsp_elm!(elm_pos, path, "{} 0> {}", elm_pos, end);
                     elm_pos = end;
                     path.pop();
                     tok_pos = pop_tok_pos(&mut tok_pos_stack);
                     matched = true;
                  } else if elm_pos == end {
                     path.pop();
                     tok_pos_stack.pop();
                  } else {
                     break;
                  }
               }
               Node::ZeroOrMore(end) => {
                  if !matched {
                     dsp_elm!(elm_pos, path, "{} *> {}", elm_pos, end);
                     elm_pos = end;
                     path.pop();
                     tok_pos = pop_tok_pos(&mut tok_pos_stack);
                     matched = true;
                  } else if elm_pos == end {
                     dsp_elm!(elm_pos, path, "{} <* {}", elm_pos, pos);
                     elm_pos = pos + 1;
                     *tok_pos_stack.last_mut().unwrap() = tok_pos;
                     break;
                  } else {
                     break;
                  }
               }
               Node::Reference(ref _element) => {
                  dsp_elm!(elm_pos, path, "{} &> {}", elm_pos, pos + 1);
                  elm_pos = pos + 1;
                  path.pop();
               }
               _ => unreachable!(),
            }
         } else {
            dsp_elm!(elm_pos, path, "{}", "FI");
            return;
         }
      }
   }
}

fn last_tok_pos(tok_pos_stack: &[usize]) -> usize {
   if let Some(tok_pos) = tok_pos_stack.last() {
      *tok_pos
   } else {
      unreachable!();
   }
}

fn pop_tok_pos(tok_pos_stack: &mut Vec<usize>) -> usize {
   if let Some(tok_pos) = tok_pos_stack.pop() {
      tok_pos
   } else {
      unreachable!();
   }
}
