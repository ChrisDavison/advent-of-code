#[macro_export]
macro_rules! time_solution {
    ($day:ident, $body:expr) => {
        let now = std::time::Instant::now();
        let res = $body;
        match res {
            Ok(_) => println!("    Time: {:.2}ms", now.elapsed().as_nanos() / 1_000_000),
            Err(e) => eprintln!("D{}: {}", $day + 1, e),
        }
    };
}
