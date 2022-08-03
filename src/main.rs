use std::{
    collections::HashMap,
    fmt::{self, Display},
    ops::{BitAnd, BitOr},
};

fn main() {
    let words = include_str!("words_alpha.txt")
        .split('\n')
        .map(|word| word.trim())
        .filter(|word| word.len() == 5)
        .map(Word::from_str)
        .filter(|word| word.letters.len() == 5)
        .filter(|word| word.letters.overlaps(LetterSet::from_str("aeiouy")))
        .collect::<Vec<_>>();

    println!("word count = {:?}", words.len());

    let mut anagrams = HashMap::<LetterSet, Vec<Word>>::new();

    for word in &words {
        anagrams.entry(word.letters).or_default().push(*word);
    }

    println!("unique word count = {:?}", anagrams.len());

    let letter_sets = anagrams.keys().copied().collect::<Vec<_>>();

    find_results(&letter_sets, LetterSetSet::new(), &mut |result| {
        print!("[");
        for (i, set) in result.sets.iter().enumerate() {
            if i > 0 {
                print!(", ");
            }

            let words = &anagrams[set];

            if words.len() == 1 {
                print!("{word}", word = words[0]);
            } else {
                print!("{{");

                for (i, word) in anagrams[set].iter().enumerate() {
                    if i > 0 {
                        print!(", ");
                    }
                    print!("{word}");
                }

                print!("}}");
            }
        }
        println!("]");
    })
}

fn find_results(sets: &[LetterSet], stack: LetterSetSet, out: &mut impl FnMut(LetterSetSet)) {
    if stack.len() == 5 {
        out(stack);
        return;
    }

    for (i, set) in sets.iter().copied().enumerate() {
        if !stack.overlaps(set) {
            find_results(&sets[i + 1..], stack.push(set), &mut *out);
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Word {
    word: &'static str,
    letters: LetterSet,
}

impl Word {
    fn from_str(word: &'static str) -> Self {
        Self {
            word,
            letters: LetterSet::from_str(word),
        }
    }
}

impl Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.word, f)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct LetterSet(u32);

impl LetterSet {
    fn empty() -> Self {
        Self(0)
    }

    fn from_str(s: &str) -> Self {
        Self(
            s.chars()
                .map(|ch| match ch as u32 {
                    n @ 0x61..=0x7a => 1 << (n - 0x61),

                    _ => panic!("invalid character: {ch}"),
                })
                .fold(0, |a, b| a | b),
        )
    }

    fn len(&self) -> u32 {
        self.0.count_ones()
    }

    fn is_empty(&self) -> bool {
        self.0 == 0
    }

    fn overlaps(self, other: Self) -> bool {
        !(self & other).is_empty()
    }
}

impl Display for LetterSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..32u8 {
            if self.0 & (1 << i) != 0 {
                write!(f, "{ch}", ch = (0x61 + i) as char)?;
            }
        }
        Ok(())
    }
}

impl BitOr for LetterSet {
    type Output = LetterSet;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitAnd for LetterSet {
    type Output = LetterSet;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct LetterSetSet {
    size: u8,
    sets: [LetterSet; 5],
    union: LetterSet,
}

impl LetterSetSet {
    fn new() -> Self {
        Self {
            size: 0,
            sets: [LetterSet::empty(); 5],
            union: LetterSet::empty(),
        }
    }

    fn len(&self) -> usize {
        self.size as usize
    }

    fn overlaps(&self, set: LetterSet) -> bool {
        self.union.overlaps(set)
    }

    fn push(mut self, set: LetterSet) -> Self {
        if self.size >= 5 {
            panic!("over capacity");
        }

        self.sets[self.size as usize] = set;
        self.union = self.union | set;
        self.size += 1;

        self
    }
}
