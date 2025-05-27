use num_bigint::BigUint;
use num_traits::One;

pub fn suggest_alias(name: &str, aliases: &[String]) -> String {
  let suggester = UniqueAliasSuggester::new(aliases);
  suggester.suggest(name)
}

pub trait AliasSuggester {
  fn suggest(&self, name: &str) -> String;
}

pub struct UniqueAliasSuggester<'a> {
  existing_aliases: &'a [String],
}

impl<'a> UniqueAliasSuggester<'a> {
  pub fn new(existing_aliases: &'a [String]) -> Self {
    Self { existing_aliases }
  }

  fn make_unique(&self, alias: String) -> String {
    if !self.existing_aliases.contains(&alias) {
      return alias;
    }

    let mut i = BigUint::one();
    loop {
      let candidate = format!("{alias}{i}");
      if !self.existing_aliases.contains(&candidate) {
        return candidate;
      }

      i += BigUint::one();
    }
  }

  fn create_alias_from_name(
    &self,
    name: &str,
    chars_per_word: usize,
  ) -> String {
    name
      .split_whitespace()
      .flat_map(|word| {
        word
          .chars()
          .filter(char::is_ascii_alphabetic)
          .take(chars_per_word)
          .map(|c| c.to_ascii_lowercase())
      })
      .collect()
  }
}

impl AliasSuggester for UniqueAliasSuggester<'_> {
  fn suggest(&self, name: &str) -> String {
    if name.is_empty() {
      return self.make_unique("buddy".to_owned());
    }

    for chars_per_word in 1..=4 {
      let alias = self.create_alias_from_name(name, chars_per_word);

      if alias.is_empty() {
        return self.make_unique("buddy".to_owned());
      }

      if !self.existing_aliases.contains(&alias) {
        return alias;
      }
    }

    self.make_unique(self.create_alias_from_name(name, 4))
  }
}

#[cfg(test)]
mod tests {
  use std::vec;

  use super::*;

  #[test]
  fn test_empty_name_returns_buddy() {
    let aliases = vec![];
    let suggester = UniqueAliasSuggester::new(&aliases);

    let result = suggester.suggest("");

    assert_eq!(result, "buddy");
  }

  #[test]
  fn test_empty_name_with_existing_buddy_returns_buddy1() {
    let aliases = vec!["buddy".to_string()];
    let suggester = UniqueAliasSuggester::new(&aliases);

    let result = suggester.suggest("");

    assert_eq!(result, "buddy1");
  }

  #[test]
  fn test_simple_name() {
    let aliases = vec![];
    let suggester = UniqueAliasSuggester::new(&aliases);

    let result = suggester.suggest("Peter Pan");

    assert_eq!(result, "pp");
  }

  #[test]
  fn test_name_with_existing_alias() {
    let aliases = vec!["wd".to_string()];
    let suggester = UniqueAliasSuggester::new(&aliases);

    let result = suggester.suggest("Wendy Darling");

    assert_eq!(result, "weda");
  }

  #[test]
  fn test_special_characters() {
    let aliases = vec![];
    let suggester = UniqueAliasSuggester::new(&aliases);

    let result = suggester.suggest("Captain Hook!");

    assert_eq!(result, "ch");
  }

  #[test]
  fn test_all_lengths_occupied() {
    let aliases = vec![
      "ch".to_string(),
      "caho".to_string(),
      "caphoo".to_string(),
      "capthook".to_string(),
    ];
    let suggester = UniqueAliasSuggester::new(&aliases);

    let result = suggester.suggest("Captain Hook");

    assert_eq!(result, "capthook1");
  }

  #[test]
  fn test_non_alphabetic_name() {
    let aliases = vec![];
    let suggester = UniqueAliasSuggester::new(&aliases);

    let result = suggester.suggest("123 @#$");

    assert_eq!(result, "buddy");
  }

  #[test]
  fn test_multiple_words() {
    let aliases = vec![];
    let suggester = UniqueAliasSuggester::new(&aliases);

    let result = suggester.suggest("Slightly Nibs Tootles");

    assert_eq!(result, "snt");
  }

  #[test]
  fn test_create_alias_from_name() {
    let aliases = vec![];
    let suggester = UniqueAliasSuggester::new(&aliases);

    let result = suggester.create_alias_from_name("Peter Pan", 2);

    assert_eq!(result, "pepa");
  }

  #[test]
  fn test_make_unique() {
    let aliases = vec!["neverland".to_string(), "neverland1".to_string()];
    let suggester = UniqueAliasSuggester::new(&aliases);

    let result = suggester.make_unique("neverland".to_string());

    assert_eq!(result, "neverland2");
  }

  #[test]
  fn test_case_insensitivity() {
    let aliases = vec![];
    let suggester = UniqueAliasSuggester::new(&aliases);

    let result = suggester.suggest("TIGER LILY");

    assert_eq!(result, "tl");
  }
}
