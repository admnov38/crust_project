mod sidebar;
mod scoreboard;
mod tetromino;
mod game;

use raylib::prelude::*;
use sidebar::SideBar;
use scoreboard::ScoreBoard;
use game::Game;

fn  main() {    
    let mut scoreboard = ScoreBoard::new("highscores.txt");
    scoreboard.format_highscores(";");

    let (mut rl, thread) = raylib::init().size(1500, 750).title("TETRIS").build();
    let mut side_bar = SideBar::new(&rl); 

    let mut ra = RaylibAudio::init_audio_device();
    let mut music = Music::load_music_stream(&thread, "theme.mp3").unwrap();
    ra.play_music_stream(&mut music);

    rl.set_window_icon(raylib::core::texture::Image::load_image("tetris.png").unwrap());

    while !rl.window_should_close() {
        ra.update_music_stream(&mut music);
        
        let mut input: Option<KeyboardKey> = None;
        if side_bar.game_started {
            input = rl.get_key_pressed();
        }
        let mut d = rl.begin_drawing(&thread);
        
        d.gui_set_style(raylib::consts::GuiControl::DEFAULT, 
            raylib::consts::GuiDefaultProperty::TEXT_SIZE as i32, 20);
        d.gui_set_style(raylib::consts::GuiControl::COMBOBOX, 
                raylib::consts::GuiComboBoxProperty::COMBO_BUTTON_WIDTH as i32, 70);

        d.clear_background(Color::from_hex("303030").unwrap());
        side_bar = side_bar.draw(&mut d, &mut scoreboard);
        
        if side_bar.game_started {
            side_bar.game.update(input, &mut d);
            side_bar.game.draw(&mut d);
        }
        if side_bar.game.is_over {
            if side_bar.game.score != 0 {
                scoreboard.update_highscore(&side_bar.game.username, side_bar.game.score as i32);
            }
        }
        drop(d);
    }

    scoreboard.save_highscores("highscores.txt").unwrap();

}