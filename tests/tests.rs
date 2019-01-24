#[test]
fn macro_name() {
    use with_macro::with as with_renamed;

    let v = with_renamed! {
        mut Vec::new() =>
            .push(42)
            .push(13)
    };

    assert_eq!(v, [42, 13]);
}
