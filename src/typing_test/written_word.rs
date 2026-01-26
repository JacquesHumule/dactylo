use std::ops::Range;

use crate::typing_test::test_stats::CharStats;

#[derive(Debug, Clone)]
pub struct Keystroke {
    // pub time: chrono::DateTime<chrono::Local>,
    // pub key: char,
    pub is_correct: bool,
}

#[derive(Debug, Clone, Default)]
pub struct WrittenWord {
    // start_time: chrono::DateTime<chrono::Local>,
    // end_time: Option<chrono::DateTime<chrono::Local>>,
    expected: String,
    written: String,
    keystrokes: Vec<Keystroke>,
}

#[derive(Debug, Clone)]
pub struct WordAnalysis<'a> {
    pub body_parts: Vec<WordBodyPart<'a>>,
    pub tail: Option<WordTail<'a>>,
}

#[derive(Debug, Clone)]
pub enum WordBodyPart<'a> {
    /// When the written letters match the expected letters
    Correct(&'a str),
    /// When the written letters do not match the expected letters
    Incorrect { expected: &'a str, written: &'a str },
}

#[derive(Debug, Clone)]
pub enum WordTail<'a> {
    /// When letters are extra
    Extra(&'a str),
    /// When letters are missing
    Missing(&'a str),
}

impl WrittenWord {
    pub fn new(expected: String) -> Self {
        Self {
            // start_time: chrono::Local::now(),
            // end_time: None,
            expected,
            written: String::new(),
            keystrokes: Vec::new(),
        }
    }

    // pub fn set_end_time(&mut self) {
    //     self.end_time = Some(chrono::Local::now());
    // }

    pub fn written_width(&self) -> usize {
        self.written.chars().count()
    }

    pub fn expected_width(&self) -> usize {
        self.expected.chars().count()
    }

    pub fn max_width(&self) -> usize {
        self.expected_width().max(self.written_width())
    }

    pub fn push(&mut self, c: char) {
        let correct = if self.written_width() >= self.expected_width() {
            false
        } else {
            self.expected.chars().nth(self.written_width()) == Some(c)
        };

        self.written.push(c);
        self.keystrokes.push(Keystroke {
            // time: chrono::Local::now(),
            // key: c,
            is_correct: correct,
        });
    }

    pub fn is_empty(&self) -> bool {
        self.written.is_empty()
    }

    pub fn pop(&mut self) {
        self.written.pop();
        // self.keystrokes.pop();
    }

    pub fn expected(&self) -> &str {
        &self.expected
    }

    pub fn written(&self) -> &str {
        &self.written
    }

    pub fn keystrokes(&self) -> &Vec<Keystroke> {
        &self.keystrokes
    }

    pub fn is_correct(&self) -> bool {
        self.written == self.expected
    }

    // pub fn wpm(&self) -> Option<f32> {
    //     let Some(end) = self.end_time else {
    //         return None;
    //     };
    //     Some(self.written_width() as f32 / (end - self.start_time).as_seconds_f32() / 300.0)
    // }

    pub fn analyze(&self) -> WordAnalysis<'_> {
        let mut parts = vec![];
        let mut correct = None;
        let mut erange = 0..0;
        let mut wrange = 0..0;

        let matcher = |correct,
                       erange: &Range<usize>,
                       wrange: &Range<usize>,
                       parts: &mut Vec<_>| match correct {
            Some(true) => {
                parts.push(WordBodyPart::Correct(&self.written[wrange.clone()]));
            }
            Some(false) => {
                parts.push(WordBodyPart::Incorrect {
                    expected: &self.expected[erange.clone()],
                    written: &self.written[wrange.clone()],
                });
            }
            // At start
            None => {}
        };

        for (wch, ech) in self.written().chars().zip(self.expected.chars()) {
            let ch_correct = ech == wch;

            if correct != Some(ch_correct) {
                matcher(correct, &erange, &wrange, &mut parts);
                correct = Some(ch_correct);
                wrange.start = wrange.end;
                erange.start = erange.end;
            }
            wrange.end += wch.len_utf8();
            erange.end += ech.len_utf8();
        }
        matcher(correct, &erange, &wrange, &mut parts);

        WordAnalysis {
            body_parts: parts,
            tail: if wrange.end < self.written.len() {
                Some(WordTail::Extra(&self.written[wrange.end..]))
            } else if erange.end < self.expected.len() {
                Some(WordTail::Missing(&self.expected[erange.end..]))
            } else {
                None
            },
        }
    }

    pub fn char_stats(&self) -> CharStats {
        let analysis = self.analyze();

        let mut correct = 0;
        let mut incorrect = 0;

        analysis.body_parts.iter().for_each(|part| match part {
            WordBodyPart::Correct(c) => correct += c.chars().count(),
            WordBodyPart::Incorrect { written, .. } => incorrect += written.chars().count(),
        });

        CharStats {
            correct,
            incorrect,
            extra: analysis.tail.as_ref().map_or(0, |tail| match tail {
                WordTail::Extra(e) => e.chars().count(),
                _ => 0,
            }),
            missing: analysis.tail.as_ref().map_or(0, |tail| match tail {
                WordTail::Missing(m) => m.chars().count(),
                _ => 0,
            }),
        }
    }
}
