use std::fs::File;
use std::io::{self, BufReader, BufRead};
struct Wordglr {
    cur_words: Vec<String>,
    filters: Vec<Vec<Character>>
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

    // no way fancy message???
    println!("+=======================================================================================+");
    println!("+ wordguzzlr v1                                                                         +");
    println!("+ Input sanitization will catch most typos, however there is no back function (yet)     +");
    println!("+ Enter digits for character state (0: not present, 1: present, 2: correct)             +");
    println!("+ Ex:                                                                                   +");
    println!("+ Guess: lares                                                                          +");
    println!("+ 00120                                                                                 +");
    println!("+=======================================================================================+");
    println!("First guess: lares");

    let input = get_input();

    let characters = to_characters("lares".to_string(), input);

    let filters = vec![characters];

    let cur_words = wordguzzle(&filters, &vec);
    // todo: fix
    // let cur_words = sort(cur_words);
    if cur_words.len() == 0 {panic!("filterer returned nothing?????????? (gfooged)")}

    // println!("{:#?}", cur_words);
    let wordglr = Wordglr {
        cur_words,
        filters
    };

    wordler(wordglr)
}

// main loop
fn wordler(mut wordglr: Wordglr) {
    loop {
        println!("Guess: {}", wordglr.cur_words[0]);
        
        let input = get_input();

        let characters = to_characters(wordglr.cur_words[0].to_string(), input);
        // println!("{:?}", characters);
        wordglr.filters.push(characters);

        let cur_words = wordguzzle(&wordglr.filters, &wordglr.cur_words);
        // let cur_words = sort(cur_words);
        if cur_words.len() == 0 {panic!("filterer returned nothing?????????? (gfooged)")}

        // println!("{:#?}", cur_words.len());
        wordglr.cur_words = cur_words;
        

        if wordglr.cur_words.len() <= 1 {
            println!("Final: {}", wordglr.cur_words[0]);
            break
        }
    }
}

fn to_characters(guess: String, input: String) -> Vec<Character> {
    let mut guess_arr = guess.chars();
    let mut vec: Vec<Character> = Vec::new();

    for char in input.chars() {
        let next = match guess_arr.next() {
            Some(item) => item,
            None => continue
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

// annoy the user until they input something correctly
fn get_input() -> String {
    let mut input = String::new();

    io::stdin().read_line(&mut input)
    .expect("Error reading input");

    // remove io::stdin carriage return fuckery
    input.pop()
    .expect("wtf???");
    input.pop()
    .expect("wtf???");

    // check input len
    if input.chars().count() != 5 {
        println!("Invalid input length (5)");
        return get_input()
    }

    // check if string is only digits
    if !input.bytes().all(|c| c.is_ascii_digit()) {
        println!("Input can only be digits");
        return get_input()
    }

    // check if digits are within bounds
    if !input.chars().all(|c| {
        let digit = c.to_digit(10);
        let digit = match digit {
            Some(c) => c,
            None => panic!("wtf!!!")
        };

        if digit > 2 {
            false
        } else {true}
    }) {
        println!("Input not within range (<2)");
        return get_input()
    }

    input
}

// eat words (cum (gfoog) (real))
fn wordguzzle(filters: &Vec<Vec<Character>>, words: &Vec<String>) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();
    
    let mut confirmeds = 0;
    let mut yellowed = 0;

    let filter = &filters[filters.len() - 1];
    for character in filter {
        match character.state {
            States::Confirmed => confirmeds += 1,
            States::Present => yellowed += 1,
            States::Nah => continue
        }
    }

    // println!("{}", yellows);
    for word in words {
        let refrence = word.clone();
        let mut char_arr = refrence.chars();

        let mut confirmed = 0;
        let mut yellows = 0;

        let mut dont_push = false;

        let mut its = 0;
        for character in filter {
            let next = match char_arr.next() {
                Some(char) => char,
                None => continue
            };

            if next == character.char && character.state == States::Confirmed {
                // check for confirmed characters
                confirmed += 1;
                continue;
            }

            // check for yellow (pee!!!) chars
            if next == character.char && character.state == States::Present {
                // check if previous guess's yellow is in the same place
                for i in 0..filters.len() {
                        // if so, discard word
                        if filters[i][its].char == next {
                        dont_push = true;
                    }
                }
            }
            // check if word has same yellow char but in different place
            if word.contains(character.char) && character.state == States::Present {
                yellows += 1;
            }

            // check for non present characters
            if next == character.char && character.state == States::Nah {
                // discard word if it contains not present chars, however only when that char isnt confirmed or present
                for i in 0..word.len() {
                    if !((filter[i].state == States::Confirmed && filter[i].char == next)
                     || (filter[i].state == States::Present && filter[i].char == next)) {
                        dont_push = true;
                    }
                }
            }
            // discard word if non present chars was present in the previous guesses
            // bug: if two letters are the same and one is yellow and other is not
            // will discard word anyway
            for filter in filters {
                for char in filter {
                    // println!("{:?}", char.char == character.char);
                    if char.char == next && char.state == States::Nah {
                        dont_push = true;
                    }
                }
            }

            its += 1;
        }
        
        if (confirmed == confirmeds) && (yellowed == yellows) && !dont_push {
            // println!("{}", dont_push);
            res.push(word.to_string())
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

    Ok(vec)
}

// shut up!!!
#[allow(dead_code)]

// sort words by how many unique chars they have
// eliminates more characters
struct Item {
    string: String,
    uniqueness: u16
}

// who asked????
#[allow(dead_code)]

fn sort(words: Vec<String>) -> Vec<String> {
    let mut vec: Vec<Item> = Vec::new();
    for word in words {
        // O(n^3)?
        vec.push(Item { uniqueness: unique(&word), string: word })
    }

    vec.sort_by(|a, b| a.uniqueness.cmp(&b.uniqueness));

    let mut nvec: Vec<String> = Vec::with_capacity(vec.len());

    for item in vec {
        nvec.push(item.string)
    }
    nvec.reverse();

    nvec
}

fn unique(word: &String) -> u16 {
    let mut uniqueness = word.len() as u16;
    let mut its = 0;

    for char in word.chars() {
        let mut its2 = 0;
        for char2 in word.chars() {
            if char2 == char && its2 == its {
                println!("{}: {}", uniqueness, word);
                uniqueness -= 1;
            }
            its2 +=1;
        }
        its += 1;
    }
    uniqueness
}
