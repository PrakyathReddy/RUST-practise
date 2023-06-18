
fn main() {

    let _four = IpAddr::V4;
    let _six = IpAddr::V6;
    // route(four);
    // route(six);
    #[derive(Debug)]
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    
    // struct IpAddr {
    //     kind: IpAddrKind,
    //     address: String,
    // }

    let home = IpAddr::V4(127,0,0,1);
    println!("{:?}",home);
    let _loopback = IpAddr::V6(String::from("::1"));

    // fn route(_ip_kind: IpAddrKind) {}
}


