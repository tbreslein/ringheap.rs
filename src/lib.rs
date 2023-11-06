#![allow(dead_code)]

/// Custom Minheap built on top of an unsafe ring buffer.
///
/// As an important implementation detail when reading this code: The ring
/// buffer part is based on [Vec], and its length is used as the ring buffer
/// capacity.
#[derive(Debug, Clone)]
pub struct RingHeap<T: Copy + std::cmp::PartialOrd> {
    data: Vec<T>,
    start: usize,
    end: usize,
    len: usize,
}

impl<T: Copy + std::cmp::PartialOrd + std::fmt::Debug> RingHeap<T> {
    /// Construct an empty [RingHeap].
    pub fn new() -> Self {
        Self::with_capacity(1)
    }

    /// Construct a new [RingHeap] with a pre-allocated capacity.
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            data: Vec::with_capacity(cap),
            start: 0,
            end: 0,
            len: 0,
        }
    }

    /// Clears the [RingHeap].
    ///
    /// Does not actually drop the data, but makes it inaccessible.
    pub fn clear(&mut self) {
        self.start = 0;
        self.end = 0;
        self.len = 0;
    }

    /// Insert a new item into the [RingHeap]
    pub fn insert(&mut self, x: T) {
        self.push(x);
        let idx = if self.end == 0 {
            self.data.len() - 1
        } else {
            self.end - 1
        };
        self.heapify_up(idx);
    }

    /// Returns an [Option] of the smallest item in the [RingHeap].
    ///
    /// Returns a `Some(x)`, where `x` is that smallest item, if the heap is not
    /// empty, and a `None` otherwise.
    ///
    /// Note that the piece of data is simply being copied and not actually
    /// removed from the heap; its just being made inaccessible.
    pub fn pop(&mut self) -> Option<T> {
        if self.len > 0 {
            let x = self.data[self.start];
            self.start = (self.start + 1) % self.data.len();
            self.len -= 1;
            self.heapify_down(0);
            Some(x)
        } else {
            None
        }
    }

    fn push(&mut self, x: T) {
        if self.len + 1 >= self.data.len() {
            self.grow(x);
        }
        self.data[self.end] = x;
        self.end = (self.end + 1) % self.data.len();
        self.len += 1;
    }

    fn peek(&self) -> Option<T> {
        if self.len > 0 {
            Some(self.data[self.start])
        } else {
            None
        }
    }

    fn real_idx(&self, i: usize) -> usize {
        (self.start + i) % self.data.len()
    }

    fn parent_idx(i: usize) -> usize {
        (i - 1) / 2
    }

    fn left_child_idx(i: usize) -> usize {
        1 + 2 * i
    }

    fn right_child_idx(i: usize) -> usize {
        2 + 2 * i
    }

    fn swap(&mut self, i: usize, j: usize) {
        let real_i = self.real_idx(i);
        let real_j = self.real_idx(j);
        self.data.swap(real_i, real_j);
    }

    fn heapify_up(&mut self, i: usize) {
        if i == self.start {
            return;
        }
        let parent_idx = Self::parent_idx(i);
        if self.get(parent_idx) > self.get(i) {
            self.swap(i, parent_idx);
            self.heapify_up(parent_idx);
        }
    }

    fn heapify_down(&mut self, i: usize) {
        let left_child_idx = Self::left_child_idx(i);
        let right_child_idx = Self::right_child_idx(i);
        if i > self.len || left_child_idx >= self.len {
            return;
        }

        let left_val = self.get(left_child_idx);
        let this_val = self.get(i);
        if right_child_idx >= self.len {
            if this_val > left_val {
                self.swap(i, left_child_idx);
                self.heapify_down(left_child_idx);
            }
            return;
        }

        let right_val = self.get(right_child_idx);
        if left_val > right_val && this_val > right_val {
            self.swap(i, right_child_idx);
            self.heapify_down(right_child_idx);
        } else if right_val > left_val && this_val > left_val {
            self.swap(i, left_child_idx);
            self.heapify_down(left_child_idx);
        }
    }

    fn set(&mut self, i: usize, x: T) {
        debug_assert!(i < self.len);
        let idx = self.real_idx(i);
        self.data[idx] = x;
    }

    fn get(&self, i: usize) -> T {
        debug_assert!(i < self.len);
        self.data[self.real_idx(i)]
    }

    fn grow(&mut self, x: T) {
        let old_len = self.data.len();
        let new_len = usize::max(2, 2 * self.data.len());

        // NOTE: This fills up the vec with junk data!
        // The function argument is simply used to provide that junk data so
        // the vec entries are initialised.
        self.data.resize(new_len, x);
        if self.start > self.end {
            let n_to_move = old_len - self.start;
            let new_start = new_len - n_to_move;
            self.data
                .copy_within(self.start..self.start + n_to_move, new_start);
            self.start = new_start;
        }
    }
}

#[cfg(test)]
mod test {
    use super::RingHeap;

    #[test]
    fn inserts_correctly() {
        let mut heap = RingHeap::<i32>::new();

        assert_eq!(heap.data.len(), 0);
        assert_eq!(heap.len, 0);
        assert_eq!(heap.start, 0);
        assert_eq!(heap.end, 0);
        assert_eq!(heap.peek(), None);

        heap.insert(2);
        assert_eq!(heap.data.len(), 2);
        assert_eq!(heap.len, 1);
        assert_eq!(heap.start, 0);
        assert_eq!(heap.end, 1);
        assert_eq!(heap.peek(), Some(2));

        heap.insert(1);
        assert_eq!(heap.data.len(), 4);
        assert_eq!(heap.len, 2);
        assert_eq!(heap.start, 0);
        assert_eq!(heap.end, 2);
        assert_eq!(heap.peek(), Some(1));

        heap.insert(4);
        assert_eq!(heap.data.len(), 4);
        assert_eq!(heap.len, 3);
        assert_eq!(heap.start, 0);
        assert_eq!(heap.end, 3);
        assert_eq!(heap.peek(), Some(1));
    }

    #[test]
    fn pops_correctly() {
        let mut heap = RingHeap::<i32>::new();
        heap.insert(2);
        heap.insert(1);
        heap.insert(4);
        heap.insert(6);
        heap.insert(-1);
        heap.insert(-3);
        heap.insert(3);
        assert_eq!(heap.data.len(), 8);
        assert_eq!(heap.len, 7);
        assert_eq!(heap.start, 0);
        assert_eq!(heap.end, 7);

        // NOTE: pop shifts .start upwards, without touching the .data or .end
        assert_eq!(heap.pop(), Some(-3));
        assert_eq!(heap.len, 6);
        assert_eq!(heap.start, 1);
        assert_eq!(heap.end, 7);

        assert_eq!(heap.pop(), Some(-1));
        assert_eq!(heap.len, 5);
        assert_eq!(heap.start, 2);
        assert_eq!(heap.end, 7);

        assert_eq!(heap.pop(), Some(1));
        assert_eq!(heap.len, 4);
        assert_eq!(heap.start, 3);
        assert_eq!(heap.end, 7);

        assert_eq!(heap.pop(), Some(2));
        assert_eq!(heap.len, 3);
        assert_eq!(heap.start, 4);
        assert_eq!(heap.end, 7);

        assert_eq!(heap.pop(), Some(3));
        assert_eq!(heap.len, 2);
        assert_eq!(heap.start, 5);
        assert_eq!(heap.end, 7);

        assert_eq!(heap.pop(), Some(4));
        assert_eq!(heap.len, 1);
        assert_eq!(heap.start, 6);
        assert_eq!(heap.end, 7);

        assert_eq!(heap.pop(), Some(6));
        assert_eq!(heap.len, 0);
        assert_eq!(heap.start, 7);
        assert_eq!(heap.end, 7);

        assert_eq!(heap.pop(), None);
        assert_eq!(heap.len, 0);
        assert_eq!(heap.start, 7);
        assert_eq!(heap.end, 7);

        assert_eq!(heap.data.len(), 8);
    }

    #[test]
    fn complex_example() {
        let mut heap = RingHeap::<i32>::new();
        heap.insert(12);
        assert_eq!(heap.peek(), Some(12));
        heap.insert(10);
        assert_eq!(heap.peek(), Some(10));
        assert_eq!(heap.pop(), Some(10));
        assert_eq!(heap.pop(), Some(12));
        assert_eq!(heap.pop(), None);

        heap.insert(5);
        heap.insert(-5);
        heap.insert(0);
        assert_eq!(heap.pop(), Some(-5));
        assert_eq!(heap.peek(), Some(0));

        heap.insert(10);
        heap.insert(-7);
        assert_eq!(heap.pop(), Some(-7));
        assert_eq!(heap.pop(), Some(0));

        heap.insert(11);
        heap.insert(-7);
        assert_eq!(heap.pop(), Some(-7));
        assert_eq!(heap.peek(), Some(5));
    }
}
