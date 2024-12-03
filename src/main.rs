fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {

    use log::{debug, error, info, trace, warn};

    #[test]
    fn test_log(){
        env_logger::init();

        error!("This is an error");
        warn!("This is a warning");
        info!("This is an info");
        debug!("This is a debug");
        trace!("This is a trace");
    }

}