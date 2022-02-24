use lsystem::{LSystem, Rules};

#[test]
fn test_binary_tree() {
    let rules = Rules::default()
        .with_rule(&['1'], &['1', '1'])
        .with_rule(&['0'], &['1', '[', '0', ']', '0']);
    let mut system = LSystem::new(&rules, vec!['0']);

    let third = system.nth(3).unwrap();
    assert_eq!(
        "1111[11[1[0]0]1[0]0]11[1[0]0]1[0]0".to_owned(),
        third.iter().collect::<String>()
    );
}
