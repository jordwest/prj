use crate::config::Config;
use crate::git::GitProject;
use crate::traverse;
use crossterm::{
    cursor,
    event::{read, Event, KeyCode},
    execute, queue,
    style::{Color, Print, SetBackgroundColor, SetForegroundColor},
    terminal, Result,
};
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use std::cmp::Ord;
use std::io::{stderr, Write};
use std::path::PathBuf;
use term_size::dimensions_stderr;

#[derive(Debug)]
struct MatchResult {
    path: PathBuf,
    score: i64,
}

struct UiState {
    query: String,
    results: Vec<MatchResult>,
    selected_index: usize,
}

impl UiState {
    fn select_next(&mut self) {
        if self.selected_index == self.results.len() - 1 {
            self.selected_index = 0;
        } else {
            self.selected_index += 1;
        }
    }

    fn select_prev(&mut self) {
        if self.selected_index == 0 {
            self.selected_index = self.results.len() - 1;
        } else {
            self.selected_index -= 1;
        }
    }

    fn add_char(&mut self, c: char) {
        self.query.push(c);
        self.selected_index = 0;
    }

    fn remove_char(&mut self) {
        self.query.pop();
        self.selected_index = 0;
    }
}

// https://jonasjacek.github.io/colors/
static HIGHLIGHT_BG: Color = Color::AnsiValue(236);
static RESULT_FOOTER_FG: Color = Color::AnsiValue(219);

fn render(query: &str, state: &UiState) -> Result<()> {
    let mut stderr = stderr();

    execute!(stderr, terminal::Clear(terminal::ClearType::All))?;

    let (_, rows) = dimensions_stderr().unwrap();
    let rows = rows as u16;

    let mut row = rows - 3;
    for (i, result) in state.results.iter().enumerate() {
        let is_selected = i == state.selected_index;

        if is_selected {
            queue!(
                stderr,
                SetBackgroundColor(HIGHLIGHT_BG),
                SetForegroundColor(Color::White),
                cursor::MoveTo(0, row),
                Print("> "),
            )?;
        } else {
            queue!(
                stderr,
                SetBackgroundColor(HIGHLIGHT_BG),
                SetForegroundColor(Color::Reset),
                cursor::MoveTo(0, row),
                Print(" "),
                SetBackgroundColor(Color::Reset),
            )?;
        }
        queue!(
            stderr,
            cursor::MoveTo(2, row),
            Print(result.path.to_str().unwrap()),
            SetForegroundColor(Color::Reset),
            SetBackgroundColor(Color::Reset),
        )?;

        if (row == 0) {
            break;
        }
        row -= 1;
    }

    let prompt_row = rows - 1;
    queue!(
        stderr,
        SetForegroundColor(RESULT_FOOTER_FG),
        cursor::MoveTo(2, prompt_row - 1),
        Print(format!("{}", state.results.len())),
        SetBackgroundColor(HIGHLIGHT_BG),
        SetForegroundColor(Color::Blue),
        cursor::MoveTo(0, prompt_row),
        Print(">"),
        cursor::MoveTo(2, prompt_row),
        Print(query),
        SetBackgroundColor(Color::Reset),
        SetForegroundColor(Color::Reset)
    )?;

    stderr.flush()?;

    Ok(())
}

pub fn run(config: &Config) -> Result<()> {
    let matcher = SkimMatcherV2::default();

    let root = traverse::Root::traverse(config).unwrap();

    let mut exit = false;
    let mut ui_state = UiState {
        query: String::from(""),
        results: vec![],
        selected_index: 0,
    };
    let mut selected_project = None;

    execute!(stderr(), terminal::EnterAlternateScreen)?;
    while !exit {
        ui_state.results = Vec::new();
        for remote in &root.remotes {
            for proj in &remote.projects {
                let match_score = matcher.fuzzy_match(proj.path.to_str().unwrap(), &ui_state.query);
                if let Some(score) = match_score {
                    ui_state.results.push(MatchResult {
                        score,
                        path: proj.path.to_path_buf(),
                    });
                }
            }
        }

        ui_state.results.sort_by(|a, b| b.score.cmp(&a.score));

        render(&ui_state.query, &ui_state)?;

        terminal::enable_raw_mode()?;
        let read_result = read();
        terminal::disable_raw_mode()?;

        match read_result.unwrap() {
            Event::Key(event) => match event.code {
                KeyCode::Char(c) => ui_state.add_char(c),
                KeyCode::Backspace => ui_state.remove_char(),
                KeyCode::Down => ui_state.select_prev(),
                KeyCode::Up => ui_state.select_next(),
                KeyCode::Esc => exit = true,
                KeyCode::Enter => {
                    selected_project = Some(ui_state.results[ui_state.selected_index].path.clone());
                    exit = true;
                }
                _ => (),
            },
            _ => (),
        };
    }
    execute!(stderr(), terminal::LeaveAlternateScreen)?;

    if let Some(path) = selected_project {
        let p = GitProject::from_path(&path);
        println!("{:#?}", p);
    }
    Ok(())
}
