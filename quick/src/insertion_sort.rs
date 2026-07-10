pub fn insertion_sort<T, C>(collection: &mut C)
where
    T: Ord,
    C: AsMut<[T]>,
{
    let coll = collection.as_mut();
    for i in 1..coll.len() {
        if coll[i] < coll[i - 1] {
            for j in (1..=i).rev() {
                if coll[j] < coll[j - 1] {
                    coll.swap(j, j - 1);
                }
            }
        }
    }
}
