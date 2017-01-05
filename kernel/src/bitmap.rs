use core::array::FixedSizeArray;

pub const BITS_PER_ITEM: usize = 64;
const FULL_ITEM: u64 = u64::max_value();

pub struct Bitmap<T: FixedSizeArray<u64>> {
    data: T
}

impl<T: FixedSizeArray<u64>> Bitmap<T> {
    pub const fn new(data: T) -> Bitmap<T> {
        Bitmap { data: data }
    }

    fn slice(&self) -> &[u64] {
        self.data.as_slice()
    }

    fn mut_slice(&mut self) -> &mut [u64] {
        self.data.as_mut_slice()
    }

    pub fn set(&mut self, index: usize, value: bool) {
        if value {
            self.mut_slice()[index / BITS_PER_ITEM] |= 1 << (index % BITS_PER_ITEM);
        } else {
            self.mut_slice()[index / BITS_PER_ITEM] &= !(1 << (index % BITS_PER_ITEM));
        }
    }

    pub fn get(&self, index: usize) -> bool {
        (self.slice()[index / BITS_PER_ITEM] & (1 << (index % BITS_PER_ITEM))) != 0
    }

    pub fn first_unset(&self, start_index: usize) -> Option<usize> {
        let start = start_index/BITS_PER_ITEM;
        let end = self.slice().len()/BITS_PER_ITEM;

        for index in start..end {
            if self.slice()[index] == FULL_ITEM {
                continue;
            }

            let start_bit = index * BITS_PER_ITEM;
            let end_bit = (index + 1) * BITS_PER_ITEM;

            for bit_index in start_bit..end_bit {
                if !self.get(bit_index) {
                    return Some(bit_index);
                }
            }
        }

        None
    }
}
