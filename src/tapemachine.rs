use std::error::Error;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub struct TapeError {
    index: usize
}

impl TapeError {
    pub fn new(index: usize) -> Self {
        Self {
            index
        }
    }
}

impl Display for TapeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Tape Error at: {}", self.index)
    }
}

impl Error for TapeError {}

pub type TapeResult<T> = Result<T, TapeError>;

const ESCAPE: char = '\\';

pub struct CharTapeMachine {
    index: usize,
    text: Vec<char>,
    current_char: char,
    previous_char: char,
}

impl CharTapeMachine {
    pub fn new(text: Vec<char>) -> Self {
        let current_char = if text.len() > 0 {
            *text.first().unwrap()
        } else {
            ' '
        };
        Self {
            text,
            index: 0,
            previous_char: current_char,
            current_char,
        }
    }

    pub fn get_text(&self) -> Vec<char> {
        self.text.clone()
    }

    /// Returns the next char
    /// if there is any
    pub fn next_char(&mut self) -> Option<char> {
        if self.index < self.text.len() {
            self.index += 1;
            self.previous_char = self.current_char;
            self.current_char = *self.text.get(self.index)?;

            Some(self.current_char)
        } else {
            None
        }
    }

    /// Peeks the next available char
    #[inline]
    pub fn peek_char(&mut self) -> Option<char> {
        Some(*self.text.get(self.index + 1)?)
    }

    /// Rewinds to a given index
    #[inline]
    pub fn rewind(&mut self, index: usize) {
        self.index = index;
        self.current_char = *self.text.get(index).unwrap();
    }

    /// Rewinds to a given index and returns an error
    #[inline]
    pub fn rewind_with_error(&mut self, index: usize) -> TapeError {
        self.rewind(index);
        TapeError::new(index)
    }

    /// Seeks one character or returns an error
    /// if there is no next character
    #[inline]
    pub fn seek_one(&mut self) -> TapeResult<()> {
        if let Some(_) = self.next_char() {
            Ok(())
        } else {
            Err(TapeError::new(self.index))
        }
    }

    /// Seeks until it encounters a non whitespace character
    pub fn seek_whitespace(&mut self) {
        if self.current_char.is_whitespace() {
            while let Some(next) = self.next_char() {
                if !next.is_whitespace() || self.check_escaped() {
                    break
                }
            }
        }
    }

    /// Checks if the machine has reached the eof
    pub fn check_eof(&self) -> bool {
        self.index >= self.text.len()
    }

    /// checks if the current char is escaped
    #[inline]
    pub fn check_escaped(&self) -> bool {
        self.previous_char == ESCAPE
    }

    /// Returns true if the given character is equal to the current one
    /// and the current character is not escaped
    #[inline]
    pub fn check_char(&self, value: &char) -> bool {
        self.current_char == *value && !self.check_escaped()
    }

    /// Checks if one of the given chars matches the current one
    #[inline]
    pub fn check_any(&self, chars: &[char]) -> bool {
        !self.check_escaped() && chars.contains(&self.current_char)
    }

    /// checks if the next characters match a given sequence of characters
    pub fn check_sequence(&mut self, sequence: &[char]) -> bool {
        let start_index = self.index;

        if self.check_escaped() {
            self.rewind(start_index);

            false
        } else {
            for sq_character in sequence {
                if self.current_char != *sq_character {
                    self.rewind(start_index);
                    return false;
                }
                if self.next_char() == None {
                    self.rewind(start_index);
                    return false;
                }
            }
            if self.index > 0 {
                self.rewind(self.index - 1);
            }
            true
        }
    }

    /// checks if the next characters match any given sequence
    #[inline]
    pub fn check_any_sequence(&mut self, sequences: &[&[char]]) -> bool {
        for seq in sequences {
            if self.check_sequence(*seq) {
                return true;
            }
        }

        false
    }

    /// returns an error on the current position and optionally rewinds
    /// if a rewind index is given
    #[inline]
    pub fn assert_error(&mut self, rewind_index: Option<usize>) -> TapeError {
        if let Some(index) = rewind_index {
            self.rewind_with_error(index)
        } else {
            TapeError::new(self.index)
        }
    }

    /// returns an error if the given char doesn't match the current one and rewinds
    /// if a rewind index is given
    #[inline]
    pub fn assert_char(&mut self, value: &char, rewind_index: Option<usize>) -> TapeResult<()> {
        if self.check_char(value) {
            Ok(())
        } else {
            Err(self.assert_error(rewind_index))
        }
    }

    /// returns an error if the current char doesn't match any of the given group
    #[inline]
    pub fn assert_any(&mut self, chars: &[char], rewind_index: Option<usize>) -> TapeResult<()> {
        if self.check_any(chars) {
            Ok(())
        } else {
            Err(self.assert_error(rewind_index))
        }
    }

    /// returns an error if the next chars don't match a special sequence
    #[inline]
    pub fn assert_sequence(&mut self, sequence: &[char], rewind_index: Option<usize>) -> TapeResult<()> {
        if self.check_sequence(sequence) {
            Ok(())
        } else {
            Err(self.assert_error(rewind_index))
        }
    }

    /// returns an error if the next chars don't match any given sequence
    pub fn assert_any_sequence(&mut self, sequences: &[&[char]], rewind_index: Option<usize>) -> TapeResult<()> {
        if self.check_any_sequence(sequences) {
            Ok(())
        } else {
            Err(self.assert_error(rewind_index))
        }
    }

    /// returns the string until any given character is matched is matched.
    /// rewinds with error if it encounters a character form the error group
    pub fn get_string_until_any(
        &mut self,
        until: &[char],
        err_at: &[char],
    ) -> TapeResult<String> {
        let start_index = self.index;
        let mut result = String::new();

        if self.check_any(until) {
            return Ok(result);
        } else if self.check_any(err_at) {
            return Err(TapeError::new(self.index));
        }

        result.push(self.current_char);
        while let Some(ch) = self.next_char() {
            if self.check_any(until) || self.check_any(err_at) {
                break;
            }
            result.push(ch);
        }

        if self.check_any(err_at) {
            Err(self.rewind_with_error(start_index))
        } else {
            Ok(result)
        }
    }

    /// Returns the string until it encounters a given sequence or rewinds with error
    /// if it encounters an err sequence
    pub fn get_string_until_sequence(
        &mut self,
        until: &[&[char]],
        err_at: &[&[char]],
    ) -> Result<String, TapeError> {
        let start_index = self.index;
        let mut result = String::new();

        if self.check_any_sequence(until) {
            return Ok(result);
        } else if self.check_any_sequence(err_at) {
            return Err(TapeError::new(self.index));
        }

        result.push(self.current_char);
        while let Some(ch) = self.next_char() {
            if self.check_any_sequence(until) || self.check_any_sequence(err_at) {
                break;
            }
            result.push(ch);
        }

        if self.check_any_sequence(err_at) {
            Err(self.rewind_with_error(start_index))
        } else {
            Ok(result)
        }
    }
}