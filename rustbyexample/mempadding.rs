struct Padded {
    a: u8,  // 1 byte
    b: u32, // 4 bytes
    c: u16, // 2 bytes
}

struct Optimized {
    a: u32, // 4 bytes
    b: u16, // 2 bytes
    c: u8,  // 1 byte
}

fn main() {
    println!("Size of Padded: {} bytes", std::mem::size_of::<Padded>());
    println!("Size of Optimized: {} bytes", std::mem::size_of::<Optimized>());
}
