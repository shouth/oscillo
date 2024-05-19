mod generated;

use self::generated::{ID_CHAR_SET, ID_START_CHAR_SET};

pub fn is_id_char(c: char) -> bool {
    ID_CHAR_SET.contains_char(c)
}

pub fn is_id_start_char(c: char) -> bool {
    ID_START_CHAR_SET.contains_char(c)
}
