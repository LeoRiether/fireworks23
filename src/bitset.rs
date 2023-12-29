pub struct Bitset {
    inner: Vec<u64>,
}

impl Bitset {
    pub fn new(n: usize) -> Self {
        let inner = vec![0; (n + 63) / 64];
        Self { inner }
    }

    pub fn set(&mut self, i: usize) {
        self.inner[i / 64] |= 1 << (i % 64);
    }

    pub fn test(&self, i: usize) -> bool {
        self.inner[i / 64] & (1 << (i % 64)) != 0
    }

    pub fn clear(&mut self) {
        for x in &mut self.inner {
            *x = 0;
        }
    }

    pub fn iter(&self) -> BitsetIter {
        BitsetIter {
            inner: &self.inner,
            block: self.inner.len() as isize - 1,
            offset: 63,
        }
    }
}

/// Iterator over the set indices in a bitset in *descending* order.
pub struct BitsetIter<'a> {
    inner: &'a [u64],
    block: isize,
    offset: isize,
}

impl<'a> Iterator for BitsetIter<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.block < 0 {
                return None;
            }

            let block = self.inner[self.block as usize];
            if block == 0 {
                self.block -= 1;
                self.offset = 63;
                continue;
            }

            let offset = self.offset;
            self.offset -= 1;
            if block & (1 << offset) != 0 {
                return Some((self.block * 64 + offset) as usize);
            }
        }
    }
}
