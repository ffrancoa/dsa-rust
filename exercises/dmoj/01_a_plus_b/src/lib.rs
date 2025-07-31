//  A Plus B
//
// Tudor is sitting in math class, on his laptop. Clearly, he is not paying attention
// in this situation. However, he gets called on by his math teacher to do some
// problems. Since his math teacher did not expect much from Tudor, he only needs to do
// some simple addition problems. However, simple for you and I may not be simple for
// Tudor, so please help him!
// 
// Input Specification
// -------------------
// The first line will contain an integer `N` ( 1 ≤ `N` ≤ 100 000 ) , the number of
// addition problems Tudor needs to do. 
//
// Output Specification
// --------------------
// Output `N` lines of one integer each, the solutions to the addition problems in
// order.

pub fn a_plus_b(input: &str) -> String {
    todo!("pending solution!")
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn example() {
        let input = indoc! {"
            2
            1 1
            -1 0
        "};
        let expected = indoc! {"
            2
            -1
        "};

        assert_eq!(a_plus_b(input), expected);
    }
}
