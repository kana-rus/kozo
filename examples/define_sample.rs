use kozo::define;

define!(struct NestedStruct {
    a: Vec<u8>,
    b: struct B {
        c: struct C {
            d: u8,
            e: u8,
        },
        f: enum F {
            X,
            Y,
            Other {
                name: String,
                id: usize
            },
        },
    },
});

fn main() {
    let sample = NestedStruct {
        a: vec![1, 1, 0, 1, 0, 1, 1],
        b: B {
            c: C {
                d: 0,
                e: 1,
            },
            f: F::X,
        },
    };

    println!("{}", sample.b.c.d);  // 0
}