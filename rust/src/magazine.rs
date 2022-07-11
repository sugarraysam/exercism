use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut magazine_hm = words_to_hashmap(magazine);
    let note_hm = words_to_hashmap(note);

    for (word, n_count) in note_hm {
        match magazine_hm.entry(word) {
            Occupied(mut o) => {
                *o.get_mut() -= n_count;
                if *o.get() < 0 {
                    return false;
                }
            }
            Vacant(_) => return false,
        }
    }
    true
}

fn words_to_hashmap<'a>(words: &[&'a str]) -> HashMap<&'a str, i32> {
    let mut hm = HashMap::new();
    for &word in words {
        let counter = hm.entry(word).or_insert(1);
        *counter += 1;
    }
    hm
}
