mod averaged_collection;


fn main() {
    let mut collection = averaged_collection::AveragedCollection::new();
    println!("Adding 1");
    collection.add(1);

    println!("Adding 2");
    collection.add(2);

    println!("Adding 3");
    collection.add(3);

    println!("Average: {}", collection.average());
}
