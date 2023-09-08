pub fn is_acceptable_scope(parent_scope: &str, target_scope: &str) -> bool {
    let parent_scope = parent_scope.split(' ').collect::<Vec<_>>();
    target_scope.split(' ').all(|e| parent_scope.contains(&e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_acceptable_scope_accepted() {
        let test_cases = [
            ("hoge fuga", "hoge fuga"),
            ("hoge fuga", "hoge fuga fuga"),
            ("", ""),
            ("hoge fuga", "hoge"),
        ];

        for (parent, target) in test_cases {
            assert!(is_acceptable_scope(parent, target));
        }
    }

    #[test]
    fn is_acceptable_scope_not_accepted() {
        let test_cases = [("hoge fuga", "hoge fuga piyo"), ("", "hoge")];

        for (parent, target) in test_cases {
            assert!(!is_acceptable_scope(parent, target));
        }
    }
}
