use crate::aes;
use crate::aes::matrix_mul;
use crate::rsa;
use rug::Integer;

pub fn test() {
    let x = aes::AES::new();
    // x.key_expansion();
    // println!(
    //     "{:x?}",
    //     x.encrypt(vec![
    //         0x6b, 0xc1, 0xbe, 0xe2, 0x2e, 0x40, 0x9f, 0x96, 0xe9, 0x3d, 0x7e, 0x11, 0x73, 0x93,
    //         0x17, 0x2a
    //     ])
    // )
    print!(
        "{:x?}",
        x.decrypt(vec![
            0xf3, 0xee, 0xd1, 0xbd, 0xb5, 0xd2, 0xa0, 0x3c, 0x06, 0x4b, 0x5a, 0x7e, 0x3d, 0xb1,
            0x81, 0xf8
        ])
    )
}
