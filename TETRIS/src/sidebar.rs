use raylib::prelude::*;
use raylib::{rgui::RaylibDrawGui};
use std::ffi::CString;

use crate::scoreboard::{ScoreBoard};
#[derive(Clone, Copy)]
pub enum SideBarContent {
    MainView{      
        butt_new_game: Rectangle, 
        button_high_score: Rectangle 
    },
    InitGame{ 
        butt_back: Rectangle,
        butt_start: Rectangle,
        curr_level: i32,
        curr_mode: i32,
        tb_username: Rectangle,
        edit_mode: bool,
        text: [u8; 64],
        cb_level: Rectangle,
        cb_mode: Rectangle
    },
    HighScore{
        butt_back: Rectangle,
        list_scores: Rectangle
     },
    ClassicGame{ 
        rec_next_piece: Rectangle,
        rec_score: Rectangle,
        rec_level: Rectangle,
        button_quit: Rectangle
    },
    ModernGame{ 
        rec_next_piece: Rectangle,
        rec_score: Rectangle,
        rec_level: Rectangle,
        rec_swap_piece: Rectangle,
        button_quit: Rectangle
    }
}

pub struct SideBar {
    rec: Rectangle, 
    padding: f32, 
    colour: Color,
    content: SideBarContent
}

impl SideBar {

    pub fn new(handle: &RaylibHandle) -> SideBar {
        let rec = Rectangle::new(handle.get_screen_width() as f32 * 0.75, 0.0, 
                                        handle.get_screen_width() as f32 * 0.25 as f32, handle.get_screen_height() as f32);

        let padding = 10.0;
        let content = Self::set_main_game_view(padding, &rec);
        
        return SideBar{
            rec: rec,
            padding: padding,
            colour: Color::LIGHTCYAN,
            content: content
        }

    }

    fn set_colour(&mut self, colour: Color) {
        self.colour = colour;
    }

    pub fn draw(mut self, handle: &mut RaylibDrawHandle, scoreboard: &mut ScoreBoard) -> SideBar {
        handle.draw_rectangle_rec(self.rec, self.colour);

        match self.content {
            SideBarContent::MainView { butt_new_game, button_high_score } => {

                let lbl_butt_new_game = CString::new("NEW GAME").unwrap();
                if handle.gui_button(butt_new_game, Some(&lbl_butt_new_game) ) {
                    // println!("Starting new game!");  
                    self.content = Self::set_init_game_view(self.padding, &self.rec);
                }
                
                let lbl_butt_high_score = CString::new("HIGH SCORES").unwrap();
                if handle.gui_button(button_high_score, Some(&lbl_butt_high_score)) {
                    // println!("Showing high score!");
                    self.content = Self::set_highscore_game_view(self.padding, &self.rec);
                }
                
                return self;
            },
            SideBarContent::InitGame { butt_back, butt_start, 
                                       cb_level, cb_mode, 
                                       tb_username, mut edit_mode, ref mut text, 
                                       ref mut curr_level , ref mut curr_mode } => {
                
                let lbl_butt_back = CString::new("BACK").unwrap();
                if handle.gui_button(butt_back, Some(&lbl_butt_back) ) {
                    // println!("GOING BACK TO MAIN VIEW");
                    self.content = Self::set_main_game_view(self.padding, &self.rec);
                    return self;
                }

                let levels = CString::new("LEVEL 1;LEVEL 2;LEVEL 3").unwrap();
                let mut active_level = *curr_level;
                active_level = handle.gui_combo_box(cb_level, Some(&levels), active_level);
                if *curr_level != active_level {
                    // print!("CHANGED ACTIVE LEVEL TO SOMETHING ELSE");
                    *curr_level = active_level;
                }

                let levels = CString::new("CLASSICAL;MODERN").unwrap();
                let mut active_mode: i32 = *curr_mode;
                active_mode = handle.gui_combo_box(cb_mode, Some(&levels), active_mode);
                if *curr_mode != active_mode {
                    // print!("CHANGED MODE TO SOMETHING ELSE");
                    *curr_mode = active_mode;
                }

                let mut username = "";
                handle.draw_text("USERNAME", tb_username.x as i32, (tb_username.y - 30.0) as i32, 20, Color::BLACK);
                if handle.gui_text_box(tb_username, text, edit_mode) {
                    edit_mode = !edit_mode;
                    username = std::str::from_utf8(text).unwrap().trim_end_matches(char::from(0));
                }

                let lbl_butt_start = CString::new("START GAME").unwrap();
                if handle.gui_button(butt_start, Some(&lbl_butt_start) ) {
                    if *curr_mode == 0 {
                        self.content = Self::set_classic_game_view(self.padding, &self.rec, &username, *curr_level);
                    }
                    else {
                        self.content = Self::set_modern_game_view(self.padding, &self.rec, &username, *curr_level);
                    }
                    return self;
                }

                return self;
            },
            SideBarContent::HighScore { butt_back, list_scores } => {
                let lbl_butt_back = CString::new("BACK").unwrap();                
                let content = self.content;

                handle.draw_text("SCORE BOARD", list_scores.x as i32, (list_scores.y - 50.0) as i32, 40, Color::BLACK);

                handle.gui_set_style(raylib::consts::GuiControl::LISTVIEW, 
                    raylib::consts::GuiListViewProperty::LIST_ITEMS_HEIGHT as i32, 40);

                let _active = handle.gui_list_view(list_scores, Some(&scoreboard.formatted_highscores), &mut 0, -1);

                if handle.gui_button(butt_back, Some(&lbl_butt_back)) {
                    // println!("Showing high score!");
                    self.content = Self::set_main_game_view(self.padding, &self.rec);
                    return self;
                }
                content
            },
            SideBarContent::ClassicGame { rec_next_piece, rec_score, rec_level, button_quit} => {            
                let content = self.content;

                handle.draw_text("NEXT PIECE", rec_next_piece.x as i32, (rec_next_piece.y - 20.0) as i32, 20, Color::BLACK);
                handle.draw_rectangle(rec_next_piece.x as i32, rec_next_piece.y as i32, rec_next_piece.width as i32, rec_next_piece.height as i32, Color::GRAY);

                handle.draw_text("SCORE", rec_score.x as i32, (rec_score.y - 20.0) as i32, 20, Color::BLACK);
                handle.draw_rectangle(rec_score.x as i32, rec_score.y as i32, rec_score.width as i32, rec_score.height as i32, Color::GRAY);

                handle.draw_text("LEVEL", rec_level.x as i32, (rec_level.y - 20.0) as i32, 20, Color::BLACK);
                handle.draw_rectangle(rec_level.x as i32, rec_level.y as i32, rec_level.width as i32, rec_level.height as i32, Color::GRAY);


                let lbl_butt_quit = CString::new("QUIT GAME").unwrap();    
                if handle.gui_button(button_quit, Some(&lbl_butt_quit)) {
                    self.content = Self::set_main_game_view(self.padding, &self.rec);
                    return self;
                }
                content
            },
            SideBarContent::ModernGame { rec_next_piece, rec_score, rec_level, rec_swap_piece, button_quit} => todo!(),
        };

        return self
        
    }

    fn set_main_game_view(padding: f32, rec: &Rectangle) -> SideBarContent {

        let button_new_game = Rectangle::new(rec.x + padding, padding, 
                                                    rec.width - 2.0  * padding, 
                                                   50.0);

        let button_high_score = Rectangle::new(rec.x + padding, 2.0*padding + 50.0, 
                                               rec.width - 2.0  * padding, 
                                               50.0);
        SideBarContent::MainView {  
            butt_new_game: button_new_game, 
            button_high_score: button_high_score 
        }       
    }

    fn set_init_game_view(padding: f32, rec: &Rectangle) -> SideBarContent {

        let comobox_level: Rectangle = Rectangle::new(rec.x + padding, padding,
                                                      rec.width - 2.0  * padding,
                                                      50.0);

        let combobox_mode: Rectangle = Rectangle::new(rec.x + padding, padding * 2.0 + 50.0,
                                                      rec.width - 2.0  * padding,
                                                      50.0);

        let textbox_username: Rectangle = Rectangle::new(rec.x + padding, padding * 3.0 + 150.0,
                                                      rec.width - 2.0  * padding, 
                                                      50.0);

        let button_start = Rectangle::new(rec.x + padding, rec.height - 100.0 - 2.0 * padding, 
                                                rec.width - 2.0  * padding, 
                                               50.0);
        
        let button_back = Rectangle::new(rec.x + padding, rec.height - 50.0 - padding, 
                                         rec.width - 2.0  * padding, 
                                         50.0);

        let mut text = [0u8; 64];

        SideBarContent::InitGame { butt_back: button_back, 
                                   butt_start: button_start,
                                   cb_level: comobox_level, 
                                   cb_mode: combobox_mode,
                                   tb_username: textbox_username, 
                                   edit_mode: true,
                                   text: text,
                                   curr_level: -1, 
                                   curr_mode: -1 }
    }

    fn set_highscore_game_view(padding: f32, rec: &Rectangle) -> SideBarContent {

        let list_scores: Rectangle = Rectangle::new(rec.x + padding, padding * 2.0 + 50.0, 
                                                rec.width - 2.0  * padding, 
                                               rec.height - padding * 4.0 - 50.0 * 2.0);

        let button_back = Rectangle::new(rec.x + padding, rec.height - padding - 50.0, 
                                                        rec.width - 2.0  * padding, 
                                                        50.0);

        SideBarContent::HighScore { list_scores: list_scores, butt_back: button_back } 
    }

    fn set_classic_game_view(padding: f32, rec: &Rectangle, username: &str, starting_level: i32) -> SideBarContent {

        let rec_next_piece = Rectangle::new(rec.x + padding, rec.y + padding + 50.0, 
                                                   rec.width - 2.0  * padding, 
                                                  100.0);

        let rec_score = Rectangle::new(rec.x + padding, rec.y + 2.0 * padding + 150.0, 
                                                   rec.width - 2.0  * padding, 
                                                  50.0);

        let rec_level = Rectangle::new(rec.x + padding, rec.y + 3.0 *padding + 200.0, 
                                                   rec.width - 2.0  * padding, 
                                                  50.0);

        let button_quit = Rectangle::new(rec.x + padding, rec.height - padding - 50.0, 
                                                rec.width - 2.0  * padding, 
                                               50.0);

        SideBarContent::ClassicGame { rec_next_piece: rec_next_piece, 
                                      rec_score: rec_score, 
                                      rec_level: rec_level, 
                                      button_quit: button_quit }
    }

    fn set_modern_game_view(padding: f32, rec: &Rectangle, username: &str, starting_level: i32) -> SideBarContent {

        todo!()
    }


}
