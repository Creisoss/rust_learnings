use std::ops::Add;
fn set_ay(s: &str, ch: char) -> String{
    let mut st = s.to_string().add(ch.to_string().as_str()); 
    st.remove(0);
    st = st + "ay";
    st
}

fn set_hay(s: &str, ch: char) -> String{
    let mut st = s.to_string().add(ch.to_string().as_str()); 
    st.remove(0);
    st = st + "hay";
    st
}
fn pig_latin(s: &str) -> String {
    let st = s.clone().to_string() + " ";
    let mut void: String = "".to_string();
    let mut new_word: bool = true;
    let mut indexf = 0;
    let mut indexb = 0;
    let mut first_letter: char = 'a';
    let mut use_ay: bool = true;
    for i in st.to_lowercase().bytes(){
        let ch = i as char;
        if new_word && ch.is_alphabetic(){
            new_word = false;
            indexb = indexf;
            first_letter = ch;
            if (ch == 'a') || (ch == 'e') || (ch == 'i') || (ch == 'o') || (ch == 'u'){ 
                use_ay = true
            }else{
                use_ay = false
            }
        }
        if ch == ' '{
            new_word = true;
            if use_ay{
                void = void + (set_ay(&st[indexb..indexf], first_letter)).as_str();
            }else{
                void = void + (set_hay(&st[indexb..indexf], first_letter)).as_str();
            }   
            void = void + " ";
        } 
        indexf += 1;
    }
    void
}

fn main() {
    println!("{0}", pig_latin("Brazil"));
    println!("{0}", pig_latin("Germany"));
    println!("{0}", pig_latin("United States of America"));
    println!("{0}", pig_latin("Ukraine"));
    println!("{0}", pig_latin("United kingdon"));
    println!("{0}", pig_latin("Australia"));
    println!("{0}", pig_latin("Russia"));
}
