#![feature(test)]
extern crate test;
pub mod tapemachine;

#[cfg(test)]
mod tests {
    use crate::tapemachine::TapeResult;
    use crate::tapemachine::{CharTapeMachine, TapeError};
    use test::Bencher;

    const TEST_STRING: &str = "TEST STRING 1234 \\l \\n";

    #[test]
    fn it_returns_the_next_char() {
        let mut ctm = CharTapeMachine::new(TEST_STRING.chars().collect());
        let test_chars: Vec<char> = TEST_STRING.chars().collect();

        let mut next = ctm.next_char().unwrap();
        assert_eq!(next, *test_chars.get(1).unwrap());

        next = ctm.next_char().unwrap();
        assert_eq!(next, *test_chars.get(2).unwrap());

        let _ = ctm.next_char().unwrap();
        let _ = ctm.next_char().unwrap();
        let _ = ctm.next_char().unwrap();
        next = ctm.next_char().unwrap();
        assert_eq!(next, *test_chars.get(6).unwrap());
    }

    #[test]
    fn it_rewinds() {
        let mut ctm = CharTapeMachine::new(TEST_STRING.chars().collect());
        let test_chars: Vec<char> = TEST_STRING.chars().collect();

        ctm.next_char().unwrap();
        ctm.next_char().unwrap();
        assert_eq!(ctm.next_char(), Some(*test_chars.get(3).unwrap()));

        ctm.rewind(1);
        assert_eq!(ctm.next_char(), Some(*test_chars.get(2).unwrap()));
    }

    #[test]
    fn it_seeks() {
        let mut ctm = CharTapeMachine::new(TEST_STRING.chars().collect());
        let test_chars: Vec<char> = TEST_STRING.chars().collect();

        assert_eq!(ctm.next_char(), Some(*test_chars.get(1).unwrap()));
        ctm.seek_one().unwrap();
        assert_eq!(ctm.next_char(), Some(*test_chars.get(3).unwrap()));
        ctm.seek_one().unwrap();
        ctm.seek_whitespace();
        assert_eq!(ctm.next_char(), Some(*test_chars.get(6).unwrap()));
    }

    #[test]
    fn it_asserts_chars() -> TapeResult<()> {
        let mut ctm = CharTapeMachine::new(TEST_STRING.chars().collect());
        ctm.assert_any(&['A', 'B', 'T'], None)?;
        ctm.seek_one().unwrap();
        ctm.assert_char(&'E', None)?;
        ctm.seek_one().unwrap();
        ctm.assert_str_sequence("ST ", None)?;
        ctm.seek_one().unwrap();
        ctm.assert_any_sequence(&[&['C'], &['A'], &['A', 'B'], &['S', 'T', 'R']], None)?;

        if let Ok(_) =
            ctm.assert_any_sequence(&[&['C'], &['A'], &['A', 'B'], &['S', 'T', 'R']], None)
        {
            Err(TapeError::new(0))
        } else {
            Ok(())
        }
    }

    #[test]
    fn it_checks_eof() -> TapeResult<()> {
        let mut ctm = CharTapeMachine::new(TEST_STRING.chars().collect());
        let _ = ctm.get_string_until_any(&['n'], &[]);
        assert!(ctm.check_eof());

        Ok(())
    }

    #[bench]
    fn bench_assert_seek(b: &mut Bencher) {
        let mut ctm = CharTapeMachine::new(TEST_STRING.chars().collect());
        b.iter(|| {
            ctm.check_char(&'T');
            ctm.seek_one().unwrap();
            ctm.check_char(&'E');
            ctm.seek_one().unwrap();
            ctm.check_char(&'F');
            ctm.seek_one().unwrap();
            ctm.check_any(&['A', 'B', 'C', 'D', 'E', '2']);
            ctm.seek_one().unwrap();
            ctm.seek_whitespace();
            ctm.check_sequence(&['S', 'T', 'R', 'I', 'N', 'T']);
            ctm.check_sequence(&['S', 'T', 'R', 'I', 'N', 'G']);
            ctm.check_eof();
            ctm.rewind(0);
        })
    }
}
