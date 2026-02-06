mod cli;
mod dictionary;
mod typing_test;

use std::io;

use clap::{CommandFactory, Parser};
use clap_complete::generate;
use crossterm::event::{Event, EventStream, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    macros::{line, span, text},
    widgets::{Block, Paragraph},
};
use tokio_stream::StreamExt;

use crate::{
    cli::{Cli, Command, DictionarySourceEnum},
    dictionary::monkeytype,
    typing_test::{TestStats, TypingTest},
};

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();

    if let Command::Completions { shell } = cli.command {
        generate(shell, &mut Cli::command(), "dactylo", &mut io::stdout());
        return Ok(());
    }

    let terminal = ratatui::init();
    let result = App::new(cli).await?.run(terminal).await;
    ratatui::restore();
    result
}

#[derive(Debug, Default)]
enum AppState {
    #[default]
    MainMenu,
    ShowStats(TestStats),
    TypingTest(TypingTest),
}

#[derive(Debug, Default)]
pub struct App {
    /// Is the application running?
    running: bool,
    /// Event stream.
    event_stream: EventStream,

    state: AppState,
}

impl App {
    /// Construct a new instance of [`App`].
    pub async fn new(cli: Cli) -> color_eyre::Result<Self> {
        let state = match cli.command {
            Command::Words { number, dictionary } => {
                let dict = match dictionary.to_enum() {
                    DictionarySourceEnum::MonkeytypeLang(l) => {
                        monkeytype::fetch(l.to_url()).await?
                    }
                    DictionarySourceEnum::MonkeytypeUrl(u) => monkeytype::fetch(u).await?,
                };
                AppState::TypingTest(TypingTest::new(number, &dict))
            }
            _ => {
                color_eyre::eyre::bail!("Invalid command")
            }
        };

        Ok(Self {
            state,
            ..Default::default()
        })
    }

    /// Run the application's main loop.
    pub async fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        self.running = true;

        while self.running {
            terminal.draw(|f| self.draw(f))?;
            self.handle_crossterm_events().await?;

            if let AppState::TypingTest(game) = &mut self.state
                && let Some(finished) = game.finished()
            {
                self.state = AppState::ShowStats(finished);
            }
        }

        Ok(())
    }

    /// Draw the app
    pub fn draw(&self, frame: &mut Frame) {
        match &self.state {
            AppState::TypingTest(game) => {
                let widget = game.widget_two_lines();
                let pos = widget.cursor_position(frame.area());
                frame.render_widget(widget, frame.area());
                frame.set_cursor_position(pos);
            }
            AppState::ShowStats(finished) => {
                let paragraph = Paragraph::new(text![
                    line![span!("wpm {}", finished.wpm().round())],
                    line![span!("raw wpm {}", finished.raw_wpm().round())],
                    line![span!("acc {}%", (finished.accuracy() * 100.0).round())],
                    line![span!("chars {}", finished.char_stats())],
                ])
                .block(Block::bordered().title("results"));

                frame.render_widget(paragraph, frame.area());
            }
            _ => {}
        }
    }

    /// Reads the crossterm events and updates the state of [`App`].
    async fn handle_crossterm_events(&mut self) -> color_eyre::Result<()> {
        let event = self.event_stream.next().await;

        if let Some(Ok(evt)) = event {
            match evt {
                Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
                _ => {}
            }
        }

        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    fn on_key_event(&mut self, key: KeyEvent) {
        if key.code == KeyCode::Esc {
            self.quit();
            return;
        }

        if let AppState::TypingTest(game) = &mut self.state {
            game.on_key_event(key);
        }
    }

    /// Set running to false to quit the application.
    const fn quit(&mut self) {
        self.running = false;
    }
}
