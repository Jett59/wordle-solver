use std::{env, fs};

#[derive(Debug, Clone)]
struct WrongPosition {
    pub impossible_positions: Vec<u32>,
    pub letter_index: usize,
    pub letter: char,
}
#[derive(Debug, Clone)]
struct RightPosition {
    pub letter_index: usize,
    pub letter: char,
}
#[derive(Debug, Clone)]
struct AbsentPosition {
    pub letter: char,
    pub letter_index: usize,
}

#[derive(Debug, Clone)]
enum Fact {
    Absent(AbsentPosition),
    Somewhere(WrongPosition),
    Right(RightPosition),
}

struct WordleResultsIterator<'a, 'b> {
    index: usize,
    possible_words: &'a Vec<String>,
    guess: &'b String,
}

impl Iterator for WordleResultsIterator<'_, '_> {
    type Item = Vec<Fact>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.possible_words.len() {
            return None;
        }
        let wordle = &self.possible_words[self.index];
        let mut facts_for_word = Vec::new();
        let mut used_letter_indices = Vec::new();
        let mut used_wordle_letter_indices = Vec::new();
        for (i, c) in wordle.chars().enumerate() {
            if self.guess.chars().nth(i) == Some(c) {
                used_letter_indices.push(i);
                used_wordle_letter_indices.push(i);
                facts_for_word.push(Fact::Right(RightPosition {
                    letter_index: i,
                    letter: c,
                }));
            }
        }
        for (i, c) in self.guess.chars().enumerate() {
            if !used_letter_indices.contains(&i) {
                if let Some(wordle_match) = wordle
                    .chars()
                    .enumerate()
                    .filter(|(wordle_i, wordle_c)| {
                        *wordle_i != i
                            && *wordle_c == c
                            && !used_wordle_letter_indices.contains(wordle_i)
                    })
                    .nth(0)
                {
                    used_letter_indices.push(i);
                    used_wordle_letter_indices.push(wordle_match.0);
                    facts_for_word.push(Fact::Somewhere(WrongPosition {
                        impossible_positions: vec![i as u32],
                        letter_index: i,
                        letter: c,
                    }));
                }
            }
        }
        for (i, c) in self.guess.chars().enumerate() {
            if !used_letter_indices.contains(&i) {
                facts_for_word.push(Fact::Absent(AbsentPosition {
                    letter: c,
                    letter_index: i,
                }));
            }
        }
        self.index += 1;
        Some(facts_for_word)
    }
}

fn generate_results_for_guess<'a, 'b>(
    words: &'a Vec<String>,
    guess: &'b String,
) -> WordleResultsIterator<'a, 'b> {
    WordleResultsIterator {
        index: 0,
        possible_words: words,
        guess: guess,
    }
}

fn calculate_score<'a, 'b>(possible_resulting_facts: WordleResultsIterator<'a, 'b>) -> f64 {
    let mut score = 0.0;
    for facts in possible_resulting_facts {
        for fact in facts {
            match fact {
                Fact::Absent(_) => score += 0.0,
                Fact::Somewhere(_) => score += 1.0,
                Fact::Right(_) => score += 2.0,
            }
        }
    }
    score
}

fn find_best_word(words: &Vec<String>) -> String {
    let mut best_word = String::new();
    let mut best_score = 0.0;
    for word in words {
        let possible_resulting_facts = generate_results_for_guess(words, &word);
        let score = calculate_score(possible_resulting_facts);
        if score > best_score {
            best_score = score;
            best_word = word.to_string();
            println!("Best so far: {}", best_word);
        }
    }
    best_word
}

fn filter_words(words: &Vec<String>, facts: &Vec<Fact>) -> Vec<String> {
    let mut filtered_words = Vec::new();
    for word in words {
        let mut word_is_valid = true;
        let mut used_indices = Vec::new();
        for fact in facts {
            match fact {
                Fact::Absent(absent) => {
                    if word
                        .chars()
                        .enumerate()
                        .any(|(i, c)| !used_indices.contains(&i) && c == absent.letter)
                    {
                        word_is_valid = false;
                        break;
                    }
                    used_indices.push(absent.letter_index);
                }
                Fact::Somewhere(somewhere) => {
                    if !word.contains(somewhere.letter)
                        || somewhere
                            .impossible_positions
                            .iter()
                            .any(|&i| word.chars().nth(i as usize) == Some(somewhere.letter))
                    {
                        word_is_valid = false;
                        break;
                    }
                    used_indices.push(somewhere.letter_index);
                }
                Fact::Right(right) => {
                    if word.chars().nth(right.letter_index) != Some(right.letter) {
                        word_is_valid = false;
                        break;
                    }
                    used_indices.push(right.letter_index);
                }
            }
        }
        if word_is_valid {
            filtered_words.push(word.clone());
        }
    }
    filtered_words
}

fn solve_wordle(words: &Vec<String>, wordle: &String) {
    let wordle_vec = vec![wordle.clone()];
    let mut possible_words = words.clone();
    let mut found_it = false;
    let mut attempts = 0;
    while !found_it {
        attempts += 1;
        if attempts > 6 {
            println!("I failed!");
            break;
        }
        println!("I have {} words to choose from.", possible_words.len());
        let best_word = find_best_word(&possible_words);
        println!("I guess {}.", best_word);
        let facts = generate_results_for_guess(&wordle_vec, &best_word)
            .next()
            .expect("No facts found");
        possible_words = filter_words(&possible_words, &facts);
        if best_word == *wordle {
            println!("I found it in {}! It's {}.", attempts, possible_words[0]);
            found_it = true;
        }
        if !possible_words.contains(wordle) {
            println!("Its not there?!?!?!");
            println!("Facts that ruled it out: {:?}", facts);
            break;
        }
    }
}

fn main() {
    let words = fs::read_to_string("wordles.txt").expect("Unable to read file");
    let words: Vec<String> = words.split_whitespace().map(String::from).collect();
    let wordle = env::args().nth(1).expect("No wordle provided");
    solve_wordle(&words, &wordle);
}
