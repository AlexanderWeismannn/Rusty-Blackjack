#![allow(unused)]

//player class
mod player;
use player::Player;

//file and env vars
use::std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::env::current_dir;

//gameplay and rng 
use rand::Rng;
use std::io::{self, BufReader, BufRead};
use std::cmp::Ordering;
use map_in_place::MapVecInPlace;
use std::thread;
use std::time::Duration;
use std::process::exit;


fn main() {

    //Random Set Of Cards
    //Generates an "empty" vector and maps a num between 2 and 11, 104 times. Creating a pseudo-random deck of cards to mimic a casino
    let empty_card_deck = vec![0;104];
    let mut card_deck = empty_card_deck.map_in_place(|x| x + rand::thread_rng().gen_range (2..=11)); 

    //Check if a player has been created already (i.e. does  a json file exist)
    let path = Path::new(".\\src");
    let extension = "txt";

    let mut file_found = false;

    //default player
    let mut p1: Player = Player{name: String::from("Player"), hand: vec![],balance: 100, sum: 0};  


    //check for a text file from previous session
    for entry in fs::read_dir(path).unwrap(){
        let file_path = entry.unwrap().path();
        //println!("{:?}",file_path);

        if file_path.is_file() && file_path.extension().unwrap() == extension{
            //Yes - read the text file for the number and generate the player off of it
            file_found = true;
            let file = File::open(file_path).unwrap();
            let reader = BufReader::new(file);

            let line = reader.lines().next().unwrap().unwrap();
            
            println!("Previous Game Found, Generating Player Data: {}", line);
            let val: i32 = line.parse().unwrap();
            p1.balance = val;
        }
    }


    //generate the dealer (name, and balance dont matter)
    let mut p2: Player = Player{name: String::from("Dealer"), hand: vec![], balance: 1000000, sum: 0};

    println!("<==========|| Welcome {} to: ||==========>",p1.name);
    println!(r" _     _            _    _            _    ");
    println!(r"| |   | |          | |  (_)          | |   ");
    println!(r"| |__ | | __ _  ___| | ___  __ _  ___| | __");
    println!(r"| '_ \| |/ _` |/ __| |/ / |/ _` |/ __| |/ /");
    println!(r"| |_) | | (_| | (__|   <| | (_| | (__|   < ");
    println!(r"|_.__/|_|\__,_|\___|_|\_\ |\__,_|\___|_|\_|");
    println!(r"                       _/ |                ");
    println!(r"                      |__/                 ");
    
    
    //game loop
    loop{

        //the player bets
        //Also makes sure they can bet that amount / entered an appropriate value type (<i32>)
        let mut bet_val: i32;


        loop{
            println!("Balance: {}",p1.balance);
            println!("Please Enter Your Bet: (x to exit)");
            //make a copy of the balance
            let mut p_input = String::new();
            io::stdin().read_line(&mut p_input)
            .expect("Failed to user input from line");


            //check for user input, exit on "x"
            match p_input.trim().parse::<i32>(){
                Ok(n) if n > 0 => {
                    if n <= p1.balance {
                        bet_val = n;
                        break;
                    } else {
                        continue;
                    } 
                },
                _ => if p_input.trim() == "x"{
                    std::process::exit(0);
                },
                Err(_) => continue, 
            };
        }

        //call the bet method
        p1.bet(bet_val);
        println!("Leftover Money: {}\n", p1.balance);

        //player is given an initial hand (1 card)
        let card_sum = p1.draw_card(&mut card_deck);
        println!("Card Drawn!",);
        println!("Current Hand: {:?}",p1.hand);

        //loop asking if the player wants to keep drawing cards
        //they choose to draw a new one until they are at 21 or over
        loop{
            println!("Draw another Card? (y/n): ");
            let mut p_input = String::new();
            io::stdin().read_line(&mut p_input)
            .expect("Failed to user input from line");

            //check for user input
            if p_input.trim() == "y"{
                p1.draw_card(&mut card_deck);
                println!("Card Drawn!",);
                println!("Current Hand: {:?}",p1.hand);
            }else if p_input.trim() == "n"{
                break;
            }else{
                println!("Please Enter A Correct Option");
                continue;
            }

            //force a break out of the loop if the playre goes over
            if p1.sum_value() > 21{
                break
            }
        }

        //ULTRA INTELLIGENT AI 
        //The dealer draws until the have reached 17 or over
        println!("Dealer Drawing:");
        loop{
            p2.draw_card(&mut card_deck);
            if p2.sum_value() >= 17{
                break;
            }else{
                continue
            }
        }

        //reveal to the player what the dealer drew
        for card in p2.hand.iter(){
            println!("Dealer Draws: {}",card);
            thread::sleep(Duration::from_secs(1));
        }

        //Compare the results of the player and the dealer in a fancy way (i.e. build up tension with wait)
        if p1.sum_value() > p2.sum_value() && (p1.sum_value() <= 21) {
            //player wins
            println!("You Win! {} > {}",p1.sum_value(),p2.sum_value());
            p1.balance = p1.balance + (bet_val * 2);
        }else if p1.sum_value() < p2.sum_value() && (p2.sum_value() <= 21){
            //dealer wins
            println!("You Lose! {} < {}",p1.sum_value(),p2.sum_value());
        }else{
            //its a tie and no one wins
            println!("Its a tie");
            p1.balance += bet_val;
        }

        println!("Your New Balance: {}",p1.balance);
        thread::sleep(Duration::from_secs(3));

        //check if the player is broke
        if p1.balance == 0 {
            println!("You're Broke! Get out of here!");
            break;
        }

        //updates the players winnings before looping
        let mut file = File::create([path.to_str().unwrap(),"p1.txt"].join("\\")).unwrap();
        file.write_all(&p1.balance.to_string().as_bytes()).unwrap();
        
    }


}


