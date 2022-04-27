use rug::Integer;

fn aes_key_expansion(key: Integer) -> Integer {
    Integer::new()
}

fn aes_key_gen(length: Option<i32>) -> Integer {
    let length = if let Some(length) = length {
        length
    } else {
        128
    };
    Integer::new()
}
