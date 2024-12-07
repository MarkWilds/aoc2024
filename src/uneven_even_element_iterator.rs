pub struct UnEvenEvenElementIterator<'a, T> {
    data: &'a Vec<T>,
    index: usize,
    modulo: usize,
}

impl<'a, T> UnEvenEvenElementIterator<'a, T> {
    pub fn new(data: &'a Vec<T>, modulo: usize) -> Self {
        Self { data, index: 0, modulo }
    }
}

impl<'a, T> Iterator for UnEvenEvenElementIterator<'a, T>
where
    T: Copy, // Or Clone, depending on your use case
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.data.len() {
            let current_index = self.index;
            self.index += 1;

            // Check if the index is even
            if current_index % 2 == self.modulo {
                return Some(self.data[current_index]);
            }
        }
        None
    }
}
