mod http;
mod net;

pub(crate) fn main() {
    let mut i = 0;
    while i < 5 {
        println!("Hello, world! {}",i);
        i += 1;
    }
    
    match http::request() {
        _ => {}
    }
}
