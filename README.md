# blur

A simple fuzzy search library for Rust.

## Features
- Fuzzy substring matching
- Sorts results by match quality
- Easy to use API

## Usage

Add to your `Cargo.toml`:
```toml
[dependencies]
blur = "0.1.0"
```

## Example
```rust
use blur::FuzzySearcher;

fn main() {
    let mut searcher = FuzzySearcher::new(
        "hello".to_string(),
        vec!["hel--lo".to_string(), "h-e-l-l-o".to_string()],
    );
    searcher.display();
}
```

## License
MIT OR Apache-2.0 