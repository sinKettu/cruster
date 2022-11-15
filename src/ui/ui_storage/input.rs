use std::cmp::min;
use super::UI;
use super::filter::DEFAULT_MESSAGE_LENGTH;

///
/// Inputs
/// The part of UI's implementation responsible for
/// converting input from user's keyboard to UI objects' changes
///

impl<'ui_lt> UI<'ui_lt> {
    ///
    /// Handle entering one (printable) char from keyboard
    ///
    pub(crate) fn handle_char_input(&mut self, chr: char) {
        let mut left = self.input_buffer[0..self.input_cursor].to_string();
        left.push(chr);
        let right = &self.input_buffer[self.input_cursor..];
        left.push_str(right);
        self.input_buffer = left;

        // TODO: length limitation
        self.input_cursor += 1;
    }

    ///
    /// Remove previous char in input buffer (if any)
    ///
    pub(crate) fn handle_backspace_input(&mut self) {
        if self.input_cursor > 0 {
            self.input_cursor = self.input_cursor.saturating_sub(1);
            self.input_buffer.remove(self.input_cursor);
        }
    }

    ///
    /// Remove char on which the cursor is
    ///
    pub(crate) fn handle_delete_input(&mut self) {
        if self.input_cursor < self.input_buffer.len() {
            self.input_buffer.remove(self.input_cursor);
            self.input_cursor -= 1;
        }
    }

    ///
    /// Cursor one-char step backward
    ///
    pub(crate) fn handle_move_cursor_left(&mut self) {
        self.input_cursor = self.input_cursor.saturating_sub(1);
    }

    ///
    /// Cursor one-char step forward
    ///
    pub(crate) fn handle_move_cursor_right(&mut self) {
        self.input_cursor = min(self.input_cursor + 1, self.input_buffer.len());
    }

    ///
    /// Move cursor to beginning of input
    ///
    pub(crate) fn handle_move_cursor_home(&mut self) {
        self.input_cursor = 0_usize;
    }

    ///
    /// Move cursor to ending of input
    ///
    pub(crate) fn handle_move_cursor_end(&mut self) {
        self.input_cursor = self.input_buffer.len();
    }

    ///
    /// Get cursor position inside rect for rendering
    ///
    pub(crate) fn get_cursor_relative_position(&mut self) -> (usize, usize) {
        let x = DEFAULT_MESSAGE_LENGTH + 1 + self.input_cursor;
        let y = 2_usize;

        return (x, y);
    }

    ///
    /// Get index of area where text is edited now
    ///
    pub(crate) fn get_currently_edited_area(&mut self) -> Option<usize> {
        self.editable_area
    }

    ///
    /// Remove all entered data
    /// 
    pub(crate) fn clear_input(&mut self) {
        self.input_cursor = 0_usize;
        self.input_buffer.clear();
    }
}
