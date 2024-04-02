use clap::Parser;
use std::string::String;

const SUBS_I : &str= "àáâäæãåāăąçćčđďèéêëēėęěğǵḧîïíīįìıİłḿñńǹňôöòóœøōõőṕŕřßśšşșťțûüùúūǘůűųẃẍÿýžźż";
const SUBS_O: &str = "aaaaaaaaaacccddeeeeeeeegghiiiiiiiilmnnnnoooooooooprrsssssttuuuuuuuuuwxyyzzz";

fn slugify(s: &str) -> String {
    let slug = s.to_lowercase();
    let mut new_slug = String::new();
    let mut tmp = ' ';

    for c in slug.chars() {
        let ch = conv(c);

        let opt = Option::Some(ch);
        match opt {
            Some(x) if x == '-' && tmp == '-' => continue,
            _ => (),
        }
        new_slug.push(ch);
        tmp = ch;
    }
    if new_slug.ends_with('-') && new_slug.len() > 1 {new_slug.pop();}

    new_slug
}

///Converti accenti da SUBS_I a SUBS_O
fn conv(c: char) -> char {
    if c.is_ascii_alphanumeric() {
        c
    }
    else {
        match c.len_utf8() {
            2 => if SUBS_I.contains(c) {
                SUBS_O.chars().nth(SUBS_I.chars().position(|x| x == c).unwrap()).unwrap()
            } else { '-' }
            _ => '-'
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conv_accented_letter() {
        assert_eq!(conv('à'), 'a');
    }

    #[test]
    fn test_conv_non_accented_letter() {
        assert_eq!(conv('b'), 'b');
    }

    #[test]
    fn test_conv_unknown_character() {
        assert_eq!(conv('☺'), '-');
    }

    #[test]
    fn test_slugify_multiple_words() {
        assert_eq!(slugify("Hello World"), "hello-world");
    }

    #[test]
    fn test_slugify_accented_characters() {
        assert_eq!(slugify("Èéü"), "eeu");
    }

    #[test]
    fn test_slugify_empty_string() {
        assert_eq!(slugify(""), "");
    }

    #[test]
    fn test_slugify_multiple_consecutive_spaces() {
        assert_eq!(slugify("a  b"), "a-b");
    }

    #[test]
    fn test_slugify_consecutive_invalid_characters() {
        assert_eq!(slugify("##abc"), "-abc");
    }

    #[test]
    fn test_slugify_only_invalid_characters() {
        assert_eq!(slugify("###"), "-");
    }

    #[test]
    fn test_slugify_space_at_end() {
        assert_eq!(slugify("abc "), "abc");
    }

    #[test]
    fn test_slugify_invalid_characters_at_end() {
        assert_eq!(slugify("abc###"), "abc");
    }
}

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args{
    slug_in : String,

    #[arg(short, long, default_value_t = 0)]
    repeat: i32,

    #[arg(short, long, default_value_t = false, action)]
    verbose: bool
}


fn main() {
    let x = Args::parse();
    let slug = slugify(x.slug_in.as_str());

    match x.verbose {
        false => println!("Slug: {}", slug),
        true => for _ in 0..x.repeat {
            println!("Slug: {}", slug);
        }
    }
}
