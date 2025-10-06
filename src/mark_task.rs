fn mark_in_progress(id: u32) {}
fn mark_done(id: u32) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(1 + 2, 3);
    }
}
