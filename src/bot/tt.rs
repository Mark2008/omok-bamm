use crate::core::board::Move;

#[derive(Copy, Clone)]
pub struct TTEntry {
    pub hash: u64,
    pub value: f32,
    pub depth: u32, 
    pub flag: Flag,
    pub best_move: Move,
}

#[derive(Copy, Clone)]
pub enum Flag {
    Lower, Exact, Upper
}

pub struct TT {
    table: Vec<Option<TTEntry>>,
}

impl TT {
    /// the size should be 2^n-1
    pub fn new(size: usize) -> Self {
        Self {
            table: vec![None; size],
        }
    }

    pub fn get(&self, hash: u64) -> Option<TTEntry> {
        let idx = hash as usize & self.table.len();
        match self.table[idx] {
            Some(e) if e.hash == hash => Some(e),
            _ => None
        }
    }

    pub fn put(&mut self, e: TTEntry) {
        let idx = e.hash as usize & self.table.len();
        match self.table[idx] {
            Some(old) if old.hash == e.hash && old.depth > e.depth => {},
            _ => self.table[idx] = Some(e),
        }
    }
}