
fn main() {

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    route(four);
    route(six);

    enum IpAddrKind {
        V4,
        V6,
    }
    
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let _home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let _loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    fn route(_ip_kind: IpAddrKind) {}
}


