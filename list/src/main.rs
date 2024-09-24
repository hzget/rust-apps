use list::LinkedList;

fn main() {
    let mut list = LinkedList::new();
    list.push(401);
    list.push(200);
    println!("{:?}", list.length()); // 2
    println!("{:?}", list.pop()); // Some(200)
    println!("{:?}", list.pop()); // Some(401)
    println!("{:?}", list.pop()); // None

    let mut list = LinkedList::new();
    list.push("yes");
    list.push("no");
    println!("{:?}", list.pop()); // Some("no")
    println!("{:?}", list.pop()); // Some("yes")
    println!("{:?}", list.pop()); // None

    let mut list = LinkedList::new();
    list.push("yes");
    list.push("no");
    for x in list {
        println!("{:?}", x)
    }
}
