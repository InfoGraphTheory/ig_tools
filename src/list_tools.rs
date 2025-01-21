
pub fn sort_insensitive(text: String) -> String {
    let lines = text.lines();
    let mut vec: Vec<&str> = lines.collect();


    vec.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
    vec.iter()
        .flat_map(|x| x.chars().chain("\n".chars()))
        .collect()
}

pub fn append_ln(line: &str, lines: &str) -> String {

    format!("{}\n{}", line, lines)
}

//TODO: Call this from the facade: getData, createLine, append_ln_n_sort, setData
pub fn append_ln_n_sort(line: &str, lines: &str) -> String {
    let text = self::append_ln(line, lines);
    self::sort_insensitive(text)
}

pub fn append(text: &str, line: &str, buf_size: usize) -> String {
    let lines = text.lines();
    
    //buf size must be > 0 and makes no sense if < 2
    let lines = lines.skip(1); 
    let lines: Vec<String> = lines
        .map(|x| x.to_owned())
        .collect();

    let mut lines: Vec<&String> = lines.iter().collect();
    let binding = line.to_string();
    lines.push(&binding);

    let lines: String = lines
        .iter()
        .flat_map(|x| x.chars().chain("\n".chars()))
        .collect();

    format!("{}\n{}", line, lines)
}

pub fn prepend(text: &str, line: &str, buf_size: usize) -> String {
    let lines = text.lines();
    
    //buf size must be > 0 and makes no sense if < 2
    let lines = lines.take(buf_size - 1); 

    let lines: String = lines
        .flat_map(|x| x.chars().chain("\n".chars()))
        .collect();
    
    format!("{}\n{}", line, lines)
}

pub fn select_line_keep_second(text: &str, line: &str) -> String {
    let mut lines = text.lines();
    let line1 = lines.next();
    if line1.is_none() {
        return line.to_string();
    }
    let line2 = lines.next();
    if line2.is_some() {
        format!("{}\n{}", line2.unwrap(), line)
    } else {
        format!("{}\n{}", line1.unwrap(), line)
    }
}
 
pub fn select_line_keep_first(text: &str, line: &str) -> String {
    let mut lines = text.lines();
    let line1 = lines.next();
    if line1.is_none() {
        line.to_string()
    } else {
        format!("{}\n{}", line, line1.unwrap())
    }
}

pub fn select_line_no(text: & str, line_no: usize) -> Option<String> { //TODO: ask about this 
    let result = text.lines().nth(line_no);
    result.map(|str|String::from(str)) 
}


//TODO: make tests
//#[test]
//fn append_test(){
//    let result = append("hello", "world",20);
//    assert_eq!(result, "hello\nworld".to_string());
//}

//#[test]
//fn prepend_test(){
//    let result = prepend("world", "hello",20);
//    assert_eq!(result, "hello\nworld".to_string());
//}


