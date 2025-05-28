use sve::{self, SVEHeader};

fn main() {
    let header = SVEHeader::default();
    let mut raw_header = [0; size_of::<SVEHeader>()];
    let config = bincode::config::standard().with_little_endian().with_fixed_int_encoding();
    let _result = bincode::encode_into_slice(header, &mut raw_header, config);
}
