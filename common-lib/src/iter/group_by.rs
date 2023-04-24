use alloc::collections::BTreeMap;
use alloc::vec::Vec;

use crate::iter::group::Group;
use crate::iter::key_query::KeyQuery;
use crate::iter::mutable_group::MutableGroup;

pub struct GroupBy<Key, Value> {
    groups: Vec<Group<Key, Value>>,
    index: usize,
}


impl<Value, Key> GroupBy<Key, Value>
where
    Key: PartialEq + Ord + Clone,
    Value: Clone,
{
    pub fn new(
        iter: impl Iterator<Item = Value>,
        mut query: impl KeyQuery<Key, Value>,
    ) -> GroupBy<Key, Value> {
        let values: Vec<Value> = iter.into_iter().collect();
        let mut groups: BTreeMap<Key, MutableGroup<Key, Value>> = BTreeMap::new();

        values
            .into_iter()
            .for_each(|v| {
                new_group_or_push_value(&mut groups, v, &mut query);
            });

        Self {
            groups: groups
                .into_iter()
                .map(|g| g.1.into_immutable_group())
                .collect(),
            index: 0,
        }
    }


    pub fn into_vec(self) -> Vec<Group<Key, Value>> {
        self.groups
    }
}


impl<Key, Value> Iterator for GroupBy<Key, Value>
where
    Value: Clone,
    Key: Clone,
{
    type Item = Group<Key, Value>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.groups.len() <= self.index {
            return None;
        }

        let group = self
            .groups
            .get(self.index)
            .cloned();
        self.index += 1;
        group
    }
}


fn new_group_or_push_value<Key, Value>(
    groups: &mut BTreeMap<Key, MutableGroup<Key, Value>>,
    value: Value,
    query: &mut impl KeyQuery<Key, Value>,
) where
    Key: PartialEq + Ord + Clone,
    Value: Clone,
{
    let key = query.query(value.clone());

    if let Some(group) = groups.get_mut(&key) {
        group.push_value(value);
    } else {
        groups.insert(key.clone(), MutableGroup::new(key, value));
    }
}


#[cfg(test)]
mod tests {
    use alloc::vec::Vec;

    use crate::iter::group::Group;
    use crate::iter::group_by::GroupBy;
    use crate::iter::Grouping;

    #[test]
    fn it_generate_two_groups() {
        let values = [0, 1, 2, 3];

        let groups: Vec<Group<bool, i32>> =
            GroupBy::new(values.into_iter(), |v| (v % 2) == 0).collect();
        assert_eq!(groups.len(), 2);


        assert!(groups[0].eq_key(&false));
        assert!(groups[1].eq_key(&true));
    }


    #[test]
    fn it_generate_two_groups_from_iter() {
        let values = [0i32, 1, 2, 3];

        let groups: Vec<Group<bool, i32>> = values
            .into_iter()
            .group_by(|v| (v % 2) == 0)
            .collect();

        assert_eq!(groups.len(), 2);
        assert!(groups[0].eq_key(&false));
        assert!(groups[1].eq_key(&true));
    }
}
