use std::fs::File;
use std::io::{self, BufReader, BufRead};

struct Wordglr {
    all_words: Vec<String>,
    cur_words: Vec<String>,

    characters: Vec<Character>,

    game_states: Vec<Vec<Character>>
}

#[derive(Debug)]
struct Character {
    char: char,
    state: States
}

#[derive(PartialEq, Debug)]
enum States {
    Confirmed,
    Present,
    Nah
}

fn main() {
    let vec = read_file("words.txt")
    .expect("File read error");

    println!("First guess: lares");

    let mut input = String::new();

    io::stdin().read_line(&mut input)
    .expect("Error reading input");

    let characters = to_characters("lares".to_string(), input);

    let wordglr = Wordglr {
        all_words: vec,
        cur_words: vec![],
        characters: characters,
        game_states: vec![vec![]]
    };

    main();
}

fn to_characters(guess: String, input: String) -> Vec<Character> {
    let mut guess_arr = guess.chars();
    let mut vec: Vec<Character> = Vec::new();

    for char in input.chars() {
        let next = match guess_arr.next() {
            Some(item) => item,
            None => panic!("wtf?")
        };

        if char == '2' {
            vec.push(Character { char: next, state: States::Confirmed });
        } else if char == '1' {
            vec.push(Character { char: next, state: States::Present });
        } else if char == '0' {
            vec.push(Character { char: next, state: States::Nah });
        }
    }

    vec
}

fn wordguzzle(filter: Vec<Character>, words: Vec<String>) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();
    
    for word in words {
        let refrence = &word;
        let mut char_arr = refrence.chars();

        for character in &filter {
            let next = match char_arr.next() {
                Some(char) => char,
                None => panic!("wtf!!!!!!")
            };

            if next == character.char && character.state == States::Confirmed {
                res.push(word)
            } else if next == character.char && character.state == States::Present {
                res.push(word)
            } else if next == character.char && character.state == States::Nah {
                res.push(word)
            }
        }
    }

    res
}

fn read_file(name: &str) -> Result<Vec<String>, io::Error> {
    // words.txt is 12972 lines
    let mut vec: Vec<String> = Vec::with_capacity(12972);

    let file = File::open(name)?;
    
    let reader = BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(str) => vec.push(str),
            Err(err) => panic!("Error parsing file: {}", err)
        };
    }
    println!("{}", vec.capacity());

    Ok(vec)
}
