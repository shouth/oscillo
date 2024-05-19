mod generated;

pub fn is_id_char(c: char) -> bool {
    generated::ID_CHAR_SET.contains_char(c)
}

pub fn is_id_start_char(c: char) -> bool {
    generated::ID_START_CHAR_SET.contains_char(c)
}
