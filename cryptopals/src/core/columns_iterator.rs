use std::iter::Iterator;

pub struct Columns<T> {
    chunks: Vec<T>,
}

impl<T> Iterator for Columns<T>
where
    T: Iterator,
{
    type Item = Vec<T::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        let result: Self::Item = self.chunks.iter_mut().filter_map(Iterator::next).collect();

        if result.is_empty() {
            None
        } else {
            Some(result)
        }
    }
}

impl<T: Iterator> Columns<T> {
    #[must_use]
    pub fn from(chunks: Vec<T>) -> Self {
        Self { chunks }
    }
}
