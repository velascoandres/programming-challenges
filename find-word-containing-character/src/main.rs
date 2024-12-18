pub fn find_words_containing(words: Vec<String>, x: char) -> Vec<i32> {
    if words.is_empty() {
        return vec![];
    }

    let mut results: Vec<i32> = vec![];

    for (index, word) in words.into_iter().enumerate() {
        if word.contains(x) {
            results.push(index as i32);
        }
    }

    results
}

fn main() {
    let case = ["abc", "bcd", "aaaa", "cbc"]
        .into_iter()
        .map(String::from)
        .collect::<Vec<String>>();

    assert_eq!(find_words_containing(case, 'a'), [0, 2]);
}
