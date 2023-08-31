// options2.rs
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a hint.

#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        if let Some(o) = optional_target {
            assert_eq!(target, o);
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..(range + 1) {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        while let Some(Some(o)) = optional_integers.pop() {
            assert_eq!(o, cursor);
            cursor -= 1;
        }

        assert_eq!(cursor, 0);
    }
}
