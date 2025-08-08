//! FrameBuffer: a simple 2D character buffer for building frames (no ANSI).
//! - Storage: Vec<char>, row-major (index = y * width + x)
//! - OOB writes/reads are ignored (clipped); invariants guarded with debug_asserts.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FrameBuffer {
    width: usize,
    height: usize,
    cells: Vec<char>,
}

impl FrameBuffer {
    /// Create a new framebuffer filled with `fill`.
    pub fn new(width: usize, height: usize, fill: char) -> Self {
        let width = width.max(1);
        let height = height.max(1);
        let len = width.saturating_mul(height);
        let mut cells = Vec::with_capacity(len);
        cells.resize(len, fill);
        Self {
            width,
            height,
            cells,
        }
    }

    #[inline]
    pub fn width(&self) -> usize {
        self.width
    }

    #[inline]
    pub fn height(&self) -> usize {
        self.height
    }

    /// Clear the entire buffer to `fill`.
    pub fn clear(&mut self, fill: char) {
        self.cells.fill(fill);
    }

    /// Set a cell to `ch`; if out-of-bounds, ignore.
    pub fn set(&mut self, x: usize, y: usize, ch: char) {
        if let Some(i) = self.idx(x, y) {
            self.cells[i] = ch;
        }
    }

    /// Get a cell; returns None if out-of-bounds.
    pub fn get(&self, x: usize, y: usize) -> Option<char> {
        self.idx(x, y).map(|i| self.cells[i])
    }

    /// Convert to a newline-terminated string of lines.
    pub fn to_string_lines(&self) -> String {
        let w = self.width;
        let h = self.height;
        let mut out = String::with_capacity((w + 1) * h);
        for y in 0..h {
            let row_start = y * w;
            for x in 0..w {
                out.push(self.cells[row_start + x]);
            }
            out.push('\n');
        }
        out
    }

    #[inline]
    fn idx(&self, x: usize, y: usize) -> Option<usize> {
        if x < self.width && y < self.height {
            let i = y.checked_mul(self.width).and_then(|v| v.checked_add(x));
            debug_assert!(i.is_some(), "index overflow");
            i
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_and_clear() {
        let mut fb = FrameBuffer::new(3, 2, '.');
        assert_eq!(fb.width(), 3);
        assert_eq!(fb.height(), 2);
        assert_eq!(fb.get(0, 0), Some('.'));
        assert_eq!(fb.get(2, 1), Some('.'));
        fb.clear(' ');
        assert_eq!(fb.get(0, 0), Some(' '));
        assert_eq!(fb.get(2, 1), Some(' '));
    }

    #[test]
    fn set_and_get_in_bounds() {
        let mut fb = FrameBuffer::new(4, 3, ' ');
        fb.set(1, 1, 'X');
        assert_eq!(fb.get(1, 1), Some('X'));
        // neighbors unaffected
        assert_eq!(fb.get(0, 1), Some(' '));
        assert_eq!(fb.get(2, 1), Some(' '));
    }

    #[test]
    fn oob_is_ignored() {
        let mut fb = FrameBuffer::new(2, 2, '.');
        fb.set(2, 0, 'X'); // x out of bounds
        fb.set(0, 2, 'X'); // y out of bounds
        assert_eq!(fb.get(2, 0), None);
        assert_eq!(fb.get(0, 2), None);
        // unchanged interior
        assert_eq!(fb.get(0, 0), Some('.'));
        assert_eq!(fb.get(1, 1), Some('.'));
    }

    #[test]
    fn to_string_lines_shape() {
        let mut fb = FrameBuffer::new(3, 2, ' ');
        fb.set(0, 0, 'A');
        fb.set(2, 1, 'B');
        let s = fb.to_string_lines();
        let lines: Vec<&str> = s.lines().collect();
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0].chars().count(), 3);
        assert_eq!(lines[1].chars().count(), 3);
        assert_eq!(lines[0].chars().collect::<Vec<_>>(), vec!['A', ' ', ' ']);
        assert_eq!(lines[1].chars().collect::<Vec<_>>(), vec![' ', ' ', 'B']);
    }
}
