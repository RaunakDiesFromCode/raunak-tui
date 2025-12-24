use ratatui::{
    Frame,
    layout::{ Layout, Constraint, Direction, Alignment, Rect },
    widgets::{ Block, Borders, Paragraph },
    text::{ Span, Line },
    style::{ Style, Color },
};

use ratatui::widgets::Wrap;

use crate::app::{ App, Screen };

/* ---------- STYLE ---------- */

const BASE_STYLE: Style = Style::new().fg(Color::White);
const HIGHLIGHT_STYLE: Style = Style::new().bg(Color::DarkGray).fg(Color::White);
const PSYDUCK_ASCII: &str =
    r#"
⠀⠀⠀⠀⠀⠀⠀⠀⣤⡀⠀⣶⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠙⣿⣆⣿⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠸⣷⣮⣿⣿⣄⣀⣀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⢀⡠⠒⠉⠀⠀⠀⠀⠀⠀⠈⠁⠲⢖⠒⡀⠀⠀
⠀⠀⠀⡠⠴⣏⠀⢀⡀⠀⢀⡀⠀⠀⠀⡀⠀⠀⡀⠱⡈⢄⠀
⠀⠀⢠⠁⠀⢸⠐⠁⠀⠄⠀⢸⠀⠀⢎⠀⠂⠀⠈⡄⢡⠀⢣
⠀⢀⠂⠀⠀⢸⠈⠢⠤⠤⠐⢁⠄⠒⠢⢁⣂⡐⠊⠀⡄⠀⠸
⠀⡘⠀⠀⠀⢸⠀⢠⠐⠒⠈⠀⠀⠀⠀⠀⠀⠈⢆⠜⠀⠀⢸
⠀⡇⠀⠀⠀⠀⡗⢺⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠑⡄⢀⠎
⠀⢃⠀⠀⠀⢀⠃⢠⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠷⡃⠀
⠀⠈⠢⣤⠀⠈⠀⠀⠑⠠⠤⣀⣀⣀⣀⣀⡀⠤⠒⠁⠀⢡⠀
⡀⣀⠀⡆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢘⠀
⠑⢄⠉⢳⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡸⠀
⠀⠀⠑⠢⢱⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⡴⠁⠀
⠀⠀⠀⠀⢀⠠⠓⠢⠤⣀⣀⡀⠀⠀⣀⣀⡀⠤⠒⠑⢄⠀⠀
⠀⠀⠀⠰⠥⠤⢄⢀⡠⠄⡈⡀⠀⠀⣇⣀⠠⢄⠀⠒⠤⠣⠀
⠀⠀⠀⠀⠀⠀⠀⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠀⠀⠀⠀
"#;

/* ---------- HELPERS ---------- */

fn nav_item(label: &str, active: bool) -> Span<'static> {
    if active {
        Span::styled(format!(" {} ", label), HIGHLIGHT_STYLE)
    } else {
        Span::styled(format!(" {} ", label), BASE_STYLE)
    }
}

/* ---------- DRAW ---------- */

pub fn draw(f: &mut Frame, app: &App) {
    let size = f.size();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // header
            Constraint::Min(5), // body
            Constraint::Length(3), // footer
        ])
        .split(size);

    draw_header(f, chunks[0], app);
    draw_body(f, chunks[1], app);
    draw_footer(f, chunks[2]);
}

/* ---------- HEADER ---------- */

fn draw_header(f: &mut Frame, area: Rect, app: &App) {
    let line = Line::from(
        vec![
            Span::styled(" RAUNAK ", BASE_STYLE),
            Span::raw("   "),
            nav_item("(h)ome", app.current_screen == Screen::Home),
            Span::raw(" "),
            nav_item("(p)rojects", app.current_screen == Screen::Projects),
            Span::raw(" "),
            Span::styled(" (q)uit ", BASE_STYLE)
        ]
    );

    let header = Paragraph::new(line)
        .block(Block::default().borders(Borders::ALL))
        .alignment(Alignment::Center);

    f.render_widget(header, area);
}

/* ---------- BODY ---------- */

fn draw_body(f: &mut Frame, area: Rect, app: &App) {
    match app.current_screen {
        Screen::Home => draw_home(f, area),
        Screen::Projects => draw_projects(f, area, app),
    }
}

fn draw_home(f: &mut Frame, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(55), // left text
            Constraint::Percentage(45), // right ascii
        ])
        .split(area);

    // ---------- LEFT PANEL ----------
    let left_text =
        "I am Raunak
I make the web beautiful

Based in India, I am a web developer who loves to create beautiful and responsive websites.
I have been working in the field for over 2 years and have worked with a variety of technologies
including ReactJS, NextJS, NodeJS, and more. I am passionate about creating beautiful and
responsive websites that are easy to use and look great on all devices. I am always looking
for new opportunities to learn and grow as a developer and am excited to see where my career
takes me.";

    let left = Paragraph::new(left_text)
        .block(Block::default().borders(Borders::ALL).title("About"))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: false })
        .style(BASE_STYLE);

    f.render_widget(left, chunks[0]);

    // ---------- RIGHT PANEL ----------
    let right = Paragraph::new(PSYDUCK_ASCII)
        .block(Block::default().borders(Borders::ALL))
        .alignment(Alignment::Center)
        .style(BASE_STYLE);

    f.render_widget(right, chunks[1]);
}

fn draw_projects(f: &mut Frame, area: Rect, app: &App) {
    let content = if app.projects_loading {
        "Loading projects from GitHub…".to_string()
    } else if let Some(err) = &app.project_error {
        format!("Error loading projects:\n\n{}", err)
    } else if app.projects.is_empty() {
        "No projects found.".to_string()
    } else {
        app.projects
            .iter()
            .enumerate()
            .map(|(i, repo)| {
                let desc = repo.description.as_deref().unwrap_or("No description provided.");

                format!("({}) {}\n{}\n", i + 1, repo.name, desc)
            })
            .collect::<Vec<_>>()
            .join("\n")
    };

    let body = Paragraph::new(content)
        .block(Block::default().borders(Borders::ALL).title("Projects"))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: false })
        .style(BASE_STYLE);

    f.render_widget(body, area);
}

/* ---------- FOOTER ---------- */

fn draw_footer(f: &mut Frame, area: Rect) {
    let line = Line::from(
        vec![
            Span::styled(" (w)ebsite ", BASE_STYLE),
            Span::styled(" (g)ithub ", BASE_STYLE),
            Span::styled(" (t)witter ", BASE_STYLE),
            Span::styled(" (l)inkedin ", BASE_STYLE),
            Span::styled(" (i)nstagram ", BASE_STYLE),
            Span::styled(" (e)mail ", BASE_STYLE)
        ]
    );

    let footer = Paragraph::new(line)
        .block(Block::default().borders(Borders::ALL))
        .alignment(Alignment::Center);

    f.render_widget(footer, area);
}
