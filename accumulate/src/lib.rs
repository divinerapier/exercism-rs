pub fn map<F, I, O>(input: Vec<I>, mut f: F) -> Vec<O>
    where
        F: FnMut(I) -> O,
{
    let mut v = vec![];
    for i in input {
        v.push(f(i));
    }
    v
}
