use std::collections::HashMap;
use std::io;

fn main() {
    // 1文字目の変換マッピングを定義
    let mut first_char_map = HashMap::new();
    first_char_map.insert('a', "%1A".to_string());
    first_char_map.insert('b', "%1B".to_string());
    first_char_map.insert('c', "%1C".to_string());
    first_char_map.insert('d', "%1D".to_string());
    first_char_map.insert('e', "%1E".to_string());
    first_char_map.insert('f', "%1F".to_string());
    first_char_map.insert('g', "%20".to_string());
    first_char_map.insert('h', "%21".to_string());
    first_char_map.insert('i', "%22".to_string());
    first_char_map.insert('j', "%23".to_string());
    first_char_map.insert('k', "%24".to_string());
    first_char_map.insert('l', "%08".to_string());
    first_char_map.insert('m', "%26".to_string());
    first_char_map.insert('n', "%27".to_string());
    first_char_map.insert('o', "%28".to_string());
    first_char_map.insert('p', "%29".to_string());
    first_char_map.insert('q', "%2A".to_string());
    first_char_map.insert('r', "%69".to_string());
    first_char_map.insert('s', "%6A".to_string());
    first_char_map.insert('t', "%6B".to_string());
    first_char_map.insert('u', "%6C".to_string());
    first_char_map.insert('v', "%6D".to_string());
    first_char_map.insert('w', "%30".to_string());
    first_char_map.insert('x', "%31".to_string());
    first_char_map.insert('y', "%32".to_string());
    first_char_map.insert('z', "%33".to_string());
    first_char_map.insert('A', "%00".to_string());
    first_char_map.insert('B', "%01".to_string());
    first_char_map.insert('C', "%02".to_string());
    first_char_map.insert('D', "%03".to_string());
    first_char_map.insert('E', "%04".to_string());
    first_char_map.insert('F', "%05".to_string());
    first_char_map.insert('G', "%06".to_string());
    first_char_map.insert('H', "%07".to_string());
    first_char_map.insert('I', "%46".to_string());
    first_char_map.insert('J', "%47".to_string());
    first_char_map.insert('K', "%48".to_string());
    first_char_map.insert('L', "%49".to_string());
    first_char_map.insert('M', "%88".to_string());
    first_char_map.insert('N', "%89".to_string());
    first_char_map.insert('O', "%8A".to_string());
    first_char_map.insert('P', "%8B".to_string());
    first_char_map.insert('Q', "%10".to_string());
    first_char_map.insert('R', "%11".to_string());
    first_char_map.insert('S', "%12".to_string());
    first_char_map.insert('T', "%13".to_string());
    first_char_map.insert('U', "%14".to_string());
    first_char_map.insert('V', "%15".to_string());
    first_char_map.insert('W', "%16".to_string());
    first_char_map.insert('X', "%17".to_string());
    first_char_map.insert('Y', "%18".to_string());
    first_char_map.insert('Z', "%19".to_string());
    first_char_map.insert('0', "%34".to_string());
    first_char_map.insert('1', "%35".to_string());
    first_char_map.insert('2', "%36".to_string());
    first_char_map.insert('3', "%37".to_string());
    first_char_map.insert('4', "%38".to_string());
    first_char_map.insert('5', "%39".to_string());
    first_char_map.insert('6', "%78".to_string());
    first_char_map.insert('7', "%79".to_string());
    first_char_map.insert('8', "%7A".to_string());
    first_char_map.insert('9', "%7B".to_string());

    // 2文字目以降の変換マッピングを定義
    let mut other_chars_map = HashMap::new();
    other_chars_map.insert('a', "X".to_string());
    other_chars_map.insert('b', "Y".to_string());
    other_chars_map.insert('c', "Z".to_string());
    other_chars_map.insert('d', "[".to_string());
    other_chars_map.insert('e', '\\'.to_string());
    other_chars_map.insert('f', "]".to_string());
    other_chars_map.insert('g', "+".to_string());
    other_chars_map.insert('h', "!".to_string());
    other_chars_map.insert('i', '"'.to_string());
    other_chars_map.insert('j', "a".to_string());
    other_chars_map.insert('k', "b".to_string());
    other_chars_map.insert('l', "c".to_string());
    other_chars_map.insert('m', "d".to_string());
    other_chars_map.insert('n', "e".to_string());
    other_chars_map.insert('o', "f".to_string());
    other_chars_map.insert('p', "g".to_string());
    other_chars_map.insert('q', "h".to_string());
    other_chars_map.insert('r', "i".to_string());
    other_chars_map.insert('s', "j".to_string());
    other_chars_map.insert('t', "k".to_string());
    other_chars_map.insert('u', "l".to_string());
    other_chars_map.insert('v', "m".to_string());
    other_chars_map.insert('w', "n".to_string());
    other_chars_map.insert('x', "o".to_string());
    other_chars_map.insert('y', "p".to_string());
    other_chars_map.insert('z', "q".to_string());
    other_chars_map.insert('A', ">".to_string());
    other_chars_map.insert('B', "?".to_string());
    other_chars_map.insert('C', "@".to_string());
    other_chars_map.insert('D', "A".to_string());
    other_chars_map.insert('E', "B".to_string());
    other_chars_map.insert('F', "C".to_string());
    other_chars_map.insert('G', "D".to_string());
    other_chars_map.insert('H', "E".to_string());
    other_chars_map.insert('I', "F".to_string());
    other_chars_map.insert('J', "G".to_string());
    other_chars_map.insert('K', "H".to_string());
    other_chars_map.insert('L', "I".to_string());
    other_chars_map.insert('M', "J".to_string());
    other_chars_map.insert('N', "K".to_string());
    other_chars_map.insert('O', "L".to_string());
    other_chars_map.insert('P', "M".to_string());
    other_chars_map.insert('Q', "N".to_string());
    other_chars_map.insert('R', "O".to_string());
    other_chars_map.insert('S', "P".to_string());
    other_chars_map.insert('T', "Q".to_string());
    other_chars_map.insert('U', "R".to_string());
    other_chars_map.insert('V', "S".to_string());
    other_chars_map.insert('W', "T".to_string());
    other_chars_map.insert('X', "U".to_string());
    other_chars_map.insert('Y', "V".to_string());
    other_chars_map.insert('Z', "W".to_string());
    other_chars_map.insert('0', "r".to_string());
    other_chars_map.insert('1', "s".to_string());
    other_chars_map.insert('2', "t".to_string());
    other_chars_map.insert('3', "u".to_string());
    other_chars_map.insert('4', "v".to_string());
    other_chars_map.insert('5', "w".to_string());
    other_chars_map.insert('6', "x".to_string());
    other_chars_map.insert('7', "y".to_string());
    other_chars_map.insert('8', "z".to_string());
    other_chars_map.insert('9', "{".to_string());

    // ユーザーから入力を受け取る
    println!("素トリップにする文字列を半角英数字10文字で入力してください");

    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("入力に失敗しました");

        // 改行を削除
        let input = input.trim();

        // 入力された文字列が正しい形式であるか判定
        if input.len() != 10 {
            println!("エラー: 文字列は10文字である必要があります");
        } else {
            // 半角英数字以外の文字が含まれているかチェック
            let is_valid = input.chars().all(|c| c.is_ascii_alphanumeric());
            if !is_valid {
                println!("エラー: 半角英数字以外の文字は使えません");
            } else {
                // 入力文字列の変換結果を生成
                let mut result = String::new();
                for (i, c) in input.chars().enumerate() {
                    let converted = if i == 0 {
                        // 1文字目の場合の変換
                        first_char_map.get(&c).unwrap_or(&c.to_string()).clone()
                    } else {
                        // 2文字目以降の場合の変換
                        other_chars_map.get(&c).unwrap_or(&c.to_string()).clone()
                    };
                    result.push_str(&converted);
                }

                // 最後に "a" を追加
                result.push('a');

                // 結果を表示
                println!("素酉キーは {} です", result);
                break;
            }
        }
    }
}
