/// Defines single row in a buffer
pub struct Row {
    /// Row index
    idx: usize,

    /// Row text
    text: String,
}

/// Row implementation
impl Row {
    /// Creates new row
    pub fn new(idx: usize, text: String) -> Self {
        Self { idx, text }
    }

    /// Drops first `n` chars and returns new row
    pub fn drop_first(&self, n: usize) -> Self {
        Self {
            idx: self.idx,
            text: self.text[n.saturating_sub(1)..self.text.len()].to_string(),
        }
    }

    /// Drops last `n` chars and returns new row
    pub fn drop_last(&self, n: usize) -> Self {
        Self {
            idx: self.idx,
            text: self.text[0..(self.text.len() - n)].to_string(),
        }
    }

    /// Returns new row with specific range
    pub fn range(&self, from: usize, to: usize) -> Self {
        Self {
            idx: self.idx,
            text: self.text[from..to].to_string(),
        }
    }

    /// Returns len of row
    pub fn len(&self) -> usize {
        self.text.len()
    }

    /// Returns row index
    pub fn idx(&self) -> usize {
        self.idx
    }
}
