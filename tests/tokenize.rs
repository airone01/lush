mod tokenize;

#[cfg(test)]
mod unittest_tokenize_command {
    use super::*;

    #[test]
    #[ignore]
    fn empty_command() {
        assert_eq!("", tokenize_command("".to_string()).keyword)
    }

    #[test]
    fn test_keyword() {
        assert_eq!("test", tokenize_command("test".to_string()).keyword)
    }

    #[test]
    fn no_arg() {
        assert_eq!(0, tokenize_command("test".to_string()).args.len())
    }

    #[test]
    fn one_arg() {
        assert_eq!(1, tokenize_command("test one".to_string()).args.len())
    }

    #[test]
    fn multi_args() {
        assert_eq!(
            3,
            tokenize_command("test one two three".to_string())
                .args
                .len()
        )
    }

    #[test]
    #[ignore]
    fn quotes() {
        assert_eq!(
            2,
            tokenize_command("test \"one two\" three".to_string())
                .args
                .len()
        )
    }
}
