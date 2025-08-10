use assert_cmd::Command;
use indoc::indoc;

#[test]
fn test_cli() {
    let input = indoc! {"
        5 3
        2 2 3 3 3
    "};
    let expected = indoc! {"
        2
        2
        3
    "};

    Command::cargo_bin("food_lines").unwrap()
        .write_stdin(input)
        .assert()
        .stdout(expected);
}
