#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Qux{
    Abc,
    Def,
    Ghi,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Quxx{
    Jkl,
    Mno,
    Pqr,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Quxxx{
    Stu,
    Vwx,
    Yz,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum MetaSyntaxCore {
    Foo(Qux),
    Bar(Quxx),
    Baz(Quxx),
    Zed(Quxxx),
}

fn main() {
    let msc = 1;

    if MetaSyntaxCore::Foo == MetaSyntaxCore::Bar {
        println!("hey hey");
    } else {
        println!("no no");
    }
}

