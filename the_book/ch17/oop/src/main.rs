use oop::AveragedCollection;

fn main() {
    let mut ac = AveragedCollection::new();
    ac.add(1);
    ac.add(2);
    ac.add(3);
    assert_eq!(2.0, ac.average());
}
