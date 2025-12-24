mod app;
mod ui;
mod github;

use std::io;
use std::time::Duration;
use std::sync::{ Arc, Mutex };

use crossterm::{
    execute,
    event::{ self, Event, KeyCode },
    terminal::{ enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen },
};

use ratatui::{ Terminal, backend::CrosstermBackend };
use open;

use app::App;

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    dotenvy::dotenv().ok();

    // ---------- terminal setup ----------
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // ---------- app setup ----------
    let app = Arc::new(Mutex::new(App::new()));

    // ---------- background GitHub fetch ----------
    let app_clone = Arc::clone(&app);
    tokio::spawn(async move {
        {
            let mut app = app_clone.lock().unwrap();
            app.start_loading_projects();
        }

        match github::fetch_starred_owned_repos().await {
            Ok(projects) => {
                let mut app = app_clone.lock().unwrap();
                app.set_projects(projects);
                app.finish_loading_projects();
            }
            Err(err) => {
                let mut app = app_clone.lock().unwrap();
                app.set_project_error(err);
                app.finish_loading_projects();
            }
        }
    });

    // ---------- run tui ----------
    let res = run_app(&mut terminal, app);

    // ---------- cleanup ----------
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: Arc<Mutex<App>>
) -> io::Result<()> {
    loop {
        // ---- quit check ----
        {
            let app_guard = app.lock().unwrap();
            if app_guard.should_quit {
                break;
            }
        }

        // ---- draw UI using snapshot ----
        let snapshot = {
            let app = app.lock().unwrap();
            app.clone()
        };

        terminal.draw(|f| ui::draw(f, &snapshot))?;

        // ---- input handling ----
        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => {
                        let mut app = app.lock().unwrap();
                        app.quit();
                    }
                    KeyCode::Char('h') => {
                        let mut app = app.lock().unwrap();
                        app.go_home();
                    }
                    KeyCode::Char('p') => {
                        let mut app = app.lock().unwrap();
                        app.go_projects();
                    }

                    // ----- external links -----
                    KeyCode::Char('w') => {
                        let _ = open::that("https://hifromraunak.vercel.app/");
                    }
                    KeyCode::Char('g') => {
                        let _ = open::that("https://github.com/RaunakDiesFromCode");
                    }
                    KeyCode::Char('t') => {
                        let _ = open::that("https://www.linkedin.com/in/raunak-manna/");
                    }
                    KeyCode::Char('l') => {
                        let _ = open::that("https://x.com/RaunakM298742");
                    }
                    KeyCode::Char('i') => {
                        let _ = open::that("https://www.instagram.com/har.jagah.raunak/");
                    }
                    KeyCode::Char('e') => {
                        let _ = open::that(
                            "mailto:raunakmanna43@gmail.com?subject=I%20saw%20your%20portfolio"
                        );
                    }

                    // ----- project selection -----
                    KeyCode::Char(c) if c.is_ascii_digit() => {
                        let index = c.to_digit(10).unwrap() as usize;

                        if index == 0 {
                            return Ok(()); // ignore 0
                        }

                        let repo_opt = {
                            let app = app.lock().unwrap();
                            app.projects.get(index - 1).cloned()
                        };

                        if let Some(repo) = repo_opt {
                            let _ = open::that(repo.html_url);
                        }
                    }

                    _ => {}
                }
            }
        }
    }

    Ok(())
}
