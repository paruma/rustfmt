//@ rustfmt-edition: 2021

fn single() {}
fn lines() {
}
fn blank() {

}
struct S {

}
enum E {

}
trait T {

}
impl S {

}
mod m {

}
fn main() {
    for _i in 0..3 {

    }
    while false {

    }
    loop {

    }
    if true {

    } else {

    }
    let _f = || {

    };
    let _a = async {

    };
    let _v = {

    };
    unsafe {

    }
    match 1 {
        _ => {

        }
    }
}

fn more() {
    if true {} else {}
    for _ in 0..2 {}
    let _single = || {};
    let _no_blank = {
    };
    let _comment = {
        // keep this comment
    };
    let _constant = const {

    };
    let _match = match 0 {

    };
    'label: {

    }
}
