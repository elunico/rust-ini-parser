use std::collections::HashMap;
use std::fs;
use std::io::Write;

#[derive(Debug)]
#[allow(dead_code)]
pub struct IniFile {
  pub filename: String,
  pub sections: HashMap<String, IniSection>,
}

#[allow(dead_code)]
impl IniFile {
  pub fn new(filename: &str) -> IniFile {
    IniFile {
      filename: filename.to_owned(),
      sections: HashMap::new(),
    }
  }

  pub fn add_section(&mut self, section: IniSection) {
    // self.sections.push(section);
    self.sections.insert(section.name.to_string(), section);
  }

  pub fn get_section(&mut self, section: &str) -> Option<&mut IniSection> {
    return self.sections.get_mut(section);
  }

  pub fn peek_section(&self, section: &str) -> Option<&IniSection> {
    return self.sections.get(section);
  }

  pub fn get_entry(&mut self, key: &str) -> Option<&mut IniEntry> {
    for section in self.sections.values_mut() {
      if let Some(entry) = section.get_entry(key) {
        return Some(entry);
      }
    }
    return None;
  }

  pub fn peek(&self, key: &str) -> Option<(&IniSection, &str)> {
    for (_, v) in &self.sections {
      if let Some(val) = v.peek_entry(key) {
        return Some((&v, val.value.as_str()));
      }
    }
    return None;
  }

  pub fn peek_value(&self, key: &str) -> Option<&str> {
    for (_, sec) in &self.sections {
      if let Some(val) = sec.peek_entry(key) {
        return Some(val.value.as_str());
      }
    }
    return None;
  }

  pub fn write_to_file(&self, filename: &str, include_empty_sections: bool) -> Result<(), String> {
    let mut file = match fs::OpenOptions::new()
      .write(true)
      .create(true)
      .truncate(true)
      .open(filename)
    {
      Ok(f) => f,
      Err(e) => return Err(format!("{}", e)),
    };

    fn write_entries(file: &mut fs::File, section: &IniSection) {
      for (_, entry) in &section.entries {
        let _ = writeln!(file, "{} = {}", entry.key, entry.value);
      }
    }

    // handle default section separately
    // default section is always written first to the file
    // otherwise the entries would be usurped by the other sections
    if let Some(sec) = self.sections.get("<default>") {
      write_entries(&mut file, sec);
    }

    for (_, section) in &self.sections {
      if section.is_default || (!include_empty_sections && section.entries.len() == 0) {
        continue;
      }
      let _ = writeln!(file, "\n[{}]", section.name);
      write_entries(&mut file, &section);
    }
    Ok(())
  }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct IniSection {
  pub name: String,
  pub entries: HashMap<String, IniEntry>,
  is_default: bool,
}

#[allow(dead_code)]
impl IniSection {
  pub fn new(name: &str) -> IniSection {
    IniSection {
      name: name.to_owned(),
      entries: HashMap::new(),
      is_default: false,
    }
  }

  pub fn new_default() -> IniSection {
    IniSection {
      name: "<default>".to_owned(),
      entries: HashMap::new(),
      is_default: true,
    }
  }

  pub fn is_default(&self) -> bool {
    return self.is_default;
  }

  pub fn add_entry(&mut self, entry: IniEntry) {
    self.entries.insert(entry.key.to_string(), entry);
  }

  pub fn create_entry(&mut self, key: &str, value: &str) {
    self.entries.insert(
      key.to_owned(),
      IniEntry {
        key: key.to_owned(),
        value: value.to_owned(),
      },
    );
  }

  pub fn get_value(&mut self, key: &str) -> Option<&mut str> {
    for (k, entry) in &mut self.entries {
      if k == key {
        return Some(&mut entry.value);
      }
    }
    return None;
  }

  pub fn peek_value(&self, key: &str) -> Option<&str> {
    for (k, entry) in &self.entries {
      if k == key {
        return Some(entry.value.as_str());
      }
    }
    return None;
  }

  pub fn get_entry(&mut self, key: &str) -> Option<&mut IniEntry> {
    for (k, entry) in &mut self.entries {
      if k == key {
        return Some(entry);
      }
    }
    return None;
  }

  pub fn peek_entry(&self, key: &str) -> Option<&IniEntry> {
    for (k, entry) in &self.entries {
      if k == key {
        return Some(entry);
      }
    }
    return None;
  }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct IniEntry {
  pub key: String,
  pub value: String,
}
