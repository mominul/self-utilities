use std::collections::{BTreeMap, HashMap};
use std::fs::read;

fn main() {
    let dict: HashMap<String, Vec<String>> =
        serde_json::from_slice(&read("dictionary.json").unwrap()).unwrap();
    let emoji_list: HashMap<String, Vec<String>> =
        serde_json::from_slice(&read("emoji-bn.json").unwrap()).unwrap();
    let mut listed_emojis: HashMap<String, Vec<String>> = HashMap::new();

    for (emoji, codes) in emoji_list {
        // Ignore entries which have spaces, we'll need to figure out what to do about them later.
        if codes.iter().any(|s| s.contains(" ")) {
            continue;
        }

        for code in codes {
            listed_emojis
                .entry(code.to_owned())
                .or_default()
                .push(emoji.to_owned());
        }
    }

    let mut emojis: HashMap<char, Vec<(String, Vec<String>)>> = HashMap::new();

    for (name, emoji) in listed_emojis {
        let ch = name.chars().nth(0).unwrap();
        emojis.entry(ch).or_default().push((name, emoji));
    }

    let mut new_dict: BTreeMap<String, Vec<(String, Vec<String>)>> = BTreeMap::new();

    for (table, vec) in dict {
        let mut words = Vec::with_capacity(vec.capacity());

        for word in vec {
            let ch = word.chars().nth(0).unwrap();
            let emoji = emojis
                .get(&ch)
                .map(|vec| vec.iter().find(|s| s.0 == word).map(|s| &s.1))
                .flatten()
                .cloned()
                .unwrap_or_default();
            words.push((word, emoji));
        }

        new_dict.insert(table, words);
    }
    
    // let dict = ron::ser::to_string_pretty(&new_dict, ron::ser::PrettyConfig::default()).unwrap();
    let dict = ron::to_string(&new_dict).unwrap();

    std::fs::write("dictionary.ron", dict).unwrap();
    println!("Done!");
}
