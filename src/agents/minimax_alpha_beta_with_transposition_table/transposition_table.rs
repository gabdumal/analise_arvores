use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub enum NodeType {
    Exact,
    LowerBound,
    UpperBound,
}

#[derive(Debug, Clone, Copy)]
pub struct TranspositionTableEntry {
    pub value: isize,
    pub depth: usize,
    pub node_type: NodeType,
    pub source_node_id: Option<usize>,
}

pub struct TranspositionTable {
    entries: HashMap<u64, TranspositionTableEntry>,
}

impl TranspositionTable {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    pub fn get(&self, hash: u64) -> Option<&TranspositionTableEntry> {
        self.entries.get(&hash)
    }

    pub fn insert(&mut self, hash: u64, entry: TranspositionTableEntry) {
        match self.entries.get(&hash) {
            Some(existing) if existing.depth > entry.depth => {
                return;
            }
            _ => {}
        }

        self.entries.insert(hash, entry);
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }
}
