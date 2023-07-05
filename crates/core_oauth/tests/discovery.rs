#[cfg(test)]
mod tests {
    // validate if tests are executed correctly in crates/workspaces
    #[test]
    fn check_tests_work_in_crates() {
        let a = 2;
        let b = 1 + 1;

        assert_eq!(a, b, "we are testing addition with {} and {}", a, b);
    }
}
