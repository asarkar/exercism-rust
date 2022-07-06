use super::Error;
use super::Result;
use super::Rule;
use pest::iterators::Pair;
use std::collections::HashMap;

type Word = String;
type Defn<'a> = (usize, Pair<'a, Rule>);

#[derive(Default, Debug)]
pub struct Dict<'a> {
    words: HashMap<Word, Vec<Defn<'a>>>,
    defn_id: usize,
}

impl<'a> Dict<'a> {
    pub fn to_word(pair: &Pair<'a, Rule>) -> Word {
        pair.as_str().trim().to_ascii_uppercase()
    }

    pub fn is_word(&self, pair: &Pair<'a, Rule>) -> bool {
        let word = Dict::to_word(pair);
        self.words.contains_key(&word)
    }

    pub fn new_word(&mut self, pair: Pair<'a, Rule>) -> Result {
        let mut defn: Option<Pair<'a, Rule>> = None;

        // p: Rule:: WordDefn. Reverse the inners so that definition
        // comes before the word
        for p in pair.into_inner().rev() {
            match p.as_rule() {
                Rule::Word => {
                    let word = Dict::to_word(&p);
                    let d = defn
                        .clone()
                        .unwrap_or_else(|| panic!("No definition found for word: {}", word));
                    self.words.entry(word).or_default().push((self.defn_id, d));
                    self.defn_id += 1;
                }
                Rule::Defn => {
                    defn.replace(p);
                }
                _ => panic!("Unknown rule in word definition: {}", p),
            };
        }
        Ok(())
    }

    // Resolve a definition by recursively replacing all user-defined words with built-in
    // words. If a word is not found in the dictionary, it could be a built-in word, or
    // an invalid one. We will find out when we later try to parse it as a built-in word.
    pub fn resolve_defn(&self, pair: &Pair<'a, Rule>) -> std::result::Result<Vec<String>, Error> {
        let word = Dict::to_word(pair);
        if !self.is_word(pair) {
            return Ok(vec![word]);
        }
        let mut defn = Vec::new();
        // Take the latest definition
        let (id, p) = self.words[&word].last().unwrap();

        // p: Rule::Defn. We process in the reverse order,
        // or in a right-associative manner, recursively replacing
        // each word with its value.
        //
        // Example:
        // : foo 5 ;
        // : foo foo 1 ;
        // foo = 5 1
        for p in p.clone().into_inner().rev() {
            let w = Dict::to_word(&p);
            let built_in = !self.words.contains_key(&w);
            // p: Rule::Cmd
            let rule = p.into_inner().peek().unwrap().as_rule();

            match rule {
                Rule::BuiltInCmd => defn.push(w),
                Rule::Word => {
                    // Since built-in commands can be redefined,
                    // most tokens other than int come in as words.
                    if built_in {
                        defn.push(w);
                    }
                    // Refers to another word in the dictionary.
                    // For the other word, we find the greatest id
                    // that is smaller than this word's id.
                    // We can use a binary search because definitions
                    // are added with monotonically increasing ids, thus
                    // establishing a happens-before relationship
                    // between two definitions.
                    else if let Some(d) = self.words.get(&w) {
                        let i = match d.binary_search_by_key(id, |&(a, _)| a) {
                            // References an earlier version of itself,
                            // like : foo foo 1 ;
                            Ok(i) => i,
                            // References some other word,
                            // like : bar foo 1 ;
                            // The index is the insersion point,
                            // i.e. the first element greater than
                            // id.
                            Err(i) => i,
                        };
                        let d = self.resolve_defn(&d[i - 1].1)?;
                        defn.extend(d);
                    } else {
                        defn.push(w);
                    }
                }
                _ => {}
            }
        }
        // Don't forget to reverse the definition since we processed
        // it in the reverse order
        defn.reverse();
        Ok(defn)
    }
}
