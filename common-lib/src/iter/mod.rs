use crate::iter::group_by::GroupBy;
use crate::iter::key_query::KeyQuery;

pub mod group;
pub mod group_by;
pub mod key_query;
pub mod mutable_group;


pub trait Grouping<Key, Value>
where
    Key: PartialEq,
{
    fn group_by(self, query: impl KeyQuery<Key, Value>) -> GroupBy<Key, Value>;
}


impl<Key, Value, Ite> Grouping<Key, Value> for Ite
where
    Value: Clone,
    Key: PartialEq + Ord + Clone,
    Ite: Iterator<Item = Value>,
{
    fn group_by(self, query: impl KeyQuery<Key, Value>) -> GroupBy<Key, Value> {
        GroupBy::new(self, query)
    }
}
