use super::inifile::*;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, PartialEq)]
pub enum CharResult {
  Char(char),
  Eof,
}

pub struct ParsableString {
  chars: Vec<char>,
  current_index: usize,
  mark_index: usize,
  pos: usize,
  line: usize,
  col: usize,
}

impl ParsableString {
  pub fn new(s: &str) -> ParsableString {
    ParsableString {
      chars: s.chars().collect(),
      current_index: 0,
      mark_index: 0,
      pos: 1,
      line: 1,
      col: 1,
    }
  }

  pub fn done(&self) -> bool {
    self.current_index >= self.chars.len()
  }

  pub fn mark(&mut self) {
    self.mark_index = self.current_index;
  }

  pub fn is_marked(&self) -> bool {
    self.mark_index != self.current_index
  }

  pub fn unmark(&mut self) {
    self.mark_index = self.current_index;
  }

  pub fn get_marked_string(&self) -> String {
    return self.chars[self.mark_index..self.current_index]
      .iter()
      .collect();
  }

  pub fn peek(&self) -> CharResult {
    if self.current_index >= self.chars.len() {
      return CharResult::Eof;
    } else {
      return CharResult::Char(self.chars[self.current_index]);
    }
  }

  pub fn advance(&mut self) -> CharResult {
    if self.current_index < self.chars.len() {
      let c = self.chars[self.current_index];
      self.current_index += 1;
      self.pos += 1;
      self.col += 1;
      if c == '\n' {
        self.line += 1;
        self.col = 1;
      }
      return CharResult::Char(c);
    } else {
      return CharResult::Eof;
    }
  }

  pub fn error(&self, msg: &str) -> String {
    return format!(
      "Parse error: {} at line {}, column {} (char {})",
      msg, self.line, self.col, self.pos
    );
  }

  fn eat_whitespace(&mut self) {
    while let CharResult::Char(c) = self.peek() {
      if c.is_whitespace() {
        self.advance();
      } else {
        break;
      }
    }
  }

  fn warning(&self, c: char) {
    println!(
      "Warning: '{}' detected in key. '{}' deliminiates sections only at the start of a line",
      c, c
    );
  }

  fn parse_key(&mut self) -> Result<Option<String>, String> {
    self.mark();
    while let CharResult::Char(c) = self.peek() {
      if c == '\n' {
        return Err(self.error("Missing equals sign in file"));
      }
      if c == '[' {
        self.warning('[');
      }
      if c == ']' {
        self.warning(']');
      }
      if c == '=' {
        return Ok(Some(self.get_marked_string()));
      }
      self.advance();
    }
    return Ok(None);
  }

  fn parse_value(&mut self) -> Result<Option<String>, String> {
    assert!((self.done() || self.peek() == CharResult::Char('=')));
    self.advance();
    self.mark();
    while let CharResult::Char(c) = self.peek() {
      if c == '\n' || c == '\r' || c == '#' {
        return Ok(Some(self.get_marked_string()));
      }
      self.advance();
    }
    return Ok(None);
  }

  fn parse_section(&mut self) -> Result<Option<IniSection>, String> {
    if self.done() || self.peek() != CharResult::Char('[') {
      return Ok(None);
    }
    self.advance(); // consume '['
    self.mark();
    // capture name of the section
    while let CharResult::Char(c) = self.peek() {
      if c == ']' {
        break;
      }
      if c == '\n' {
        return Err(self.error("Missing closing ']' in section name"));
      }
      self.advance();
    }

    let section_name = self.get_marked_string();
    self.advance(); // consume ']'

    // ensure that the section name is on its own line
    while let CharResult::Char(c) = self.peek() {
      if c == '#' || c == '\n' {
        return Ok(Some(IniSection::new(&section_name)));
      }
      if c.is_whitespace() {
        self.advance();
      } else {
        return Err(self.error("Extraneous characters after section name"));
      }
    }
    return Err(self.error(
      "Empty section! Set environment var INI_PARSER_ALLOW_EMPTY_SECTIONS to true to allow",
    ));
  }

  fn parse_entry(&mut self) -> Result<Option<IniEntry>, String> {
    return match (self.parse_key()?, self.parse_value()?) {
      (Some(k), Some(v)) => Ok(Some(IniEntry { key: k, value: v })),
      (_, _) => Ok(None),
    };
  }

  fn eat_comments(&mut self) {
    while self.peek() == CharResult::Char('#') {
      while self.peek() != CharResult::Char('\n') {
        self.advance();
      }
      self.eat_whitespace();
    }
  }
}

pub fn parse_ini(content: &mut String) -> Result<IniFile, String> {
  // parser location for error reporting
  // iterate over each character
  let mut ps = ParsableString::new(content);

  // create the default section and a list of other sections
  let default_section = IniSection::new_default();
  let mut sections: HashMap<String, IniSection> = HashMap::new();

  // hold the current section
  let mut active_section: IniSection = default_section;

  // parse the file
  while !ps.done() {
    ps.eat_whitespace();
    ps.eat_comments();
    ps.eat_whitespace();
    while let Some(sec) = ps.parse_section()? {
      if !active_section.is_default()
        && active_section.entries.len() == 0
        && !std::env::var("INI_PARSER_ALLOW_EMPTY_SECTIONS").is_ok()
      {
        return Err(ps.error(
          "Empty section! Set environment var INI_PARSER_ALLOW_EMPTY_SECTIONS to true to allow",
        ));
      }
      sections.insert(active_section.name.to_string(), active_section);
      active_section = sec;
      ps.eat_whitespace();
      ps.eat_comments();
      ps.eat_whitespace();
    }

    let entry = ps.parse_entry()?;
    if let Some(e) = entry {
      active_section.entries.insert(e.key.to_string(), e);
    }
  }

  // add the last active section when the file ends
  sections.insert(active_section.name.to_string(), active_section);

  // return the parsed file
  return Ok(IniFile {
    filename: String::from("<missing>"),
    sections: sections,
  });
}

pub fn parse_ini_file(filename: &str) -> Result<IniFile, String> {
  let content = fs::read_to_string(&filename);
  if content.is_err() {
    return Err(format!("Could not read file {}", filename));
  }
  let mut file = parse_ini(&mut content.unwrap())?;
  file.filename = filename.to_owned();
  return Ok(file);
}
