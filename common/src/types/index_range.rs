#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct IndexRange {
    pub start: usize,
    pub end: usize,
}

impl IndexRange {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn len(&self) -> usize {
        self.end - self.start
    }

    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }

    pub fn contains(&self, index: usize) -> bool {
        index >= self.start && index < self.end
    }

    pub fn contains_range(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    pub fn intersection(&self, other: &Self) -> Option<Self> {
        let start = self.start.max(other.start);
        let end = self.end.min(other.end);
        if start < end {
            Some(Self::new(start, end))
        } else {
            None
        }
    }

    pub fn union(&self, other: &Self) -> Self {
        Self::new(self.start.min(other.start), self.end.max(other.end))
    }

    pub fn extend(&mut self, other: &Self) {
        self.start = self.start.min(other.start);
        self.end = self.end.max(other.end);
    }

    pub fn shrink(&mut self, other: &Self) {
        self.start = self.start.max(other.start);
        self.end = self.end.min(other.end);
    }

    pub fn shift(&mut self, offset: isize) {
        self.start = (self.start as isize + offset).max(0) as usize;
        self.end = (self.end as isize + offset).max(0) as usize;
    }

    pub fn shift_start(&mut self, offset: isize) {
        self.start = (self.start as isize + offset).max(0) as usize;
    }

    pub fn shift_end(&mut self, offset: isize) {
        self.end = (self.end as isize + offset).max(0) as usize;
    }
}