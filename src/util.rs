pub(crate) fn merge_overlapped<A>(iter : A) -> Vec<(usize, f64)> where A : IntoIterator<Item=(usize, f64)>
{
    let mut container : Vec<(usize, f64)> = Vec::new();
    for i in iter.into_iter() {
        if let Some(x) = container.last_mut() {
            if i.0 == x.0 {
                x.1 += i.1;
                continue;
            }
        }
        container.push(i);
    }
    return container;
}