use kdam::tqdm;
use std::{
    collections::HashMap,
    fmt::Display,
    ops::{Index, IndexMut},
    str::FromStr,
};

type Real = f32;

fn main() {
    let hints = Hints([
        Hint::Exact,
        Hint::Exist,
        Hint::Missing,
        Hint::Missing,
        Hint::Missing,
    ]);
    println!("{}", hints);

    // get list of allowed words
    let allowed_words = include_str!("../possible-words.txt")
    // let allowed_words = include_str!("../allowed-words.txt")
        .lines()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<Word>>();

    // get hints distribution across all words
    let mut words_entropy: Vec<(Word, Real)> = vec![];

    for &guess in tqdm!(allowed_words.iter()) {
        let dist: HashMap<Hints, usize> = allowed_words
            .iter()
            .map(|&word| check(guess, word))
            .fold(HashMap::new(), |mut acc, hint| {
                *acc.entry(hint).or_insert(0) += 1;
                acc
            });

        let entropy = dist
            .iter()
            .map(|(_, &count)| {
                let p = count as Real / allowed_words.len() as Real;
                -p * p.log2()
            })
            .sum::<Real>();

        words_entropy.push((guess, entropy));
    }

    // sort by entropy in descending order
    words_entropy.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());

    // print top 20
    for (word, entropy) in words_entropy.iter().take(20) {
        println!("{}: {}", word, entropy);
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Hint {
    Exact,
    Exist,
    Missing,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Hints([Hint; 5]);

impl Display for Hints {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for hint in self.0.iter() {
            match hint {
                Hint::Exact => write!(f, "🟩")?,
                Hint::Exist => write!(f, "🟨")?,
                Hint::Missing => write!(f, "⬜️")?,
            }
        }
        Ok(())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Word([char; 5]);

impl Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for &c in self.0.iter() {
            write!(f, "{}", c)?;
        }
        Ok(())
    }
}

impl Index<usize> for Word {
    type Output = char;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Word {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Word {
    fn new(word: [char; 5]) -> Self {
        Self(word)
    }

    fn iter(&self) -> std::slice::Iter<'_, char> {
        self.0.iter()
    }
}

impl FromStr for Word {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        Ok(Self([
            chars.next().ok_or("missing char")?,
            chars.next().ok_or("missing char")?,
            chars.next().ok_or("missing char")?,
            chars.next().ok_or("missing char")?,
            chars.next().ok_or("missing char")?,
        ]))
    }
}

trait IntoWord {
    fn into_word(self) -> Word;
}

impl IntoWord for [char; 5] {
    fn into_word(self) -> Word {
        Word::new(self)
    }
}

impl IntoWord for &str {
    fn into_word(self) -> Word {
        self.parse().unwrap()
    }
}

impl IntoWord for Word {
    fn into_word(self) -> Word {
        self
    }
}

fn check(guess: impl IntoWord, truth: impl IntoWord) -> Hints {
    let guess = guess.into_word();
    let mut truth = truth.into_word();
    let mut hints = Hints([Hint::Missing; 5]);
    for (i, &g) in guess.iter().enumerate() {
        if g == truth[i] {
            hints.0[i] = Hint::Exact;
        }
    }
    for (i, &g) in guess.iter().enumerate() {
        if hints.0[i] == Hint::Exact {
            continue;
        }
        if let Some(f) = truth.iter().position(|&t| t == g) {
            hints.0[i] = Hint::Exist;
            truth[f] = '\0';
        }
    }
    hints
}

#[cfg(test)]
mod tests {
    use super::{check, Hint, Hints};

    #[test]
    fn test_check() {
        use Hint::*;
        assert_eq!(check("abced", "abced"), Hints([Exact; 5]));
        assert_eq!(check("abced", "edbca"), Hints([Exist; 5]));
        assert_eq!(check("abcde", "fghij"), Hints([Missing; 5]));
        assert_eq!(check("torus", "torus"), Hints([Exact; 5]));
        assert_eq!(
            check("surot", "torus"),
            Hints([Exist, Exist, Exact, Exist, Exist])
        );
        assert_eq!(
            check("acute", "torus"),
            Hints([Missing, Missing, Exist, Exist, Missing])
        );
        assert_eq!(
            check("maths", "torus"),
            Hints([Missing, Missing, Exist, Missing, Exact])
        );
        assert_eq!(
            check("speed", "abide"),
            Hints([Missing, Missing, Exist, Missing, Exist])
        );
        assert_eq!(
            check("speed", "erase"),
            Hints([Exist, Missing, Exist, Exist, Missing])
        );
    }
}
