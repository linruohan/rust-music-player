use ratatui::{
    prelude::{Alignment, Frame},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::app::App;

// Thisis the main function that renders stuff in the UI
pub fn render(app: &mut App, f: &mut Frame) {
    // Printing all the paths
    let mut path_string = String::new();
    for (i, path) in app.songs_list.list.iter().enumerate() {
        let tmp_path = path.split("/").last().unwrap();
        if i == app.songs_list.index {
            path_string = format!("{}>> - {}\n", path_string, tmp_path);
        } else if i as i32 > app.songs_list.index as i32 - 3 {
            path_string = format!("{}   - {}\n", path_string, tmp_path);
        }
    }

    let mut play_deque = String::new();
    for p in &app.play_deque {
        let tmp_path = p.split("/").last().unwrap();
        play_deque = format!("{}    {}\n", play_deque, tmp_path);
    }

    let volume = app.sink.volume() * 100.0;

    // Starting with Ratatui
    let rect1 = f.area();
    //rect1.height /= 2;
    f.render_widget(
    Paragraph::new(format!("\n  Volume: {}% \n  Now Playing:\n   {}\n  Playing Queue:\n{}\n\n  Current Directory: {}\n{}",
                           volume.floor(),
                           app.now_playing,
                           play_deque,
                           app.songs_list.path,
                           path_string
                           )
                   )
    .block(
      Block::default()
        .title("Rusty Music Player")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded),
    )
    .style(Style::default().fg(Color::Blue))
    .alignment(Alignment::Left),
    rect1,
  );

    // Second Widget Test
    /*
    let mut rect2 = f.size();
    rect2.height /= 2;
    rect2.y = rect2.height;

    f.render_widget(
      Paragraph::new(format!("Ciaoo"))
      .block(
         Block::default()
          .title("Youtube Downloader")
          .title_alignment(Alignment::Center)
          .borders(Borders::ALL)
          .border_type(BorderType::Rounded)
          /*
          .padding(Padding::new(
              0, // left
              0, // right
              f.size().height / 2 + 1, // top
              0, // bottom
          ))
          */
          ,
        )
      .style(Style::default().fg(Color::Blue))
      .alignment(Alignment::Left),
      rect2,
    );
      */
}
