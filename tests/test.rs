include!("dto/os_detail.rs");

#[test]
fn simple() {
    let t = trybuild::TestCases::new();
    t.pass("tests/dto/user_status.rs");
}

#[test]
fn complex() {
    let t = trybuild::TestCases::new();
    t.pass("tests/dto/os_detail.rs");
}
