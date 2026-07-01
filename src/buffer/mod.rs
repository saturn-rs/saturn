/// Modules
mod row;
#[cfg(test)]
mod tests;

/// Imports
use crate::buffer::row::Row;

/// Defines a text buffer we manipulate
/// during edit mode
#[derive(Debug)]
pub struct Buffer {
    /// Buffer text
    text: Vec<Row>,

    /// Buffer file name
    file_name: String,
}

/// Buffer implementation
impl Buffer {
    /// Creates new buffer with text spllited by `\n` and passed file name
    pub fn new(text: String, file_name: String) -> Self {
        Self {
            text: text
                .split("\n")
                .enumerate()
                .map(|(idx, text)| Row::new(idx, text.to_string()))
                .collect(),
            file_name,
        }
    }

    /// Returns file name
    pub fn file_name(&self) -> &str {
        &self.file_name
    }

    /// Returns rows array by offset, width, height
    pub fn rows(&self, offset: (usize, usize), width: usize, height: usize) -> Vec<Row> {
        // Validating rows offset
        if offset.1 < self.text.len() {
            // Getting rows in range from `rows offset`
            // to min between `rows offset + height` and `rows amount`
            let start = offset.1;
            let end = (start + height).min(self.text.len());
            let rows = &self.text[start..end];

            // Getting rows text in range `columns offset`
            // to min between `columns offset + width` and `row len`
            rows.iter()
                .map(|row| {
                    // Validating columns offset
                    if offset.0 < row.len() {
                        // Preparing range
                        let start = offset.0;
                        let end = (start + width).min(row.len());

                        // Getting new row with range
                        row.range(start, end)
                    } else {
                        panic!("columns offset ({}) >= row len ({})", offset.0, row.len())
                    }
                })
                .collect()
        } else {
            panic!(
                "rows offset ({}) >= rows amount ({})",
                offset.1,
                self.text.len()
            )
        }
    }

    /// Returns row by row index
    pub fn row(&self, idx: usize) -> &Row {
        // Validating row index
        if idx < self.text.len() {
            &self.text[idx]
        } else {
            panic!("row index >= rows amount")
        }
    }
}
