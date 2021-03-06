use std::fs::File;
use std::io::prelude::*;
use std::io;
extern crate rand;

fn main() {
    let dict = open_dictionary();
    
    let word: String = pick_word(dict);
    let mut right_guesses: Vec<String> = Vec::new();
    let mut wrong_guesses: Vec<String> = Vec::new();
    let mut fouls: usize;

    //set up the game
    display_progress(&right_guesses, &wrong_guesses, &word);
    println!("Begin guessing");
    
    
    loop{
        //update the fouls score, and check to see if the player has lost
        fouls = wrong_guesses.len();
        if fouls >= 8 {
            println!("URK! you have been hanged");
            println!("The word you were looking for was {}", &word);
            break;
        }
        // get a new letter, and decide if it is a good letter or a bad letter
        let new_guess = get_letter();
        if word.contains(&new_guess){
            right_guesses.push(new_guess);
        } else {
            wrong_guesses.push(new_guess);
        }
        //display the player's progress
        display_progress(&right_guesses, &wrong_guesses, &word);

        //check and see if the player has won
        if final_check(right_guesses.clone(), word.clone()){
            display_progress(&right_guesses, &wrong_guesses, &word);
            println!("Good Job!");
            break;
        }
    }
}

fn get_letter() -> String {
    let mut input: String = String::new();
    println!("guess a single letter:");
    loop{
        io::stdin().read_line(&mut input);
        input = String::from(input.trim());
        if input.len() == 1 {
            break;
        } else {
            input = String::new();
            println!("That wasn't one letter");
        }
    }
    input
}


//takes a vector of Strings and concatenates them into one string
fn veccombine(victor: Vec<String>) -> String{
    let mut out: String = String::new();
    for c in victor{
        out = out + &c;
    }
    out
}

fn to_char_vec(victor: Vec<String>) -> Vec<char>{
    let mut output: Vec<char> = Vec::new();
    
    for s in victor{
        let cs: Vec<char> = s.chars().collect();
        for c in cs {
            output.push(c);
        }
    }
    output
}

fn display_progress(rights: &Vec<String>, wrongs: &Vec<String>, word: &String){
    let mut printstring: Vec<String> = Vec::new();
    let charvec: Vec<char> = word.chars().collect();
    for c in charvec{
        if rights.contains(&c.clone().to_string()){
            printstring.push((c.clone().to_string()));
        } else {
            printstring.push(String::from("_ "));
        }
    }
    
    for c in printstring {
        print!("{}", c);
    }
    print!("\nwrong guesses: ");
    for w in wrongs{
        print!("{},", w);
    }
    println!("\nfouls: {}/8", wrongs.len());
}


fn final_check(final_guess: Vec<String>, correct_answer: String) -> bool{

    let mut check = true;
    let correct_chars: Vec<char> = correct_answer.chars().collect();
    let guess_chars: Vec<char> = to_char_vec(final_guess);

    for c in correct_chars{
        if !guess_chars.contains(&c){
            check = false;
        }
    }
    
    check
}

fn str_collect(input: String) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();
    let iter = input.lines();
    for s in iter{
        output.push(s.to_string());
    }
    output
}

//time consuming dictionary open should be run only once.
fn open_dictionary() -> Vec<String>{
    let mut file = File::open("english-words/words.txt").expect("Couldn't find the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    str_collect(contents)
}

fn pick_word(dictionary: Vec<String>) -> String{
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let choice: usize = rng.gen_range(0, dictionary.len());
    let out: String = dictionary[choice].clone();
    out
        
}
