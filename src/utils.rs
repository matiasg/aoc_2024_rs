use std::collections::HashMap;

pub fn counter<I>(ns: I) -> HashMap<I::Item, usize>
where
    I: IntoIterator,
    I::Item: Eq + core::hash::Hash,
{
    let mut ret: HashMap<I::Item, usize> = HashMap::new();
    for n in ns {
        *ret.entry(n).or_default() += 1;
    }
    ret
}
