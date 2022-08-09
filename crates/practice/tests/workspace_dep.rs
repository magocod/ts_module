use add_one::add_one;

#[test]
fn test_add_one() {
    let result = add_one(1);
    assert_eq!(result, 2);
}
