use bitcoin_hashes::sha256;
use bitcoin_hashes::Hash;

pub fn sort_two(text1: &str, text2: &str) -> (String, String) {
            
    let mut t1 = text1;
    let mut t2 = text2;

    if text1 > text2 {
        t2 = text1;
        t1 = text2;
    }
    
    (t1.to_string(), t2.to_string())
}

pub fn hash_it(it: &str) -> String {

    sha256::Hash::hash(it.as_bytes()).to_string()
}

pub fn sort_cat_n_hash(text1: &str, text2: &str) -> String {
    let sorted = sort_two(text1, text2);
    cat_n_hash(sorted.0, sorted.1)
}


pub fn cat_n_hash(left: String, right: String) -> String {

    let mut concat: String = left.clone();
    concat.push_str(&right);
    sha256::Hash::hash(concat.as_bytes()).to_string()
}

//old create_triple
pub fn concat_n_hash(text1: &str, text2: &str) -> (String, String, String) {
//println!("t1:{} t2:{}",text1,text2);
    let sorted = sort_two(text1.trim_matches('"'), text2.trim_matches('"'));
    let id = cat_n_hash(sorted.0.clone(), sorted.1.clone());
//println!("t1:{} t2:{}", sorted.0, sorted.1);
    (id, sorted.0, sorted.1)
}

pub fn hash_text(text: &str) -> String {
    let hash_of_string = sha256::Hash::hash(text.as_bytes());
    hash_of_string.to_string()
}

pub fn push_hashed(id: &str, name: &str, label: &str, description: &str) -> String{
    let mut pushed: String = id.to_string();
    pushed.push_str(name);
    pushed.push_str(label);
    pushed.push_str(description);
    pushed
}


#[test]
fn sort_two_test() {
    let text1 = String::from("Text1");
    let text2 = String::from("text2");
    let expected = (text1.clone(), text2.clone());
    let actual = self::sort_two(&text1, &text2);
    assert_eq!(expected, actual);
}


