use std::sync::Once;

static START: Once = Once::new();

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    //#[cfg(feature = "console_error_panic_hook")]
    START.call_once(|| {
        console_error_panic_hook::set_once();
        tracing_wasm::set_as_global_default();
        use log::Level;
        console_log::init_with_level(Level::Debug).expect("error initializing log");
    });
}
