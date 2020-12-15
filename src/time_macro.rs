#[macro_export]
macro_rules! time_solution {
    ($day:ident, $body:expr) => {
        let now = std::time::Instant::now();
        let res = $body;
        match res {
            Ok(_) => println!("    Time: {:.2}ms", now.elapsed().as_nanos() as f64 / 1e6),
            Err(e) => eprintln!("D{}: {}", $day, e),
        }
    };
}
