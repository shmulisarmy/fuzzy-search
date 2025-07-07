use super::*;

#[test]
fn test_fuzzy_searching_ability() {
    let mut searcher = FuzzySearcher::new(
        "hello".to_string(),
        vec!["hel--lo".to_string(), "h-e-l-l-o".to_string()],
    );
    assert_eq!(
        searcher.search_states[0].matching_indexes.len(),
        searcher.search_term.len()
    );
    assert_eq!(
        searcher.search_states[1].matching_indexes.len(),
        searcher.search_term.len()
    );
}

#[test]
fn test_remove_last_char() {
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
    let mut searcher1 = FuzzySearcher::new(
        "erttlr".to_string(),
        vec![
            "in the sunrise of the falling rainbows".to_string(),
            "where the devil lies so are the tears in his eyes".to_string(),
        ],
    );
    searcher1.remove_last_char_from_search_term();
    searcher1.remove_last_char_from_search_term();
    let mut searcher2 = FuzzySearcher::new(
        "ert".to_string(),
        vec![
            "in the sunrise of the falling rainbows".to_string(),
            "where the devil lies so are the tears in his eyes".to_string(),
        ],
    );
    searcher2.add_char_to_end('t');
    assert_eq!(searcher1, searcher2);
} 