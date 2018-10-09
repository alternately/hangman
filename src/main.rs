
use std::io;

fn main() {
    let word: String = String::from("foobar");
    let mut right_guesses: Vec<String> = Vec::new();
    let mut wrong_guesses: Vec<String> = Vec::new();
    
    
    loop{
        
        display_progress(&right_guesses, &wrong_guesses, &word);
        &right_guesses.push(get_letter());
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
    println!("");
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
