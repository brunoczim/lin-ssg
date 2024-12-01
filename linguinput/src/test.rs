use crate::encode;

#[test]
fn no_code() {
    let input = "hello";
    let expected = "hello";
    let actual = encode(input).unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn simple_hello() {
    let input = "h{e}l.o{U} {4}{2}{3}{2}";
    let expected = "hɛl.oʊ ˦˨˧˨";
    let actual = encode(input).unwrap();
    assert_eq!(actual, expected);
}
