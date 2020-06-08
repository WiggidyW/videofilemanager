use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct B(Vec<String>);

#[derive(Debug, Deserialize)]
struct A {
    A: Vec<String>,
}

#[derive(Serialize)]
struct Field {
    a: &'static str,
    b: &'static str,
    c: &'static str,
}

fn main() {
    let field = Field {
        a: "Foo",
        b: "Bar",
        c: "FooBar",
    };
    let field = serde_json::to_string(&field);
    println!("{:?}", field);
    let vec = B(vec!["Foo".to_string(), "Bar".to_string()]);
    let vec = serde_json::to_string(&vec);
    println!("{:?}", vec);
}