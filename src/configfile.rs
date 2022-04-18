use super::inifile::*;
use super::parsing::*;
use std::collections::HashMap;

// Simpler interface for flat INI files or INI files with only one section.

pub struct ConfigFile {
  inifile: IniFile,
}

impl ConfigFile {
  pub fn new(named: &str) -> ConfigFile {
    ConfigFile {
      inifile: IniFile::new(named),
    }
  }

  pub fn load(filename: &str) -> Result<ConfigFile, String> {
    return Ok(ConfigFile {
      inifile: parse_ini_file(filename)?,
    });
  }

  pub fn value(&self, key: &str) -> Option<&str> {
    self.inifile.peek_value(key)
  }

  pub fn value_in(&self, section: &str, key: &str) -> Option<&str> {
    return Some(self.inifile.peek_section(section)?.peek_value(key)?);
  }

  pub fn set_value(&mut self, key: &str, value: &str) {
    self.set_value_in("<default>", key, value);
  }

  pub fn set_value_in(&mut self, section: &str, key: &str, value: &str) {
    if let Some(section) = self.inifile.get_section(section) {
      if let Some(entry) = section.get_entry(key) {
        entry.value = value.to_string();
      } else {
        section.create_entry(key, value);
      }
    } else {
      let mut section = IniSection::new(section);
      section.create_entry(key, value);
      self.inifile.add_section(section);
    }
  }

  pub fn write(&mut self, filename: &str) -> Result<(), String> {
    return self.inifile.write_to_file(filename, false);
  }
}
