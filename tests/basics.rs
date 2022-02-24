use lsystem::{LSystem, Rules};

#[test]
fn test_basics() {
    let empty: Rules<i32> = Rules::default();
    assert!(empty.is_empty());
    assert_eq!(0, empty.len());

    let abc = Rules::default()
        .with_rule(&['a'], &['b'])
        .with_rule(&['b'], &['c'])
        .with_rule(&['c'], &['a']);
    assert_eq!(3, abc.len());

    let mut abc_sys = LSystem::new(&abc, vec!['a', 'b', 'c']);
    assert_eq!(
        "cab".to_owned(),
        abc_sys.nth(368).unwrap().iter().collect::<String>()
    );
}
