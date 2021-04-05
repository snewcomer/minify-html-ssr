struct Writable<'a> {
    html: &'a [u8],

    cursor: usize,
    write_next: usize,
}

// impl Writable {
//     pub fn peek() {
//         self.html[self.cursor]
//     }
// }
