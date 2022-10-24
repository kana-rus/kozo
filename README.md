kozo provides some syntax sugars to use Rust's struct easily.\
Current kozo provides following 2 proc macros:

- `define!`
- `retrieve!`

<br/>
<br/>

# define!
`define!` macro enables to define nested structs in a way easy to see.

```rs
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
```
( examples/define_sample.rs )

<br/>

Then, `define!` is **just a syntax sugar** of defining each named structs separately like

```rs
struct DeepNestedStruct {
    a: Vec<u8>,
    b: B,
}
struct B {
    c: C,
    f: F,
}
struct C {
    d: u8,
    e: u8,
}
enum F {
    X,
    Y,
    Other {
        name: String,
        id: usize,
    },
}
```
So please pay attension to that **all structs declared in define!(); are visible** in its scope.

<br/>
<br/>

# retrieve!
`retrieve!` enables to simply get more than 1 value from a struct:

```rs
use kozo::{define, retrieve};

define!(struct Sample {
    a: u8,
    b: struct B {
        c: String,
        d: Vec<u8>,
    },
});

fn main() {
    let s = Sample {
        a: 0,
        b: B {
            c: "I have an apple?".into(),
            d: vec![1, 1, 0, 1, 0, 1, 1],
        },
    };
    retrieve!(a, b from s);

    println!("{a}");  // 0,
    println!("{}", b.c);
}
```
( examples/retrieve_sample.rs )

<br/>
<br/>

### NOTICEs
In next version (v0.2),

- `define!` will be able to accept `derive`s.

```rs
#[derive(Clone)]
define!(struct S {
    // ...
})
```

- `retrieve!` will be able to get nested values directly like

```rs
let s = Sample {
    a: 0,
    b: B {
        c: "You have an apple!".into(),
        d: vec![1, 1, 0, 1, 0, 1, 1,],
    },
};
retrieve!(a, c(b), from s);
```

<br/>

In future, `retrieve!` will support named retrieving:

```rs
let s = Sample {
    a: 0,
    b: B {
        c: "Is this an apple?".into(),
        d: vec![1, 1, 0, 1, 0, 1, 1,],
    },
};
retrieve!(var1 @ a, crazy_apple_man @ c(b), from s);

println!("{var1}");  // 0
```

<br/>

for Japanese speakers...\
"kozo" は「小僧」の発音で読んでください。struct「構造」と掛けたネーミングで、構造体に関する小さく便利な機能を提供するクレートという意味を込めています。