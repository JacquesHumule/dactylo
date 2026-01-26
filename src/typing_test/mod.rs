mod test_stats;
mod widgets;

pub mod written_word;

use std::collections::VecDeque;

use crossterm::event::{KeyCode, KeyEvent};
use rand::seq::IndexedRandom;
pub use test_stats::TestStats;
use widgets::TwoLines;
use written_word::WrittenWord;

use crate::dictionary::Dictionary;

#[derive(Debug, Clone, Default)]
pub struct TypingTest {
    pub start_time: chrono::DateTime<chrono::Local>,
    pub end_time: Option<chrono::DateTime<chrono::Local>>,
    /// The queue of words to be typed.
    pub word_queue: VecDeque<String>,
    /// The current word being typed.
    pub current_word: Option<WrittenWord>,
    /// The list of words that have been completed.
    pub completed_word_list: Vec<WrittenWord>,
}

impl TypingTest {
    pub fn new(size: usize, dictionary: Dictionary) -> Self {
        let mut word_queue: VecDeque<_> = dictionary
            .words
            .choose_iter(&mut rand::rng())
            .unwrap()
            .take(size)
            .map(Clone::clone)
            .collect();

        Self {
            start_time: chrono::Local::now(),
            end_time: None,
            current_word: Some(WrittenWord::new(word_queue.pop_front().unwrap())),
            word_queue,
            completed_word_list: Vec::new(),
        }
    }

    /// Set previous word as current
    /// Clear current word
    fn write_prev_word(&mut self) {
        let Some(word) = self.completed_word_list.pop() else {
            return;
        };

        if let Some(current_word) = &mut self.current_word {
            self.word_queue
                .push_front(current_word.expected().to_owned());
        }

        self.current_word = Some(word);
    }

    /// Set next word as current
    /// Put current word in completed word list
    fn write_next_word(&mut self) {
        let Some(current_word) = self.current_word.take() else {
            return;
        };
        // current_word.set_end_time();
        self.completed_word_list.push(current_word);

        if let Some(word) = self.word_queue.pop_front() {
            self.current_word = Some(WrittenWord::new(word));
        } else {
            self.end_time = Some(chrono::Local::now());
        }
    }

    /// Remove last letter from current word
    fn remove_letter(&mut self) {
        if let Some(current_word) = &mut self.current_word {
            current_word.pop();
        }
    }

    fn handle_backspace(&mut self) {
        if let Some(current_word) = &self.current_word {
            if current_word.is_empty() && !self.completed_word_list.is_empty() {
                self.write_prev_word();
            } else if !current_word.is_empty() {
                self.remove_letter();
            }
        }
    }

    fn handle_space(&mut self) {
        if let Some(current_word) = &self.current_word
            && !current_word.is_empty()
        {
            self.write_next_word();
        }
    }

    /// Append a letter to the current word
    fn handle_letter(&mut self, letter: char) {
        if let Some(current_word) = &mut self.current_word {
            current_word.push(letter);
        }
    }

    pub fn on_key_event(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Backspace => self.handle_backspace(),
            KeyCode::Char(' ') => self.handle_space(),
            KeyCode::Char(c) => self.handle_letter(c),
            _ => {}
        }
    }

    pub fn widget_two_lines(&self) -> TwoLines<'_> {
        TwoLines { game: self }
    }

    pub fn finished(&self) -> Option<TestStats> {
        if self.current_word.is_none() {
            Some(TestStats {
                start_time: self.start_time,
                end_time: self.end_time.unwrap_or(chrono::Local::now()),
                completed_word_list: self.completed_word_list.clone(),
            })
        } else {
            None
        }
    }
}
