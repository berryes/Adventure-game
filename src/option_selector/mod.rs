
use std::{io::{stdin, stdout, Write}, clone};
use crossterm::{execute, event::DisableMouseCapture};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use crate::{input_handler::{FilterInputStreamForArrows, GameDirectionKey}, game::Character};


pub fn chooser(
    options: Vec<String>,
    display_text: String
    ) -> i8{

    let (x, y) = termion::terminal_size().unwrap();
    let termWidth = usize::try_from(x).expect("failed to covnert");

    let mut selectionText: String = String::new();
    // putting text in the middle
    for i in 0..termWidth {
        if ( termWidth - display_text.len() ) / 2 <= i {
        selectionText.push_str(display_text.as_str());
        break
        } 
                selectionText.push_str(" ")
     }


    println!("{}",selectionText);

    return 3
}   




pub async fn chooseCharacter(characters: Vec<Character>,) -> usize {

    let (x, y) = termion::terminal_size().unwrap();
    let termWidth = usize::try_from(x).expect("failed to covnert");


    let mut index = 0;

    // console 
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    execute!(stdout, DisableMouseCapture); // reduce bloatware xd


    writeln!( stdout, 
        "{} {} {} {} {} {} {} {} {} ", 
        termion::clear::All,

        termion::cursor::Goto(1,1),
        characters[index].name,
        
        termion::cursor::Goto(1,2),
        format!("attack: {}", characters[index].weapon.normal),
        
        termion::cursor::Goto(1,3),
        format!("health: {}", characters[index].health),

        termion::cursor::Goto(1,4),
        format!("protection: {}", characters[index].armour.normal),
        ).unwrap();



    for keyPress in stdin.keys() {

        match FilterInputStreamForArrows(keyPress.expect("Failed to read key")) {
            
            // do nothing
            GameDirectionKey::DownArrow => { }
            GameDirectionKey::UpArrow => { }

            // selection 
            GameDirectionKey::LeftArrow => {
                if index !=  0  {
                    index -= 1;

                    writeln!( stdout, 
                        "{} {} {} {} {} {} {} {} {} ", 
                        termion::clear::All,
                
                        termion::cursor::Goto(1,1),
                        characters[index].name,
                        
                        termion::cursor::Goto(1,2),
                        format!("attack: {}", characters[index].weapon.normal),
                        
                        termion::cursor::Goto(1,3),
                        format!("health: {}", characters[index].health),
                
                        termion::cursor::Goto(1,4),
                        format!("protection: {}", characters[index].armour.normal),
                        ).unwrap();

                    
                }else {
                    // do nothing
                }

            }
            GameDirectionKey::RightArrow => {
                if index +1 < characters.len()  {
                    index += 1;
                    writeln!( stdout, 
                        "{}  {} {} {} {} {} {} {} {} ", 
                        termion::clear::All,
                
                        termion::cursor::Goto(1,1),
                        characters[index].name,
                        
                        termion::cursor::Goto(1,2),
                        format!("attack: {}", characters[index].weapon.normal),
                        
                        termion::cursor::Goto(1,3),
                        format!("health: {}", characters[index].health),
                
                        termion::cursor::Goto(1,4),
                        format!("protection: {}", characters[index].weapon.normal),

                        ).unwrap();
                }else {
                    // do nothing
                }
            }
        
                GameDirectionKey::Enter => {
                    return index
                }

            GameDirectionKey::Void => {
            } 

        }
    }

    // returning character
    index
}