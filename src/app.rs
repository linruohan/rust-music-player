use rodio::Sink;

use crate::list::ContentList;
use directories::UserDirs;
use std::collections::VecDeque;
use std::fs;
use std::path::PathBuf;
/// Application.
pub struct App {
    /// should the application exit?
    pub should_quit: bool,
    pub sink: Sink,
    pub songs_list: ContentList,
    pub play_deque: VecDeque<String>,
    pub now_playing: String,
}
use anyhow::{Context, Result};

fn get_music_dir_with_fallback() -> Result<PathBuf> {
    let user_dirs = UserDirs::new().context("can't get music dir")?;

    // 优先尝试音乐目录，失败则回退到家目录下的 Music 子目录
    let music_dir = user_dirs
        .audio_dir()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| user_dirs.home_dir().join("Music"));

    Ok(music_dir)
}

fn ensure_music_dir() -> Result<PathBuf> {
    let dir = get_music_dir_with_fallback()?;
    if !dir.exists() {
        fs::create_dir_all(&dir).context("can't create music dir")?;
    }
    Ok(dir)
}
impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(s: Sink) -> Self {
        let music_dir = ensure_music_dir().unwrap();
        App {
            should_quit: false,
            sink: s,
            songs_list: ContentList::from_dir(music_dir.to_str().unwrap()),
            play_deque: VecDeque::new(),
            now_playing: String::new(),
        }
    }

    // Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set should_quit to true to quit the application.
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    /// memove the front of the queue
    pub fn pop_play_deque(&mut self) {
        if !self.play_deque.is_empty() {
            let _ = self.play_deque.pop_front();
        }
    }

    pub fn add_play_deque(&mut self, s: String) {
        self.play_deque.push_back(s);
    }
}
