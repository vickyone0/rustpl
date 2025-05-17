pub fn is_palintrome(data: &String) -> bool {
    let s:Vec<char> = data.chars().filter(|c| c.is_alphanumeric()).collect();
    s.iter().eq(s.iter().rev())
}