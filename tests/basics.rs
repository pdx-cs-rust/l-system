use lsystem::{LSystem, Rules};

#[test]
fn test_basics() {
    let empty: Rules<i32> = Rules::default();
    assert!(empty.is_empty());
    assert_eq!(0, empty.len());

    let abc = Rules::default()
        .with_rule(&[1], &[2])
        .with_rule(&[2], &[3])
        .with_rule(&[3], &[1]);
    assert_eq!(3, abc.len());

    let mut abc_sys = LSystem::new(&abc, vec![1, 2, 3]);
    assert_eq!(
        vec![3, 1, 2],
        abc_sys.nth(368).unwrap()
    );
    assert_eq!(
        vec![1, 2, 3],
        abc_sys.nth(369).unwrap()
    );
}
