pub struct PowerSet<'a, T: 'a> {
    source: &'a [T],
    position: usize,
}

impl<'a, T> PowerSet<'a, T>
where
    T: Clone,
{
    pub fn new(source: &'a [T]) -> PowerSet<'a, T> {
        PowerSet {
            source: source,
            position: 0,
        }
    }
}

impl<'a, T> Iterator for PowerSet<'a, T>
where
    T: Clone,
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if 2usize.pow(self.source.len() as u32) <= self.position {
            None
        } else {
            let res = self.source
                .iter()
                .enumerate()
                .filter(|&(i, _)| (self.position >> i) % 2 == 1)
                .map(|(_, element)| element.clone())
                .collect();
            self.position = self.position + 1;
            Some(res)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_empty() {
        let empty: Vec<i32> = Vec::new();
        let powerset: Vec<Vec<i32>> = PowerSet::new(&empty[..]).collect();
        let expected_empty: Vec<Vec<i32>> = vec![Vec::new()];
        assert_eq!(expected_empty, powerset);
    }

    #[test]
    pub fn test_elements() {
        let powerset: Vec<Vec<i32>> = PowerSet::new(&[1, 2, 3, 4]).collect();
        let expected_elements = vec![
            vec![],
            vec![1],
            vec![2],
            vec![1, 2],
            vec![3],
            vec![1, 3],
            vec![2, 3],
            vec![1, 2, 3],
            vec![4],
            vec![1, 4],
            vec![2, 4],
            vec![1, 2, 4],
            vec![3, 4],
            vec![1, 3, 4],
            vec![2, 3, 4],
            vec![1, 2, 3, 4],
        ];
        assert_eq!(expected_elements, powerset);
    }

    #[test]
    pub fn test_size() {
        let powerset: Vec<Vec<i32>> = PowerSet::new(&[1, 2, 3, 4, 5, 6, 7, 8, 9]).collect();
        assert_eq!(512, powerset.len());
    }
}
