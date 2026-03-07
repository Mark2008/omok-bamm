use crate::core::board::Move;

#[derive(Copy, Clone, Debug)]
pub struct TTEntry {
    pub hash: u64,
    pub value: f32,
    pub depth: u32,
}

#[derive(Debug)]
pub struct TT {
    table: Vec<Option<TTEntry>>,
    divisor: usize
}

impl TT {
    /// the size should be 2^n
    pub fn new(size: usize) -> Self {
        Self {
            table: vec![None; size],
            divisor: size - 1,
        }
    }

    pub fn get(&self, hash: u64) -> Option<TTEntry> {
        let idx = hash as usize & self.divisor;
        match self.table[idx] {
            Some(e) if e.hash == hash => Some(e),
            _ => None
        }
    }

    pub fn put(&mut self, e: TTEntry) {
        let idx = e.hash as usize & self.divisor;
        match self.table[idx] {
            Some(old) if old.hash == e.hash && old.depth > e.depth => {},
            _ => self.table[idx] = Some(e),
        }
    }
}