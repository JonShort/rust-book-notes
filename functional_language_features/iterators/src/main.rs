fn main() {
    let v1 = vec![1, 2, 3];
    // iterators are lazy, so nothing happens until we
    // use a method on one
    let v1_iter = v1.iter();

    // iterates over each item
    for val in v1_iter {
        println!("Got: {}", val);
    }
}

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    // sum takes ownership of v1_iter
    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}

#[test]
fn iterator_use() {
    let v1: Vec<i32> = vec![1, 2, 3];

    // this code will warn the "unused_must_use" as the iterator is not used
    // v1.iter().map(|x| x + 1);

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}

#[test]
fn iterator_demonstration() {
    // Also note that the values we get from the calls to next are
    // immutable references to the values in the vector. The iter
    // method produces an iterator over immutable references. If we
    // want to create an iterator that takes ownership of v1 and
    // returns owned values, we can call into_iter instead of iter.
    // Similarly, if we want to iterate over mutable references, we
    // can call iter_mut instead of iter.
    let v1 = vec![1, 2, 3];
    let v2 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();
    let mut v2_iter = v2.into_iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

    // notce these are owned
    assert_eq!(v2_iter.next(), Some(1));
    assert_eq!(v2_iter.next(), Some(2));
    assert_eq!(v2_iter.next(), Some(3));
    assert_eq!(v2_iter.next(), None);
}
