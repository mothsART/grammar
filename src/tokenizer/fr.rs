struct Splitter;

impl Splitter {
    fn is_match(c: char) -> bool {
        match c {
            ' ' | ',' | '.' | '!' | '?' | ';' | '\'' |  '"'
            | ':' | '\t' | '\n' | '(' | ')' | '-' => true,
            _ => false
        }
    }
}

pub fn tokenize(text: &str) -> Vec<&str> {
  text.split(Splitter::is_match)
      .filter(|s| s.is_empty())
      .collect()
}

#[cfg(all(feature = "unstable", test))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
      assert_eq!(tokenize("hello, world!"), vec!["hello", "world"]);
      assert_eq!(tokenize("My dog has fleas."), vec!["My", "dog", "has", "fleas"]);
    }
}

#[cfg(all(feature = "unstable", test))]
mod bench {
    #[bench]
    fn bench_tokenize() {
      b.iter(|| (tokenize("hello, world!"), vec!["hello", "world"]));
    }
}