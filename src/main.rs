fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {

    use log::{debug, error, info, trace, warn};

    #[test]
    fn test_log(){
        log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

        error!("This is an error");
        warn!("This is a warning");
        info!("This is an info");
        debug!("This is a debug");
        trace!("This is a trace");
    }

}