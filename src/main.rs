use std::io;

fn green(text: &str) -> String {
    format!("\x1b[32m{}\x1b[0m", text)
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct BeingSearchedState {
    text: String, //as of now the assumption is that the text is static, if it changes, then you remove and readd it, (imediate mode)
    matching_indexes: Vec<usize>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct FuzzySearcher {
    search_states: Vec<BeingSearchedState>,
    search_term: String,
}

impl FuzzySearcher {
    fn new(search_term: String, searching_against: Vec<String>) -> Self {
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
            .sort_by(|a, b| b.matching_indexes.len().cmp(&a.matching_indexes.len())); //we want them to be sorted by the number of matches, from most to least
        res
    }

    fn display(&self) {
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
            for j in search_state.matching_indexes[search_state.matching_indexes.len() - 1] + 1
                ..search_state.text.len()
            {
                print!("{}", &search_state.text.chars().nth(j).unwrap().to_string());
            }
            dbg!(search_state.matching_indexes.len());
            println!();
            println!();
        }
        println!();
    }

    fn remove_last_char_from_search_term(&mut self) {
        for search_state in self.search_states.iter_mut() {
            if search_state.matching_indexes.len() == self.search_term.len() {
                search_state.matching_indexes.pop();
            }
        }
        self.search_term.pop();
    }
    fn add_char_to_end(&mut self, letter: char) {
        let mut search_state_indexes_to_push_up = Vec::new(); // when changing values, do a bit of resorting, but we cant mutate that while iterating through a mutable vector
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

impl BeingSearchedState {}

fn main() {
    let mut searcher = FuzzySearcher::new(
        "helloyo".to_string(),
        vec![
            "he laloynnop".to_string(),
            "world has gone lolonlyodkasp".to_string(),
        ],
    );
    // searcher.display();
    // searcher.remove_last_char_from_search_term();
    println!("Enter your name:");
    while true {
        searcher.display();

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.chars().nth(0).unwrap() {
            'q' => break,
            '/' => searcher.remove_last_char_from_search_term(),
            _ => searcher.add_char_to_end(input.chars().nth(0).unwrap()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_fuzzy_searching_ability() {
        let mut searcher = FuzzySearcher::new(
            "hello".to_string(),
            vec!["hel--lo".to_string(), "h-e-l-l-o".to_string()],
        );
        assert_eq!(searcher.search_states[0].matching_indexes.len(), searcher.search_term.len()); 
        assert_eq!(searcher.search_states[1].matching_indexes.len(), searcher.search_term.len());
    }

    #[test]
    fn test_remove_last_char() {
        //the point is to make sure that removing a char yields out a similar state to never having that char before there at all
        let mut searcher1 = FuzzySearcher::new(
            "hello".to_string(),
            vec!["hello".to_string(), "world".to_string()],
        );
        searcher1.remove_last_char_from_search_term();
        let searcher2 = FuzzySearcher::new(
            "hell".to_string(),
            vec!["hello".to_string(), "world".to_string()],
        );
        assert_eq!(searcher1, searcher2);
    }

   

    #[test]
    
    fn test_add_char() {
        // the point is to make sure that adding a char yields out a similar state to having started with that char in the search term
        let mut searcher1 = FuzzySearcher::new(
            "hell".to_string(),
            vec!["hello".to_string(), "world".to_string()],
        );
        searcher1.add_char_to_end('o');
        let searcher2 = FuzzySearcher::new(
            "hello".to_string(),
            vec!["hello".to_string(), "world".to_string()],
        );
        assert_eq!(searcher1, searcher2);
    }
    #[test]
    fn test_remove_and_add() {
        //this combines the idea from the past two tests to simulate a more real life use case
        let mut searcher1 = FuzzySearcher::new(
            "erttlr".to_string(),
            vec!["in the sunrise of the falling rainbows".to_string(), "where the devil lies so are the tears in his eyes".to_string()],
        );
        searcher1.remove_last_char_from_search_term();
        searcher1.remove_last_char_from_search_term();
        let mut searcher2 = FuzzySearcher::new(
            "ert".to_string(),
            vec!["in the sunrise of the falling rainbows".to_string(), "where the devil lies so are the tears in his eyes".to_string()],
        );
        searcher2.add_char_to_end('t');
        assert_eq!(searcher1, searcher2);
    }
}

fn single_push_up_sort(search_states: &mut Vec<BeingSearchedState>, start_index: usize) {
    //after we've just raised the item at this indexes value, we want to push it as much to the top as we need to, to ensure its ordered
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
