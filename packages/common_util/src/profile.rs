#[macro_export]
macro_rules! profile {
    ($tag:literal, $body: expr) => {{
        #[cfg(debug_assertions)]
        let __internal__now = std::time::Instant::now();

        let __internal__result = $body;

        #[cfg(debug_assertions)]
        let __internal__elapsed = __internal__now.elapsed();
        #[cfg(debug_assertions)]
        println!(
            "{}:{}:{}: {} ms, {} ns",
            file!(),
            line!(),
            $tag,
            __internal__elapsed.as_millis(),
            __internal__elapsed.as_nanos()
        );

        __internal__result
    }};
}
