#[cfg(test)]
mod tests {
    use super::super::format_placement;

    #[test]
    fn format_placement_uses_the_engine_protocol() {
        assert_eq!(format_placement(7, 2), "7 2\n");
    }
}
