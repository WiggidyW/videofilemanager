use serde::Serialize;

#[derive(Serialize)]
struct TestStruct {
    test_enum: TestEnum,
    other: String,
}

#[derive(Serialize)]
enum TestEnum {
    Foo(Option<String>),
    Bar(Option<String>),
}

fn main() {
    let t1 = TestStruct {
        test_enum: TestEnum::Bar(Some("Bar".to_string())),
        other: "Other".to_string(),
    };
    let t2 = TestStruct {
        test_enum: TestEnum::Foo(Some("Foo".to_string())),
        other: "Other".to_string(),
    };
    let t3 = TestStruct {
        test_enum: TestEnum::Foo(None),
        other: "Other".to_string(),
    };
    println!("{}", serde_json::to_string(&t1).unwrap());
    println!("{}", serde_json::to_string(&t2).unwrap());
    println!("{}", serde_json::to_string(&t3).unwrap());
}