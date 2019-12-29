use regex::Regex;
use std::io::prelude::*;
use std::{env, fs, io, process};

/// Opens a text file and returns the content as String
fn open_file(file_path: &str) -> Result<String, io::Error> {
    let mut file = fs::File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

/// Replace a word within text by another, while ignoring certain ranges of the
/// string
fn replace_ignoring_words(
    text: &String,
    replace_word: &str,
    replace_with: &str,
    ignore_ranges: &Vec<std::ops::Range<usize>>,
) -> String {
    let mut edited_text = text.clone();
    let re = Regex::new(replace_word).unwrap();
    
    let replace_shift = replace_with.len() - replace_word.len();
    let mut byte_pos = 0;
    let mut track_shift = 0;
    'replace_str: loop {
        if let Some(matched_pos) = re.find_at(text, byte_pos) {
            let start_idx = matched_pos.start();
            let end_idx = matched_pos.end();
            byte_pos = end_idx;
            for range in ignore_ranges.iter() {
                if start_idx >= range.start && end_idx <= range.end {
                    continue 'replace_str;
                } else {
                    edited_text.replace_range(
                        start_idx+track_shift .. end_idx+track_shift,
                        replace_with);
                    track_shift += replace_shift;
                }

            }
        } else{
            break;
        }
    }
    edited_text
}

/// Searches `text` for Umlaute (oe, ae, ue) and replaces with (ö, ä, ü)
/// while ignoring replacements in words given in `ignore_words`
fn replace_umlaute(text: &String, ignore_words: Option<Vec<String>>) -> String {
    
    // extract from ignore_words which ranges are not allowed
    let mut not_allowed_ranges = Vec::new();
    if let Some(ign_strings) = ignore_words {
        for ign_string in ign_strings.iter() {
            let re_ign = Regex::new(ign_string).unwrap();
            for matched_ign in re_ign.find_iter(text) {
                not_allowed_ranges.push(matched_ign.start()..matched_ign.end());
            }
        }
    }

    // replace umlaute
    let text = replace_ignoring_words(text, "oe", "ö", &not_allowed_ranges);
    let text = replace_ignoring_words(&text, r"ae", "ä", &not_allowed_ranges);
    let text = replace_ignoring_words(&text, r"ue", "ü", &not_allowed_ranges);
    text
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Provide path to textfile.");
        process::exit(1);
    }

    let file_content = open_file(&args[1]).unwrap();

    // let file_content = replace_umlaute(file_content, None);
    let file_content = replace_umlaute(&file_content, Some(vec!["Dominique".to_string()]));
    println!("{}", file_content);
}
