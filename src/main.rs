use std::{
    fmt::Display,
    ops::{Index, IndexMut},
    str::FromStr,
};

fn main() {
    let hints = Hints([
        Hint::Exact,
        Hint::Exist,
        Hint::Missing,
        Hint::Missing,
        Hint::Missing,
    ]);
    println!("{}", hints);
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Hint {
    Exact,
    Exist,
    Missing,
}

#[derive(Clone, Copy, PartialEq, Debug)]
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

struct Word([char; 5]);

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
