mod parser_util;
use super::inifile::*;
use parser_util::*;
use std::collections::HashMap;
use std::fs;

pub fn eat_whitespace(chars: &mut Vec<char>, start: &mut usize, location: &mut ParserLocation) {
  let mut s = *start;
  while s < chars.len() && chars[s].is_whitespace() {
    if chars[s] == '\n' {
      inc_line(location);
    } else {
      inc_char(location);
    }
    s += 1;
  }
  *start = s;
}

pub fn parse_key(
  chars: &mut Vec<char>,
  start: &mut usize,
  location: &mut ParserLocation,
) -> Result<Option<String>, String> {
  if *start >= chars.len() {
    return Ok(None);
  }
  let mut s = *start;
  let begin = *start;
  while s < chars.len() && chars[s] != '=' {
    if chars[s] == '\n' {
      return Err(format!(
        "Missing equals sign in file at at line {}, col {} (char {})",
        location.line, location.col, location.pos
      ));
    }
    if chars[s] == '[' {
      println!(
        "Warning: '[' detected in key. '[' deliminiates sections only at the start of a line"
      )
    }
    if chars[s] == ']' {
      println!(
        "Warning: ']' detected in key. ']' deliminiates sections only at the start of a line"
      )
    }
    s += 1;
    inc_char(location);
  }
  *start = s;
  return Ok(Some(chars[begin..s].into_iter().collect()));
}

pub fn parse_value(
  chars: &mut Vec<char>,
  start: &mut usize,
  location: &mut ParserLocation,
) -> Result<Option<String>, String> {
  if *start >= chars.len() {
    return Ok(None);
  }

  let mut s = *start;
  assert!(chars[*start] == '=');

  s += 1;
  inc_char(location);

  let begin = s;

  while s < chars.len() && chars[s] != '=' && chars[s] != '\n' && chars[s] != '#' {
    if chars[s] == '[' {
      println!(
        "Warning: '[' detected in value. '[' deliminiates sections only at the start of a line"
      )
    }
    if chars[s] == ']' {
      println!(
        "Warning: ']' detected in value. ']' deliminiates sections only at the start of a line"
      )
    }
    s += 1;
    inc_char(location);
  }
  *start = s;
  return Ok(Some(chars[begin..s].into_iter().collect()));
}

pub fn parse_section(
  chars: &mut Vec<char>,
  start: &mut usize,
  location: &mut ParserLocation,
) -> Result<Option<IniSection>, String> {
  if *start >= chars.len() || chars[*start] != '[' {
    return Ok(None);
  }

  let mut s = *start + 1;
  inc_char(location);
  let begin = s;

  while s < chars.len() && chars[s] != ']' {
    if chars[s] == '\n' {
      return Err(format!(
        "Unterminated bracket in section name at line {}, col {} (char {})",
        location.line, location.col, location.pos
      ));
    }
    s += 1;
    inc_char(location);
  }
  let name = chars[begin..s].into_iter().collect();
  if s < chars.len() {
    // discard the closing square bracket
    s += 1;
  }
  inc_char(location);

  // eat whitespace until the next line but allow comments
  while s < chars.len() && chars[s] != '\n' && chars[s] != '#' {
    if !chars[s].is_whitespace() {
      return Err(format!(
        "Extraneous characters after section name at line {}, col {} (char {})",
        location.line, location.col, location.pos
      ));
    }
    s += 1;
    inc_char(location);
  }

  *start = s;
  return Ok(Some(IniSection {
    name: name,
    entries: HashMap::new(),
    is_default: false,
  }));
}

pub fn parse_entry(
  chars: &mut Vec<char>,
  start: &mut usize,
  location: &mut ParserLocation,
) -> Result<Option<IniEntry>, String> {
  let key = parse_key(chars, start, location)?;
  let value = parse_value(chars, start, location)?;
  return match (key, value) {
    (Some(k), Some(v)) => Ok(Some(IniEntry { key: k, value: v })),
    (_, _) => Ok(None),
  };
  // return Ok(IniEntry { key, value });
}

pub fn eat_comments(chars: &mut Vec<char>, start: &mut usize, location: &mut ParserLocation) {
  if *start < chars.len() && chars[*start] != '#' {
    return;
  }

  let mut s = *start + 1; // discard hashtag at least
  inc_char(location);

  while s < chars.len() && chars[s] != '\n' {
    s += 1; // ini only has line comments
    inc_char(location);
  }
  inc_line(location);
  *start = s + 1; // skip the newline
}

pub fn parse_ini(content: &mut String) -> Result<IniFile, String> {
  let mut location = ParserLocation {
    pos: 1,
    line: 1,
    col: 1,
  };
  let mut chars: Vec<char> = content.chars().collect();
  let mut idx = 0;

  let default_section = IniSection {
    name: String::from("<default>"),
    entries: HashMap::new(),
    is_default: true,
  };
  let mut sections: HashMap<String, IniSection> = HashMap::new();

  let mut active_section: IniSection = default_section;
  while idx < chars.len() {
    eat_whitespace(&mut chars, &mut idx, &mut location);
    let section = parse_section(&mut chars, &mut idx, &mut location)?;
    if let Some(sec) = section {
      sections.insert(active_section.name.to_string(), active_section);
      active_section = sec;
    }
    eat_whitespace(&mut chars, &mut idx, &mut location);
    eat_comments(&mut chars, &mut idx, &mut location);
    eat_whitespace(&mut chars, &mut idx, &mut location);
    let entry = parse_entry(&mut chars, &mut idx, &mut location)?;
    if let Some(e) = entry {
      active_section.entries.insert(e.key.to_string(), e);
    }
    eat_whitespace(&mut chars, &mut idx, &mut location);
    eat_comments(&mut chars, &mut idx, &mut location);
  }

  sections.insert(active_section.name.to_string(), active_section);
  return Ok(IniFile {
    filename: String::from("<missing>"),
    sections: sections,
  });
}

pub fn parse_ini_file(filename: String) -> Result<IniFile, String> {
  let mut content = fs::read_to_string(&filename).expect("Could not read file");
  let mut file = parse_ini(&mut content)?;
  file.filename = filename;
  return Ok(file);
}
