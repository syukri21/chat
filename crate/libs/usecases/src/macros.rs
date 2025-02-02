#[macro_export]
macro_rules! measure_time {
    ($name:expr, $expr:expr) => {{
        use std::time::Instant;
        let start = Instant::now();
        let result = $expr;
        let duration = start.elapsed();
        log::info!("{}={:?}", $name, duration);
        result
    }};
}
