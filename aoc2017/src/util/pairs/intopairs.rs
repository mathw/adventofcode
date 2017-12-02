use super::pairs::Pairs;

pub trait IntoPairs<Item, Iter>
    where Item: Copy,
          Iter: Iterator<Item = Item> + Clone
{
    fn pairs(&self) -> Pairs<Item, Iter>;
}

impl<Item, Iter> IntoPairs<Item, Iter> for Iter
    where Item: Copy,
          Iter: Iterator<Item = Item> + Clone
{
    fn pairs(&self) -> Pairs<Item, Self> {
        Pairs::new(self.clone())
    }
}