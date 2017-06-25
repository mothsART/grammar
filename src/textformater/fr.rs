extern crate regex;

use std::collections::HashMap;
use self::regex::Regex;

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum TextFormaterLine {
    StartOfParagraph,
    EndOfParagraph,
    BetweenWords,
    BeforePunctuation,
    WithinParenthesis,
    WithinSquareBrackets,
    WithinQuotationMarks
}

impl TextFormaterLine {
    pub fn as_test(&self) -> &'static str {
        match *self {
            TextFormaterLine::StartOfParagraph     => "    un texte avec trop d'espaces au début",
            TextFormaterLine::EndOfParagraph       => "un texte avec trop d'espaces à la fin    ",
            TextFormaterLine::BetweenWords         => "un texte   avec    trop  d'espaces",
            TextFormaterLine::BeforePunctuation    => "un texte avec trop d'espaces dans la ponctuation  .  ;  !  ;  …",
            TextFormaterLine::WithinParenthesis    => "(  un texte avec trop d'espaces à l'intérieur des parenthèses  )",
            TextFormaterLine::WithinSquareBrackets => "[  un texte avec trop d'espaces à l'intérieur des crochets   ]",
            TextFormaterLine::WithinQuotationMarks => "“ un texte avec trop d'espaces à l'intérieur des guillements ”",
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum TextFormaterGroups {
    SurnumerarySpaces,
    MissingSpaces,
    NonBreakingSpaces,
    Erase,
    TypographicSigns,
    Misc,
    /*MissingHyphens,
    MissingApostrophes*/
}


impl TextFormaterGroups {
    pub fn as_str(&self) -> &'static str {
        match *self {
            TextFormaterGroups::SurnumerarySpaces  => "Espaces surnuméraires",
            TextFormaterGroups::MissingSpaces      => "Espaces manquants",
            TextFormaterGroups::NonBreakingSpaces  => "Espaces insécables",
            TextFormaterGroups::Erase              => "Suppressions",
            TextFormaterGroups::TypographicSigns   => "Signes typographiques",
            TextFormaterGroups::Misc               => "Divers",
            /*TextFormaterGroups::MissingHyphens     => "",
            TextFormaterGroups::MissingApostrophes => "",*/
        }
    }
}

pub struct Rule {
    pub regex       :  Option<Regex>,
    pub callback    : Option<Callback>,
    pub replace_text: &'static str
}

pub struct TextFormat {
    pub short:   &'static str,
    pub rules:   Vec<Rule>,
}


pub struct TextFormaterGroup {
    pub name: &'static str,
    pub lines: HashMap<TextFormaterLine, TextFormat>
}

#[derive(Default)]
pub struct TextFormater {
    pub groups: HashMap<TextFormaterGroups, TextFormaterGroup>
}

impl TextFormater {
    pub fn new() -> TextFormater {
        let mut groups: HashMap<TextFormaterGroups, TextFormaterGroup> = HashMap::new();
        let mut surnumerary_spaces: HashMap<TextFormaterLine, TextFormat> = HashMap::new();
        surnumerary_spaces.insert(
            TextFormaterLine::StartOfParagraph,
             TextFormat {
                short:   "En début de paragraphe",
                rules:   vec![
                    Rule {
                        regex:        None,
                        callback:     Some(ltrim),
                        replace_text: ""
                    }
                ]
            }
        );
        surnumerary_spaces.insert(
            TextFormaterLine::BetweenWords,
             TextFormat {
                short:   "Entre les mots",
                rules:   vec![
                    // espace + espace insécable -> espace
                    Rule {
                        regex:        Some(Regex::new("  |  ").unwrap()),
                        callback:     None,
                        replace_text: " "
                    },
                    // espaces surnuméraires
                    Rule {
                        regex:        Some(Regex::new("  +").unwrap()),
                        callback:     None,
                        replace_text: " "
                    },
                    // espaces insécables surnuméraires
                    Rule {
                        regex:        Some(Regex::new("  +").unwrap()),
                        callback:     None,
                        replace_text: " "
                    }
                ]
            }
        );
        surnumerary_spaces.insert(
            TextFormaterLine::EndOfParagraph,
             TextFormat {
                short:   "En fin de paragraphe",
                rules:   vec![
                    Rule {
                        regex:        None,
                        callback:     Some(rtrim),
                        replace_text: ""
                    }
                ]
            }
        );
        surnumerary_spaces.insert(
            TextFormaterLine::BeforePunctuation,
             TextFormat {
                short:   "Avant les , ; : ? ! . …",
                rules:   vec![
                    Rule {
                        regex:  Some(Regex::new("[  ]+,").unwrap()),
                        callback: None,
                        replace_text: " ,"
                    },
                    Rule {
                        regex:  Some(Regex::new("[  ]+;").unwrap()),
                        callback: None,
                        replace_text: " ;"
                    },
                    Rule {
                        regex:  Some(Regex::new("[  ]+:").unwrap()),
                        callback: None,
                        replace_text: " :"
                    },
                    Rule {
                        regex:  Some(Regex::new("[  ]+[?]").unwrap()),
                        callback: None,
                        replace_text: " ?"
                    },
                    Rule {
                        regex:  Some(Regex::new("[  ]+!").unwrap()),
                        callback: None,
                        replace_text: " !"
                    },
                    Rule {
                        regex:  Some(Regex::new("[  ]+[.]").unwrap()),
                        callback: None,
                        replace_text: " ."
                    },
                    Rule {
                        regex:  Some(Regex::new("[  ]+…").unwrap()),
                        callback: None,
                        replace_text: " …"
                    }
                ]
            }
        );
        surnumerary_spaces.insert(
            TextFormaterLine::WithinParenthesis,
             TextFormat {
                short:   "À l'intérieur des parenthèses",
                rules:   vec![
                    Rule {
                        regex:  Some(Regex::new("\\([  ]+").unwrap()),
                        callback: None,
                        replace_text: "("
                    },
                    Rule {
                        regex:  Some(Regex::new("[  ]+\\)").unwrap()),
                        callback: None,
                        replace_text: ")"
                    }
                ]
            }
        );
        surnumerary_spaces.insert(
            TextFormaterLine::WithinSquareBrackets,
             TextFormat {
                short:   "À l'intérieur des crochets",
                rules:   vec![
                    Rule {
                        regex:  Some(Regex::new("\\[[  ]+").unwrap()),
                        callback: None,
                        replace_text: "["
                    },
                    Rule {
                        regex:  Some(Regex::new("[  ]+\\]").unwrap()),
                        callback: None,
                        replace_text: "]"
                    }
                ]
            }
        );
        surnumerary_spaces.insert(
            TextFormaterLine::WithinQuotationMarks,
             TextFormat {
                short:   "À l'intérieur des guillements “ et ”",
                rules:   vec![
                    Rule {
                        regex:  Some(Regex::new("“[  ]+").unwrap()),
                        callback: None,
                        replace_text: "“"
                    },
                    Rule {
                        regex:  Some(Regex::new("[  ]”").unwrap()),
                        callback: None,
                        replace_text: "”"
                    }
                ]
            }
        );
        groups.insert(
            TextFormaterGroups::SurnumerarySpaces,
            TextFormaterGroup {
                name: "Espaces surnuméraires",
                lines: surnumerary_spaces
            }
        );
        TextFormater {
            groups: groups
        }
    }

    pub fn all_rules(&mut self) -> Vec<&Rule> {
        let mut rules = Vec::new();
        for (key, val) in &self.groups {
            for (k, v) in &val.lines {
                rules.extend(&v.rules);
            }
        }
        rules
    }

    pub fn get_rules(&mut self, group: TextFormaterGroups, line: TextFormaterLine) -> Vec<&Rule> {
        let mut rules = Vec::new();
        rules.extend(&self.groups[&group].lines[&line].rules);
        rules
    }
}

type Callback = fn(text: &str) -> &str;

pub fn ltrim(text: &str) -> &str {
    text.trim_left()
}

pub fn rtrim(text: &str) -> &str {
    text.trim_right()
}

pub fn format(raw: String, rules: &Vec<&Rule>) -> String {
    let mut value:String = raw;
    for rule in rules {
        match rule.regex {
            Some(ref x) => {
                value = x.replace_all(&*value, rule.replace_text).into_owned();
            },
            None => { }
        }
        match rule.callback {
            Some(ref c) => {
                value = (c)(&*value).to_string();
            },
            None => { }
        }
    }
    value
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn start_of_paragraph() {
        let mut formater = TextFormater::new();
        let rules = formater.get_rules(
            TextFormaterGroups::SurnumerarySpaces,
            TextFormaterLine::StartOfParagraph
        );
        assert_eq!(
            "un texte avec trop d'espaces au début",
            format(TextFormaterLine::StartOfParagraph.as_test(), &rules)
        );
    }

    #[test]
    fn between_words() {
        let mut formater = TextFormater::new();
        let rules = formater.get_rules(
            TextFormaterGroups::SurnumerarySpaces,
            TextFormaterLine::BetweenWords
        );
        assert_eq!(
            "un texte avec trop d'espaces",
            format(TextFormaterLine::BetweenWords.as_test(), &rules)
        );
    }

    #[test]
    fn end_of_paragraph() {
        let mut formater = TextFormater::new();
        let rules = formater.get_rules(
            TextFormaterGroups::SurnumerarySpaces,
            TextFormaterLine::EndOfParagraph
        );
        assert_eq!(
            "un texte avec trop d'espaces à la fin",
            format(TextFormaterLine::EndOfParagraph.as_test(), &rules)
        );
    }

    #[test]
    fn before_punctuation() {
        let mut formater = TextFormater::new();
        let rules = formater.get_rules(
            TextFormaterGroups::SurnumerarySpaces,
            TextFormaterLine::BeforePunctuation
        );
        assert_eq!(
            "un texte avec trop d'espaces dans la ponctuation . ; ! ; …",
            format(TextFormaterLine::BeforePunctuation.as_test(), &rules)
        );
    }

    #[test]
    fn within_parenthesis() {
        let mut formater = TextFormater::new();
        let rules = formater.get_rules(
            TextFormaterGroups::SurnumerarySpaces,
            TextFormaterLine::WithinParenthesis
        );
        assert_eq!(
            "(un texte avec trop d'espaces à l'intérieur des parenthèses)",
            format(TextFormaterLine::WithinParenthesis.as_test(), &rules)
        );
    }

    #[test]
    fn within_square_brackets() {
        let mut formater = TextFormater::new();
        let rules = formater.get_rules(
            TextFormaterGroups::SurnumerarySpaces,
            TextFormaterLine::WithinSquareBrackets
        );
        assert_eq!(
            "[un texte avec trop d'espaces à l'intérieur des crochets]",
            format(TextFormaterLine::WithinSquareBrackets.as_test(), &rules)
        );
    }

    #[test]
    fn within_quotation_marks() {
        let mut formater = TextFormater::new();
        let rules = formater.get_rules(
            TextFormaterGroups::SurnumerarySpaces,
            TextFormaterLine::WithinQuotationMarks
        );
        assert_eq!(
            "“un texte avec trop d'espaces à l'intérieur des guillements”",
            format(TextFormaterLine::WithinQuotationMarks.as_test(), &rules)
        );
    }
}