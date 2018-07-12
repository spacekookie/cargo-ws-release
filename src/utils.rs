pub trait VecSearch<T>
where
    T: PartialEq + Eq + Clone,
{
    fn contains(&self, t: T) -> Option<T>;
}

impl<T> VecSearch<T> for Vec<T>
where
    T: PartialEq + Eq + Clone,
{
    fn contains(&self, t: T) -> Option<T> {
        self.iter().find(|x| x == &&t).map(|t| t.clone())
    }
}
