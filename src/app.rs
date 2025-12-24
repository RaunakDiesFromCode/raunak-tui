use crate::github::Repo;

#[derive(Clone, Copy, PartialEq)]
pub enum Screen {
    Home,
    Projects,
}

#[derive(Clone)]
pub struct App {
    pub current_screen: Screen,
    pub should_quit: bool,

    pub projects: Vec<Repo>,
    pub project_error: Option<String>,
    pub projects_loading: bool, // 👈 NEW
}

impl App {
    pub fn new() -> Self {
        Self {
            current_screen: Screen::Home,
            should_quit: false,
            projects: Vec::new(),
            project_error: None,
            projects_loading: false,
        }
    }

    pub fn start_loading_projects(&mut self) {
        self.projects_loading = true;
    }

    pub fn finish_loading_projects(&mut self) {
        self.projects_loading = false;
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn go_home(&mut self) {
        self.current_screen = Screen::Home;
    }

    pub fn go_projects(&mut self) {
        self.current_screen = Screen::Projects;
    }

    // 👇 THESE TWO METHODS FIX YOUR ERROR
    pub fn set_projects(&mut self, projects: Vec<Repo>) {
        self.projects = projects;
    }

    pub fn set_project_error(&mut self, err: String) {
        self.project_error = Some(err);
    }
}
