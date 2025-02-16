#[macro_export]
macro_rules! profile {
    ($tag:literal, $body: expr) => {{
        let __internal__now = std::time::Instant::now();

        let __internal__result = $body;

        let __internal__elapsed = __internal__now.elapsed();
        $crate::udp_debugging::UDP_DEBUGGING.send(&format!(
            "{}:{}:{}: {} ms, ",
            file!(),
            line!(),
            $tag,
            __internal__elapsed.as_millis()
        ));

        __internal__result
    }};
}
