use bitcoin_hashes::sha256;
use bitcoin_hashes::Hash;

///
/// Takes two strings as parameters and returns them in a sorted string tuple.
///
pub fn sort_two(text1: &str, text2: &str) -> (String, String) {
            
    let mut t1 = text1;
    let mut t2 = text2;

    if text1 > text2 {
        t2 = text1;
        t1 = text2;
    }
    
    (t1.to_string(), t2.to_string())
}

///
/// Returns a hash of a Str in String format.
///
pub fn hash_text(it: &str) -> String {

    sha256::Hash::hash(it.as_bytes()).to_string()
}

///
/// Takes two strings, sort them and return a hash of the concatenation of the sorted pair.
///
pub fn sort_cat_n_hash(text1: &str, text2: &str) -> String {
    
    let sorted = sort_two(text1, text2);
    cat_n_hash(sorted.0, sorted.1)
}

///
/// Takes two strings as arguments and return a hash of the concatenation of the pair.
///
pub fn cat_n_hash(left: String, right: String) -> String {

    let mut concat: String = left.clone();
    concat.push_str(&right);
    hash_text(&concat)
}

///
/// Takes two strings, sort them and return a triple as a tuple consisting of a hash of the concatenation of the sorted pair and the two sorted values.
///
pub fn concat_n_hash(text1: &str, text2: &str) -> (String, String, String) {

    let sorted = sort_two(text1.trim_matches('"'), text2.trim_matches('"'));
    let id = cat_n_hash(sorted.0.clone(), sorted.1.clone());
    (id, sorted.0, sorted.1)
}

///
/// This method outght to be changed in the future as it concatenates fields without any separator
/// character, making the risk of coalition higher. Also the name by be a bit confusing. It may be
/// that the method is meant for hashed values, but the method itself does not have to do with
/// hashing processes of any kind.
///
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


