// use kozo::{define, retrieve};
// 
// define!(struct Sample {
//     a: u8,
//     b: struct B {
//         c: String,
//         d: Vec<u8>,
//     },
// });
// 
// fn main() {
//     let s = Sample {
//         a: 0,
//         b: B {
//             c: "I have an apple?".into(),
//             d: vec![1, 1, 0, 1, 0, 1, 1],
//         },
//     };
//     retrieve!(a, b from s);
// 
//     println!("{a}");  // 0,
//     println!("{}", b.c);
// }
fn main() {}