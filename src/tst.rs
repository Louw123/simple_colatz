#[cfg(test)]
mod test {
    use super::single_step_colatz;
    #[test]
    fn colup() {
        assert_eq!(single_step_colatz(3), 10);
    }
}
