use std::fmt::Display;

use crate::typing_test::written_word::WrittenWord;

#[derive(Debug, Clone)]
pub struct TestStats {
    pub start_time: chrono::DateTime<chrono::Local>,
    pub end_time: chrono::DateTime<chrono::Local>,
    pub completed_word_list: Vec<WrittenWord>,
}

#[derive(Debug, Clone, Default)]
pub struct CharStats {
    pub correct: usize,
    pub incorrect: usize,
    pub extra: usize,
    pub missing: usize,
}

impl Display for CharStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}/{}/{}/{}",
            self.correct, self.incorrect, self.extra, self.missing
        )
    }
}

impl TestStats {
    pub fn wpm(&self) -> f32 {
        let duration = (self.end_time - self.start_time).as_seconds_f32();
        let total_letters: f32 = self
            .completed_word_list
            .iter()
            .filter_map(|word| word.is_correct().then_some(word.written_width() + 1))
            .sum::<usize>() as f32;
        (total_letters / 5.0) / (duration / 60.0)
    }

    pub fn raw_wpm(&self) -> f32 {
        let duration = (self.end_time - self.start_time).as_seconds_f32();
        let total_letters: f32 = self
            .completed_word_list
            .iter()
            .map(|word| word.written_width() + 1)
            .sum::<usize>() as f32;
        (total_letters / 5.0) / (duration / 60.0)
    }

    pub fn accuracy(&self) -> f32 {
        let (correct, total) = self
            .completed_word_list
            .iter()
            .flat_map(|w| w.keystrokes().clone())
            .fold((0, 0), |(c, t), k| {
                if k.is_correct {
                    (c + 1, t + 1)
                } else {
                    (c, t + 1)
                }
            });
        correct as f32 / total as f32
    }

    pub fn char_stats(&self) -> CharStats {
        self.completed_word_list
            .iter()
            .fold(CharStats::default(), |s, w| {
                let ws = w.char_stats();
                CharStats {
                    correct: s.correct + ws.correct,
                    incorrect: s.incorrect + ws.incorrect,
                    extra: s.extra + ws.extra,
                    missing: s.missing + ws.missing,
                }
            })
    }
}
