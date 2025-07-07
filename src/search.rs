use std::fmt;

/// Represents the state of a string being searched against.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct BeingSearchedState {
    pub text: String,
    pub matching_indexes: Vec<usize>,
}

/// The main fuzzy searcher struct.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct FuzzySearcher {
    pub search_states: Vec<BeingSearchedState>,
    pub search_term: String,
}

impl FuzzySearcher {
    /// Create a new fuzzy searcher with a search term and a list of strings to search against.
    pub fn new(search_term: String, searching_against: Vec<String>) -> Self {
        let mut res = FuzzySearcher {
            search_states: Vec::new(),
            search_term,
        };
        for text in searching_against {
            res.search_states.push(BeingSearchedState {
                text,
                matching_indexes: Vec::new(),
            });
        }

        for search_state in res.search_states.iter_mut() {
            let mut search_state_index = 0;
            let mut search_term_index = 0;
            while search_state_index < search_state.text.len()
                && search_term_index < res.search_term.len()
            {
                if search_state.text.chars().nth(search_state_index).unwrap()
                    == res.search_term.chars().nth(search_term_index).unwrap()
                {
                    search_state.matching_indexes.push(search_state_index);
                    search_term_index += 1;
                }
                search_state_index += 1;
            }
        }
        res.search_states
            .sort_by(|a, b| b.matching_indexes.len().cmp(&a.matching_indexes.len()));
        res
    }

    /// Display the search results to stdout, highlighting matches in green.
    pub fn display(&self) {
        fn green(text: &str) -> String {
            format!("\x1b[32m{}\x1b[0m", text)
        }
        println!();
        println!();
        println!();
        for search_state in self.search_states.iter() {
            for (i, index) in search_state.matching_indexes.iter().enumerate() {
                if i > 0 {
                    let last_one = search_state.matching_indexes[i - 1];
                    for j in last_one + 1..*index {
                        print!("{}", &search_state.text.chars().nth(j).unwrap().to_string());
                    }
                } else {
                    for j in 0..*index {
                        print!("{}", &search_state.text.chars().nth(j).unwrap().to_string());
                    }
                }
                print!(
                    "{}",
                    green(&search_state.text.chars().nth(*index).unwrap().to_string())
                );
            }
            if search_state.matching_indexes.len() > 0 {
                for j in search_state.matching_indexes[search_state.matching_indexes.len() - 1] + 1
                    ..search_state.text.len()
                {
                    print!("{}", &search_state.text.chars().nth(j).unwrap().to_string());
                }
            }
            dbg!(search_state.matching_indexes.len());
            println!();
            println!();
        }
        println!();
    }

    /// Remove the last character from the search term and update matches.
    pub fn remove_last_char_from_search_term(&mut self) {
        for search_state in self.search_states.iter_mut() {
            if search_state.matching_indexes.len() == self.search_term.len() {
                search_state.matching_indexes.pop();
            }
        }
        self.search_term.pop();
    }

    /// Add a character to the end of the search term and update matches.
    pub fn add_char_to_end(&mut self, letter: char) {
        let mut search_state_indexes_to_push_up = Vec::new();
        for (i, search_state) in (&mut self.search_states).iter_mut().enumerate() {
            if search_state.matching_indexes.len() == self.search_term.len() {
                let mut look_ahead_by = 1;
                while search_state.matching_indexes[search_state.matching_indexes.len() - 1]
                    + look_ahead_by
                    < search_state.text.len()
                {
                    let next_place_to_compare_in_text = search_state.matching_indexes
                        [search_state.matching_indexes.len() - 1]
                        + look_ahead_by;
                    if search_state
                        .text
                        .chars()
                        .nth(next_place_to_compare_in_text)
                        .unwrap()
                        == letter
                    {
                        search_state
                            .matching_indexes
                            .push(next_place_to_compare_in_text);
                        search_state_indexes_to_push_up.push(i);
                        break;
                    }
                    look_ahead_by += 1;
                }
            }
        }
        for index in search_state_indexes_to_push_up {
            single_push_up_sort(&mut self.search_states, index);
        }
        self.search_term.push(letter);
    }
}

fn single_push_up_sort(search_states: &mut Vec<BeingSearchedState>, start_index: usize) {
    let mut current_sorting_index = start_index;
    while current_sorting_index > 0
        && search_states[current_sorting_index].matching_indexes.len()
            > search_states[current_sorting_index - 1]
                .matching_indexes
                .len()
    {
        search_states.swap(current_sorting_index, current_sorting_index - 1);
        current_sorting_index -= 1;
    }
} 