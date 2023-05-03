mod sidebar;
mod scoreboard;
mod tetromino;
mod game;

use raylib::prelude::*;
use sidebar::SideBar;
use scoreboard::ScoreBoard;
use game::Game;

fn handle_input(rl: &mut RaylibHandle) -> Option<KeyboardKey> {
    rl.get_key_pressed()
}

fn main() {
    let mut scoreboard = ScoreBoard::new("highscores.txt");
    scoreboard.format_highscores(";");

    let (mut rl, thread) = raylib::init().size(1500, 750).title("TETRIS").build();
    let mut side_bar = SideBar::new(&rl);
    let mut gameboard = Game::new(&rl, game::Mode::Classic, 32);

    let mut ra = RaylibAudio::init_audio_device();
    let mut music = Music::load_music_stream(&thread, "theme.mp3").unwrap();
    ra.play_music_stream(&mut music);

    while !rl.window_should_close() {
        ra.update_music_stream(&mut music);

        let kkt= handle_input(&mut rl);
        let mut d = rl.begin_drawing(&thread);

        d.gui_set_style(raylib::consts::GuiControl::DEFAULT, 
            raylib::consts::GuiDefaultProperty::TEXT_SIZE as i32, 20);

        d.clear_background(Color::WHITE);
        side_bar = side_bar.draw(&mut d, &mut scoreboard);
        
        gameboard.update(kkt);
        gameboard.draw(&mut d);
        drop(d);
    }

    scoreboard.save_highscores("highscores.txt").unwrap();

}