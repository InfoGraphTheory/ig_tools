


///
/// Sort lines in text case-insensitive
///
pub fn sort_insensitive(text: String) -> String {

    let lines = text.lines();
    let mut vec: Vec<&str> = lines.collect();

    vec.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
    vec.iter()
        .flat_map(|x| x.chars().chain("\n".chars()))
        .collect()
}

///
/// Append line to lines
///
pub fn append_ln(line: &str, lines: &str) -> String {

    format!("{}\n{}", line, lines)
}

///
/// Sort lines after appendning line.
///
pub fn append_ln_n_sort(line: &str, lines: &str) -> String {
    let text = self::append_ln(line, lines);
    self::sort_insensitive(text)
}

///
/// Prepends line to lines in a buffer/rolling fashion, meaning that the function also ensures that the text will only be buf_size long.
/// If the text gets longer than buf_size, excess lines at the end of the text is removed to make
/// room for the new line.
///
pub fn prepend(text: &str, line: &str, buf_size: usize) -> String {
    let lines = text.lines();
    
    //buf size must be > 0 and makes no sense if < 2
    let lines = lines.take(buf_size - 1); 

    let lines: String = lines
        .flat_map(|x| x.chars().chain("\n".chars()))
        .collect();
    
    format!("{}\n{}", line, lines)
}

///
/// This function handles an insert into a buffer of size two. It preserves the last item so
/// appending an item to the buffer.  
/// The function returns a text consisting of the second line of the argument text, a new line and then the second argument, line. 
/// If text does not have a second line, the first line plus a new line plus the line argument is returned, if a first line exists in text.
/// If no lines exists in the text, the second argument, line, is returned as a default value.
///
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
 
///
/// This function handles an insert into a buffer of size two. It preserves the first item so
/// pre-pending an item to the buffer.  
/// The function returns a text consisting of the second argument, line, a new line and then the first line of the argument text. 
/// If no lines exists in the text, the second argument, line, is returned as a default value.
///
pub fn select_line_keep_first(text: &str, line: &str) -> String {
    let mut lines = text.lines();
    let line1 = lines.next();
    if line1.is_none() {
        line.to_string()
    } else {
        format!("{}\n{}", line, line1.unwrap())
    }
}

///
/// Returns an Option with the line number line_no from text if such line exists.
///
///
pub fn select_line_no(text: & str, line_no: usize) -> Option<String> { 
    let result = text.lines().nth(line_no);
    result.map(|str|String::from(str)) 
}

