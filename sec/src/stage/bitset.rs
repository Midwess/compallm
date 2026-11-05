#[derive(Clone, Default)]
pub(super) struct BitSet {
    words: Vec<u64>,
}
impl BitSet {
    pub fn new(len: usize) -> Self {
        Self {
            words: vec![0; len.div_ceil(64)],
        }
    }
    #[inline]
    pub fn insert(&mut self, index: usize) {
        self.words[index / 64] |= 1 << (index % 64);
    }
    #[inline]
    pub fn remove(&mut self, index: usize) {
        self.words[index / 64] &= !(1 << (index % 64));
    }
    #[inline]
    pub fn contains(&self, index: usize) -> bool {
        self.words[index / 64] & (1 << (index % 64)) != 0
    }
    #[inline]
    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.words.iter().zip(&other.words).all(|(a, b)| a & b == 0)
    }
    pub fn copy_from(&mut self, other: &Self) {
        self.words.clone_from(&other.words);
    }
    pub fn reset(&mut self, len: usize) {
        self.words.resize(len.div_ceil(64), 0);
        self.words.fill(0);
    }
    pub fn ones(&self) -> impl Iterator<Item = usize> + '_ {
        self.words.iter().enumerate().flat_map(|(word, &bits)| {
            let mut remaining = bits;
            std::iter::from_fn(move || {
                if remaining == 0 {
                    return None;
                }
                let bit = remaining.trailing_zeros() as usize;
                remaining &= remaining - 1;
                Some(word * 64 + bit)
            })
        })
    }
}
