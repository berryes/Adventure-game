
use std::{thread, time::{self, Duration}, vec, io::{stdin, stdout,Write, Stdout}};
use async_std::stream::StreamExt;

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, Event, EventStream, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
    Result,
};

use futures::{FutureExt, select};
use futures_timer::Delay;
use termion::{raw::{IntoRawMode, RawTerminal}, color::{self, Reset}, input::TermRead};

use crate::{game::Game, input_handler::{FilterInputStreamForArrows, GameDirectionKey}};


// TOP BAR
fn display_level(stdout:&mut RawTerminal<Stdout>, game:Game, x:u16, y:u16){
    writeln!( stdout, 
        "{} {}", 

        termion::cursor::Goto(x/2,3),
        format!("Level {}", game.level),
        ).unwrap();

}


// LEFT BAR
fn display_stats(stdout:&mut RawTerminal<Stdout>, game:Game, x:u16, y:u16){


    writeln!( stdout, 
        "{} {} {} {} {} {} {} {}", 

        termion::cursor::Goto(1,y-4),
        format!("{} {:.2} {} Health", color::Bg(color::Red), game.character.health.round(), color::Bg(color::Reset)),                
        
        termion::cursor::Goto(1,y-3),
        format!("{} {:.2} {} Attack", color::Bg(color::LightYellow), game.character.weapon.normal.round(), color::Bg(color::Reset)),                


        termion::cursor::Goto(1,y-2),
        format!("{} {:.2} {} Protection", color::Bg(color::LightCyan), game.character.armour.normal.round(), color::Bg(color::Reset)),                


        termion::cursor::Goto(1,y-1),
        format!("{} {:.2} {} XP", color::Bg(color::LightGreen), game.xp, color::Bg(color::Reset)),                


        ).unwrap();

}

// RIGH BAR
fn display_enemy(stdout:&mut RawTerminal<Stdout>, game:Game, x:u16, y:u16){


    let enemy_faction: String = format!("Faction {} {:?} {} ", color::Bg(color::LightCyan), game.enemy.faction, color::Bg(color::Reset));
    let enemy_damage: String = format!("Attack {} {:.2} {} ", color::Bg(color::LightYellow), game.enemy.damage.round(), color::Bg(color::Reset));  
    let enemy_health: String = format!("Health {} {:.2} {}", color::Bg(color::Red), game.enemy.health.round(), color::Bg(color::Reset));                

    writeln!( stdout, 
        "{} {} {} {} {} {}", 

        termion::cursor::Goto(x-13,y-4),
        enemy_health,

        termion::cursor::Goto(x-13 as u16,y-3),
        enemy_damage,

        termion::cursor::Goto(x-16,y-2),
        enemy_faction,

        ).unwrap();

}


pub struct  Hitbar{
    pub reader: EventStream,
    pub stdout:  RawTerminal<Stdout>,
    pub game:  Game,
    
    //private
    print_string: String,
    marker: i32,
    backwards_counter: i32,
    hit_range: Vec<i32>,
    speed: u64,
}


impl Hitbar {
    // i did this just to soly practice "production" coding cus i saw this being used
    // also now i dont have to define private fields 
    pub fn new(
        reader: EventStream,
        stdout: RawTerminal<Stdout>,
        game: Game,
    ) -> Hitbar {
        Hitbar { 
            reader: reader, 
            stdout: stdout, 
            game: game, 
            print_string: String::new(),
            marker: 0,
            backwards_counter: 0,
            hit_range: vec![3,6],
            speed: 80,
         }
    }


     pub async fn play(&mut self ) -> bool{
        
        loop  {

            // player go killed
            if self.game.character.health <= 0.0{
                return false
            }

            // enemy killed
            if self.game.enemy.health <= 0.0 { 
                return true;
            };


            let (x, y) = termion::terminal_size().unwrap();

             // displaying shit in the ui
             display_stats(&mut self.stdout, self.game.clone(), x,y);
             display_enemy(&mut self.stdout, self.game.clone(), x, y);
             display_level(&mut self.stdout, self.game.clone(), x, y);

            // setting color back just in case
             writeln!( self.stdout, "{}",  color::Fg(color::Reset)).unwrap();


             let mut delay = Delay::new(Duration::from_millis( self.speed )).fuse();
             let mut event = self.reader.next().fuse();

             self.print_string.clear();

            if self.marker == 16 { self.backwards_counter+= 16 } // if its at the end
            if self.marker == 0 { self.backwards_counter = 0 } // when it arrives at pos 0 again


            for i in 0..16  {   
    
                if i == self.marker { self.print_string.push_str("|"); }
    
                else { 
                    if i == self.hit_range[0]{
                        self.print_string.push_str("[");   
                    }
                    else{
                        self.print_string.push_str("=");
                    } 
                    if i == self.hit_range[1]{
                        self.print_string.push_str("]");
                    }else{
                        self.print_string.push_str("=");
                    }
                 }
            
            }
    
            if self.backwards_counter > 0 { self.marker-=1 }
            else{ self.marker+= 1 }
    
            writeln!( self.stdout, 
                "{} {}", 
                termion::cursor::Goto(( (x as usize - self.print_string.len() ) / 2) as u16 , y/2),
                self.print_string,
                ).unwrap();
            
            // filtering input for enter  & space and adjusting scores according to it
            select! {
                _ = delay => {  },
                maybe_event = event => {
                    match maybe_event {
                        Some(Ok(event)) => {
                                if event == Event::Key( KeyCode::Enter.into() )  {
                                

                                    // if player hit between hitrange
                                    if  self.hit_range[0] <= self.marker && self.marker <= self.hit_range[1]{
                                        writeln!( self.stdout, 
                                            "{} {} Hit!{}", 
                                            termion::cursor::Goto(3,5),
                                            color::Fg(color::Green),
                                            color::Fg(color::Reset),
                                            ).unwrap();

                                            self.game.hit_attack();
                                            
                                    }else{
                                    // if not, increase speed and take one from the dmg chances
                                    writeln!( self.stdout, 
                                        "{} {} Miss!{}", 
                                        termion::cursor::Goto(3,5),
                                        color::Fg(color::Red),
                                        color::Fg(color::Reset),
                                        ).unwrap();

                                        self.game.missed_attack();
                                    }
                                   } // exit imnplementation
                                if event == Event::Key(KeyCode::Esc.into()) || event == Event::Key(KeyCode::Char('q').into()) { panic!("exited game"); }        
                            } // end of match ok scene
                        Some(Err(e)) => println!("Error: {:?}\r", e),
                        None => return false,
                    }
                }
            };
        }
}
}

pub fn get_next_step(stdout: &mut RawTerminal<Stdout>) -> i8 {

    
    let stdin = stdin();
    let (x, y) = termion::terminal_size().unwrap();

    writeln!(
        stdout,
        "{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",

        termion::clear::All,

        termion::cursor::Goto(x/2,y/2-5),
        termion::color::Fg(color::Black),
        termion::color::Bg(color::LightWhite),
        String::from("SPACE"),

        termion::color::Fg(Reset),
        termion::color::Bg(Reset),
        String::from(" Next stage"),

        termion::cursor::Goto(x/2,y/2-4),
        termion::color::Fg(color::Black),
        termion::color::Bg(color::LightWhite),
        String::from("I"),

        termion::color::Fg(Reset),
        termion::color::Bg(Reset),
        String::from(" Inventory"),


        termion::cursor::Goto(x/2,y/2-3),
        termion::color::Fg(color::Black),
        termion::color::Bg(color::LightWhite),
        String::from("U"),

        termion::color::Fg(Reset),
        termion::color::Bg(Reset),
        String::from(" Upgrade"),

        termion::cursor::Goto(x/2,y/2-2),
        termion::color::Fg(color::Black),
        termion::color::Bg(color::LightWhite),
        String::from("S"),

        termion::color::Fg(Reset),
        termion::color::Bg(Reset),
        String::from(" Stats"),
        
        termion::cursor::Goto(x/2,y/2-1),
        termion::color::Fg(color::Black),
        termion::color::Bg(color::LightWhite),
        String::from("Q"),

        termion::color::Fg(Reset),
        termion::color::Bg(Reset),
        String::from(" Quit"),


    ).unwrap();


    for key_press in stdin.keys() {

        // this match case is ugly af but ig this is how rust works
        match key_press {

            Ok(_key) => {
                match _key {
                    termion::event::Key::Char( _character ) => {
                        match _character {
                        
                                // space \ next stage 
                                '\n' => {
                                    return 1
                                }

                                //  inventory
                                'i' =>{
                                    return 2
                                }
                                // upgrade
                                'u' =>{
                                    return 3
                                }
                                // stats
                                's' => {
                                    return 4
                                }
                                
                                // quit
                                'q' =>{
                                    
                                    return 0

                                }

                            _=> {}
                        }
                    }
                    
                    _=> {}
                }

            }


            Err(_error) =>{}
        }

    }

    return 1;
}


    
/*
https://piped.video/watch?v=cojoYPRcIJA&t=51

Parlons peu, pardon maman, pardon Dieu (uh-uh)
Ton gars n'est pas dangereux, il a pas b'soin d'fer, ni d'baveux (coupe, coupe)
La jalousie, ça rend miséreux, ça dévie les hommes pieux (Glock)

wish i understood french
 */




pub fn start_countdown(mut stdout: RawTerminal<std::io::Stdout>){
    
    let (x, y) = termion::terminal_size().unwrap();

    let mut i = 3;
   while i > 0 {
      writeln!( stdout, 
        "{} {} {}",
        termion::clear::All, 
        termion::cursor::Goto(x / 2, y / 2),
        i,
        ).unwrap();
        i-=1;
        thread::sleep(time::Duration::from_millis(1000)); 
  }


}



pub struct Tui<'a> {
    stdout: &'a mut RawTerminal<Stdout>,
    reader: EventStream,
    game: &'a mut Game,

}

impl<'a> Tui<'a> {

    pub fn new(
        stdout: &'a mut RawTerminal<Stdout>,
        reader: EventStream,
        game: &'a mut Game,
        ) -> Tui<'a>{
            Tui { 
                stdout: stdout,
                reader: reader,
                game: game,
             }
    }


}