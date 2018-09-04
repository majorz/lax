extern crate lax;

use std::fs::File;
use std::io::prelude::*;
use std::usize;

use lax::indentation::estimate_indentation;
use lax::tokenize::*;

macro_rules! dsp_elm {
   ($elm_pos:expr, $path:expr, $fmt:expr, $($arg:tt)*) => {
      printi!(concat!("{}. ", $fmt), $elm_pos, "..".repeat($path.len()), $($arg)*);
   };
}

macro_rules! printi {
   ($fmt:expr, $pos:expr, $($arg:tt)*) => {
      println!(concat!("[{:03}] ", $fmt), $pos, $($arg)*);
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

   let (instructions, elements) = builder.destructure();

   println!("----------------");

   instructions
      .iter()
      .enumerate()
      .for_each(|(i, instruction)| printi!("{:?}", i, instruction));

   println!("----------------");

   elements
      .iter()
      .enumerate()
      .for_each(|(i, pos)| printi!("{}", i, pos));

   println!("----------------");

   let mut f = File::open("lax/block.lax").expect("file not found");

   let mut source = String::new();
   f.read_to_string(&mut source)
      .expect("something went wrong reading the file");

   let chars: Vec<_> = source.chars().collect();

   println!("Length: {}", chars.len(),);

   println!("----------------");

   let (toks, toks_meta, line_starts) = tokenize(&chars);

   toks
      .iter()
      .enumerate()
      .for_each(|(i, tok)| printi!("{:?}", i, tok));

   println!("----------------");

   toks_meta
      .iter()
      .enumerate()
      .for_each(|(i, tok_meta)| printi!("{:?}", i, tok_meta));

   println!("----------------");

   let module_indentation = estimate_indentation(&toks, &toks_meta, &line_starts);

   println!("Indentation: {}", module_indentation);

   println!("================");

   TokParser::new(
      &instructions,
      &elements,
      &toks,
      &toks_meta,
      module_indentation,
   ).parse();
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
enum Instruction {
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
   instructions: Vec<Instruction>,
   starts: Vec<usize>,
   elements: [usize; ELEMENTS_COUNT],
}

impl Builder {
   fn new() -> Self {
      Builder {
         instructions: Vec::new(),
         starts: Vec::new(),
         elements: [0; ELEMENTS_COUNT],
      }
   }

   fn destructure(self) -> (Vec<Instruction>, [usize; ELEMENTS_COUNT]) {
      let Self {
         instructions,
         elements,
         ..
      } = self;
      (instructions, elements)
   }

   fn element(&mut self, element: Element) -> &mut Self {
      self.elements[element as usize] = self.instructions.len();
      self.instructions.push(Instruction::Element(element));
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

   fn indentation(&mut self, indentation: usize) -> &mut Self {
      self
         .instructions
         .push(Instruction::Indentation(indentation));
      self
   }

   fn reference(&mut self, element: Element) -> &mut Self {
      self.instructions.push(Instruction::Reference(element));
      self
   }

   fn tok(&mut self, tok: Tok) -> &mut Self {
      self.instructions.push(Instruction::Tok(tok));
      self
   }

   fn sequence(&mut self) -> &mut Self {
      self.start(Instruction::Sequence(0));
      self
   }

   fn choice(&mut self) -> &mut Self {
      self.start(Instruction::Choice(0));
      self
   }

   fn zero_or_one(&mut self) -> &mut Self {
      self.start(Instruction::ZeroOrOne(0));
      self
   }

   fn zero_or_more(&mut self) -> &mut Self {
      self.start(Instruction::ZeroOrMore(0));
      self
   }

   fn start(&mut self, parent: Instruction) -> &mut Self {
      self.starts.push(self.instructions.len());
      self.instructions.push(parent);
      self
   }

   fn end(&mut self) -> &mut Self {
      debug_assert!(!self.starts.is_empty());

      let start = self.starts.pop().unwrap();

      debug_assert!(self.instructions.len() > start);

      let end = self.instructions.len();

      match *unsafe { self.instructions.get_unchecked_mut(start) } {
         Instruction::Sequence(ref mut i)
         | Instruction::Choice(ref mut i)
         | Instruction::ZeroOrOne(ref mut i)
         | Instruction::ZeroOrMore(ref mut i) => *i = end,
         _ => unreachable!(),
      }

      self
   }
}

struct TokParser<'b, 't> {
   instructions: &'b [Instruction],
   elements: &'b [usize; ELEMENTS_COUNT],
   toks: &'t [Tok],
   toks_meta: &'t [TokMeta],
   module_indentation: usize,
   current_indentation: usize,
   path: Vec<usize>,
   elm_pos: usize,
   tok_pos: usize,
   tok_pos_stack: Vec<usize>,
   matched: bool,
}

impl<'b, 't> TokParser<'b, 't> {
   fn new(
      instructions: &'b [Instruction],
      elements: &'b [usize; ELEMENTS_COUNT],
      toks: &'t [Tok],
      toks_meta: &'t [TokMeta],
      module_indentation: usize,
   ) -> Self {
      let path: Vec<usize> = Vec::new();
      let elm_pos = elements[Element::Module as usize];

      let tok_pos = 0;
      let tok_pos_stack: Vec<usize> = Vec::new();

      let current_indentation = 0;

      let matched = false;

      TokParser {
         instructions,
         elements,
         toks,
         toks_meta,
         module_indentation,
         current_indentation,
         path,
         elm_pos,
         tok_pos,
         tok_pos_stack,
         matched,
      }
   }

   fn parse(&mut self) {
      loop {
         if self.process_next() && self.try_finalize() {
            break;
         }
      }
   }

   fn process_next(&mut self) -> bool {
      dsp_elm!(
         self.elm_pos,
         self.path,
         "{:?}",
         self.instructions[self.elm_pos]
      );

      match self.instructions[self.elm_pos] {
         Instruction::Element(ref element) => self.process_element(element),
         Instruction::Sequence(end)
         | Instruction::ZeroOrOne(end)
         | Instruction::ZeroOrMore(end)
         | Instruction::Choice(end) => self.process_list(end),
         Instruction::Reference(ref element) => self.process_reference(element),
         Instruction::Tok(ref tok) => self.process_tok(tok),
         Instruction::Indentation(indentation) => self.process_indentation(indentation),
      }
   }

   fn process_element(&mut self, element: &Element) -> bool {
      dsp_elm!(
         self.elm_pos,
         self.path,
         "{:?} [{:03}] >>",
         element,
         self.tok_pos
      );
      if element == &Element::Block {
         self.current_indentation += 1;
      }
      self.path.push(self.elm_pos);
      self.tok_pos_stack.push(self.tok_pos);
      self.elm_pos += 1;

      false
   }

   fn process_list(&mut self, end: usize) -> bool {
      debug_assert!(self.elm_pos < end);
      self.path.push(self.elm_pos);
      self.tok_pos_stack.push(self.tok_pos);
      self.elm_pos += 1;

      false
   }

   fn process_reference(&mut self, element: &Element) -> bool {
      if self.tok_pos == self.toks.len() {
         self.matched = false;
         self.elm_pos += 1;

         true
      } else {
         self.path.push(self.elm_pos);
         self.elm_pos = self.elements[*element as usize];

         false
      }
   }

   fn process_tok(&mut self, tok: &Tok) -> bool {
      self.matched = if let Some(tok_src) = self.toks.get(self.tok_pos) {
         let equal = tok == tok_src;
         if equal {
            dsp_elm!(
               self.elm_pos,
               self.path,
               "TOK {:?} [{:03}]",
               tok_src,
               self.tok_pos
            );

            self.tok_pos += 1;
         } else {
            dsp_elm!(
               self.elm_pos,
               self.path,
               "TOK {:?} [{:03}]",
               tok_src,
               self.tok_pos
            );
         }
         equal
      } else {
         false
      };

      self.elm_pos += 1;

      true
   }

   fn process_indentation(&mut self, indentation: usize) -> bool {
      let spaces = (self.current_indentation + indentation) * self.module_indentation;

      self.matched = if spaces == 0 {
         dsp_elm!(self.elm_pos, self.path, "Indentation {}", 0);
         true
      } else if let Some(tok_src) = self.toks.get(self.tok_pos) {
         let span = self.toks_meta[self.tok_pos].span;
         let equal = tok_src == &Tok::Space && span == spaces;
         if equal {
            dsp_elm!(
               self.elm_pos,
               self.path,
               "Indentation {} [{:03}]",
               span,
               self.tok_pos
            );

            self.tok_pos += 1;
         } else {
            dsp_elm!(
               self.elm_pos,
               self.path,
               "Indentation {} [{:03}]",
               span,
               self.tok_pos
            );
         }

         equal
      } else {
         false
      };

      self.elm_pos += 1;

      true
   }

   fn try_finalize(&mut self) -> bool {
      loop {
         if !self.path.is_empty() {
            let pos = *self.path.last().unwrap();

            dsp_elm!(
               pos,
               self.path,
               "?? {:?} [{:03}] {}",
               self.instructions[pos],
               self.elm_pos,
               self.matched
            );

            let exit = match self.instructions[pos] {
               Instruction::Element(ref element) => self.try_finalize_element(element, pos),
               Instruction::Reference(ref element) => self.try_finalize_reference(element, pos),
               Instruction::Sequence(end) => self.try_finalize_sequence(end, pos),
               Instruction::Choice(end) => self.try_finalize_choice(end, pos),
               Instruction::ZeroOrOne(end) => self.try_finalize_zero_or_one(end, pos),
               Instruction::ZeroOrMore(end) => self.try_finalize_zero_or_more(end, pos),
               _ => unreachable!(),
            };

            if exit {
               return false;
            }
         } else {
            dsp_elm!(self.elm_pos, self.path, "{}", "FI");
            return true;
         }
      }
   }

   fn try_finalize_element(&mut self, element: &Element, pos: usize) -> bool {
      if element == &Element::Block {
         self.current_indentation -= 1;
      }
      self.path.pop();
      let element_tok_pos = self.pop_tok_pos();
      if self.matched {
         dsp_elm!(
            pos,
            self.path,
            "{:?} [{:03}-{:03}] <<",
            element,
            element_tok_pos,
            self.tok_pos
         );
      } else {
         debug_assert!(element_tok_pos == self.tok_pos);
         dsp_elm!(
            pos,
            self.path,
            "{:?} [{:03}-{:03}] <<",
            element,
            element_tok_pos,
            self.tok_pos
         );
      }

      false
   }

   fn try_finalize_reference(&mut self, _element: &Element, pos: usize) -> bool {
      dsp_elm!(self.elm_pos, self.path, "{} &> {}", self.elm_pos, pos + 1);
      self.elm_pos = pos + 1;
      self.path.pop();

      false
   }

   fn try_finalize_sequence(&mut self, end: usize, pos: usize) -> bool {
      if !self.matched {
         dsp_elm!(pos, self.path, "{} -> {}", self.elm_pos, end);
         self.elm_pos = end;
         self.path.pop();
         self.tok_pos = self.pop_tok_pos();

         false
      } else if self.elm_pos == end {
         self.path.pop();
         self.tok_pos_stack.pop();

         false
      } else {
         true
      }
   }

   fn try_finalize_choice(&mut self, end: usize, pos: usize) -> bool {
      if self.matched {
         dsp_elm!(pos, self.path, "{} |> {}", self.elm_pos, end);
         self.elm_pos = end;
         self.path.pop();
         self.tok_pos_stack.pop();

         false
      } else if self.elm_pos == end {
         self.path.pop();
         self.tok_pos = self.pop_tok_pos();

         false
      } else {
         self.tok_pos = self.last_tok_pos();

         true
      }
   }

   fn try_finalize_zero_or_one(&mut self, end: usize, _pos: usize) -> bool {
      if !self.matched {
         dsp_elm!(self.elm_pos, self.path, "{} 0> {}", self.elm_pos, end);
         self.elm_pos = end;
         self.path.pop();
         self.tok_pos = self.pop_tok_pos();
         self.matched = true;

         false
      } else if self.elm_pos == end {
         self.path.pop();
         self.tok_pos_stack.pop();

         false
      } else {
         true
      }
   }

   fn try_finalize_zero_or_more(&mut self, end: usize, pos: usize) -> bool {
      if !self.matched {
         dsp_elm!(self.elm_pos, self.path, "{} *> {}", self.elm_pos, end);
         self.elm_pos = end;
         self.path.pop();
         self.tok_pos = self.pop_tok_pos();
         self.matched = true;

         false
      } else if self.elm_pos == end {
         dsp_elm!(self.elm_pos, self.path, "{} <* {}", self.elm_pos, pos);
         self.elm_pos = pos + 1;
         *self.tok_pos_stack.last_mut().unwrap() = self.tok_pos;

         true
      } else {
         true
      }
   }

   fn last_tok_pos(&self) -> usize {
      if let Some(tok_pos) = self.tok_pos_stack.last() {
         *tok_pos
      } else {
         unreachable!();
      }
   }

   fn pop_tok_pos(&mut self) -> usize {
      if let Some(tok_pos) = self.tok_pos_stack.pop() {
         tok_pos
      } else {
         unreachable!();
      }
   }
}
