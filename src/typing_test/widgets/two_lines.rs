use std::cmp::{max, min};

use ratatui::{
    buffer::Buffer,
    layout::{Position, Rect},
    macros::span,
    style::{Color, Style},
    text::{Line, Span},
    widgets::Widget,
};

use crate::{
    TypingTest,
    typing_test::written_word::{WordBodyPart, WordTail},
};

const CORRECT_STYLE: Style = Style::new().fg(Color::White);
const INCORRECT_STYLE: Style = Style::new().fg(Color::Red);
const MISSING_STYLE: Style = Style::new().dim().underlined().underline_color(Color::Red);
const SUPERFLUOUS_STYLE: Style = Style::new().fg(Color::LightRed);
const NOT_WRITTEN_STYLE: Style = Style::new().fg(Color::DarkGray);

fn span_skip<'a>(s: &str, style: Option<Style>, skip: &mut usize) -> Option<Span<'a>> {
    let len = s.chars().count();
    if len <= *skip {
        *skip -= len;
        None
    } else {
        let remaining = s.chars().skip(*skip).collect::<String>();
        let span = if let Some(style) = style {
            span!(style; "{}", remaining)
        } else {
            span!("{}", remaining)
        };
        *skip = 0;
        Some(span)
    }
}

fn whitespace_skip<'a>(s: &str, skip: &mut usize) -> Option<Span<'a>> {
    let len = s.chars().count();
    if len <= *skip {
        *skip -= len;
        None
    } else {
        let remaining = " ".repeat(len - *skip);
        *skip = 0;
        Some(span!("{}", remaining))
    }
}

pub struct TwoLines<'a> {
    pub(crate) game: &'a TypingTest,
}

impl TwoLines<'_> {
    pub fn completed_width(&self) -> usize {
        self.game
            .completed_word_list
            .iter()
            .map(|w| w.max_width() + 1)
            .sum::<usize>()
            + self
                .game
                .current_word
                .as_ref()
                .map_or(0, |word| word.written_width())
    }

    pub fn width_skip(&self, width: u16) -> usize {
        max(0, self.completed_width() as i64 - (width as i64 / 2)) as usize
    }

    pub fn cursor_position(&self, area: Rect) -> Position {
        Position {
            x: min(self.completed_width() as u16, area.width / 2),
            y: area.y,
        }
    }

    fn render_first_line(&self, area: Rect, buf: &mut Buffer) {
        let mut line = Line::default();
        let mut skip = self.width_skip(area.width);

        self.game.completed_word_list.iter().for_each(|w| {
            if w.max_width() < skip {
                skip -= w.max_width() + 1;
                return;
            }

            let wa = w.analyze();
            for p in wa.body_parts {
                match p {
                    WordBodyPart::Correct(c) => {
                        if let Some(span) = span_skip(c, Some(CORRECT_STYLE), &mut skip) {
                            line.push_span(span);
                        }
                    }
                    WordBodyPart::Incorrect {
                        expected: _,
                        written,
                    } => {
                        if let Some(span) = span_skip(written, Some(INCORRECT_STYLE), &mut skip) {
                            line.push_span(span);
                        }
                    }
                }
            }
            match wa.tail {
                Some(WordTail::Extra(e)) => {
                    if let Some(span) = span_skip(e, Some(SUPERFLUOUS_STYLE), &mut skip) {
                        line.push_span(span);
                    }
                }
                Some(WordTail::Missing(m)) => {
                    if let Some(span) = span_skip(m, Some(MISSING_STYLE), &mut skip) {
                        line.push_span(span);
                    }
                }
                None => {}
            }
            if let Some(span) = whitespace_skip(" ", &mut skip) {
                line.push_span(span);
            }
        });

        // Current word analysis
        if let Some(cwa) = self.game.current_word.as_ref().map(|word| word.analyze()) {
            for p in cwa.body_parts {
                match p {
                    WordBodyPart::Correct(c) => {
                        line.push_span(span!(CORRECT_STYLE; "{}", c));
                    }
                    WordBodyPart::Incorrect {
                        expected: _,
                        written,
                    } => {
                        line.push_span(span!(INCORRECT_STYLE; "{}", written));
                    }
                }
            }
            match cwa.tail {
                Some(WordTail::Extra(e)) => {
                    line.push_span(span!(SUPERFLUOUS_STYLE; "{}", e));
                }
                Some(WordTail::Missing(m)) => {
                    line.push_span(span!(NOT_WRITTEN_STYLE; "{}", m));
                }
                None => {}
            }
            line.push_span(span!(" "));

            for w in &self.game.word_queue {
                line.push_span(span!(NOT_WRITTEN_STYLE; "{}", w));
                line.push_span(span!(" "));
            }
        }
        line.spans.pop();

        line.render(area, buf);
    }

    fn render_second_line(&self, area: Rect, buf: &mut Buffer) {
        let mut line = Line::default();
        let mut skip = self.width_skip(area.width);

        self.game.completed_word_list.iter().for_each(|w| {
            if w.max_width() < skip {
                skip -= w.max_width() + 1;
                return;
            }

            let wa = w.analyze();
            for p in wa.body_parts {
                match p {
                    WordBodyPart::Correct(c) => {
                        if let Some(span) = whitespace_skip(c, &mut skip) {
                            line.push_span(span);
                        }
                    }
                    WordBodyPart::Incorrect {
                        expected,
                        written: _,
                    } => {
                        if let Some(span) = span_skip(expected, Some(CORRECT_STYLE), &mut skip) {
                            line.push_span(span);
                        }
                    }
                }
            }
            match wa.tail {
                Some(WordTail::Extra(e)) => {
                    if let Some(span) = whitespace_skip(e, &mut skip) {
                        line.push_span(span);
                    }
                }
                Some(WordTail::Missing(m)) => {
                    if let Some(span) = whitespace_skip(m, &mut skip) {
                        line.push_span(span);
                    }
                }
                None => {}
            }

            if let Some(span) = whitespace_skip(" ", &mut skip) {
                line.push_span(span);
            }
        });

        // Current word analysis
        if let Some(cwa) = self.game.current_word.as_ref().map(|word| word.analyze()) {
            for p in cwa.body_parts {
                match p {
                    WordBodyPart::Correct(c) => {
                        line.push_span(span!("{}", " ".repeat(c.chars().count())));
                    }
                    WordBodyPart::Incorrect {
                        expected,
                        written: _,
                    } => {
                        line.push_span(span!("{}", expected));
                    }
                }
            }
        } else {
            line.spans.pop();
        }

        line.render(area, buf);
    }
}

impl Widget for TwoLines<'_> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let area = area.intersection(buf.area);
        self.render_first_line(Rect { height: 1, ..area }, buf);
        self.render_second_line(
            Rect {
                height: 1,
                y: area.y + 1,
                ..area
            },
            buf,
        );
    }
}
