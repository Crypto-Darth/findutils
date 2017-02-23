use glob::Pattern;
use glob::PatternError;
use walkdir::DirEntry;

use find::matchers::{Matcher, MatcherIO};

/// This matcher makes a case-sensitive comparison of the name against a
/// shell wildcard pattern. See glob::Pattern for details on the exact
/// syntax.
pub struct NameMatcher {
    pattern: Pattern,
}

impl NameMatcher {
    pub fn new(pattern_string: &str) -> Result<NameMatcher, PatternError> {
        let p = try!(Pattern::new(pattern_string));
        Ok(NameMatcher { pattern: p })
    }

    pub fn new_box(pattern_string: &str) -> Result<Box<Matcher>, PatternError> {
        Ok(Box::new(try!(NameMatcher::new(pattern_string))))
    }
}

impl Matcher for NameMatcher {
    fn matches(&self, file_info: &DirEntry, _: &mut MatcherIO) -> bool {
        return self.pattern.matches(file_info.file_name().to_string_lossy().as_ref());
    }

    fn has_side_effects(&self) -> bool {
        false
    }
}

#[cfg(test)]

mod tests {
    use find::matchers::Matcher;
    use find::matchers::tests::get_dir_entry_for;
    use find::tests::FakeDependencies;
    use super::*;


    #[test]
    fn matching_with_wrong_case_returns_false() {
        let abbbc = get_dir_entry_for("test_data/simple", "abbbc");
        let matcher = NameMatcher::new(&"A*C".to_string()).unwrap();
        let deps = FakeDependencies::new();
        assert!(!matcher.matches(&abbbc, &mut deps.new_side_effects()));
    }

    #[test]
    fn matching_with_right_case_returns_true() {
        let abbbc = get_dir_entry_for("test_data/simple", "abbbc");
        let matcher = NameMatcher::new(&"abb?c".to_string()).unwrap();
        let deps = FakeDependencies::new();
        assert!(matcher.matches(&abbbc, &mut deps.new_side_effects()));
    }

    #[test]
    fn not_matching_returns_false() {
        let abbbc = get_dir_entry_for("test_data/simple", "abbbc");
        let matcher = NameMatcher::new(&"should't match".to_string()).unwrap();
        let deps = FakeDependencies::new();
        assert!(!matcher.matches(&abbbc, &mut deps.new_side_effects()));
    }

    #[test]
    fn cant_create_with_invalid_pattern() {
        let result = NameMatcher::new(&"a**c".to_string());
        assert!(result.is_err());
    }

}
