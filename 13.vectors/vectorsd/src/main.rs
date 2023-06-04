fn main() {
    let v = vec![1,2];
    println!("{:?}",v);

    let mut v2 = Vec::new();
    v2.push(2);
    v2.push(3);
    println!("{:?}", v2); 

    let names = vec!["kpr","abc", "def"];
    println!("{:?}", &names.get(1));
}
