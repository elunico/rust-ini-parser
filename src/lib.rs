pub mod configfile;
pub mod inifile;
pub mod parsing;

#[cfg(test)]
mod tests {
    use super::inifile::*;
    #[test]
    fn create_blank_file() {
        let inifile = IniFile::new("test");

        assert_eq!(inifile.filename, "test");
        assert_eq!(inifile.sections.len(), 0);
    }

    #[test]
    fn create_one_entry() {
        let mut inifile = IniFile::new("test");
        inifile.add_section(IniSection::new("test"));
        if let Some(sec) = inifile.get_section("test") {
            sec.create_entry("test", "test")
        };

        assert_eq!(inifile.sections.len(), 1);
        assert_eq!(inifile.sections.get("test").unwrap().entries.len(), 1);
        assert_eq!(
            inifile
                .sections
                .get("test")
                .unwrap()
                .entries
                .get("test")
                .unwrap()
                .value,
            "test"
        );
    }

    #[test]
    fn create_two_entries() {
        let mut inifile = IniFile::new("test");
        inifile.add_section(IniSection::new("test"));
        if let Some(sec) = inifile.get_section("test") {
            sec.create_entry("test", "test")
        };
        if let Some(sec) = inifile.get_section("test") {
            sec.create_entry("test2", "test2")
        };

        assert_eq!(inifile.sections.len(), 1);
        assert_eq!(inifile.sections.get("test").unwrap().entries.len(), 2);
        assert_eq!(
            inifile
                .sections
                .get("test")
                .unwrap()
                .entries
                .get("test")
                .unwrap()
                .value,
            "test"
        );
    }
}
