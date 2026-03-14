use std::fs;
use std::env;
use std::collections::HashMap;

fn main() {
    // ファイル読込
    let args: Vec<String> = env::args().collect();
    
    let file_name = &args[1];

    let contents = fs::read_to_string(file_name)
        .expect("ファイルの読み込みに失敗しました");
    
    // println!("ファイルの内容:\n{}", contents);

    // 単語のカウント
    let mut map: HashMap<String, u32> = HashMap::new();

    for word in contents.split_whitespace() {
        let clean_word = word
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect::<String>()
            .to_lowercase();

        if clean_word.is_empty() {
            continue;
        }

        let entry = map.entry(clean_word).or_insert(0);
        *entry += 1;
    }

    // println!("{:?}", map);

    // 結果の表示
    let mut vec: Vec<(&String, &u32)> = map.iter().collect();

    vec.sort_by(|a, b| b.1.cmp(a.1));

    for (i, (word, count)) in vec.iter().take(10).enumerate() {
        println!("{}位: {} - {}", i + 1, word, count);
    }
}