extern crate core;

fn main() {

    let language_1 = Language::new(String::from("code a"));
    let language_2 = Language::new(String::from("code a"));
    let language_3 = Language::new(String::from("code b"));
    
    println!("{} {}", language_1 == language_2, language_1 == language_3);
}

#[derive(Eq, PartialEq)]
struct Language {
    code: String
}

impl Language {
    fn undefined() -> Language {
        Language {code: String::new()}
    }
    fn new(code: String) -> Language {
        Language {code: code}
    }
}

struct Word {
    word: String,
    language: Language
}

impl Word {
    fn new(word: String) -> Word {
        Word {
            word: word,
            language: Language::undefined()
        }
    }
    
    fn has_defined_language(&self) -> bool {
        self.language != Language::undefined()
    }
}
