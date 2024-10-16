use std::env;

fn main() {
    println!(
        "cargo:rustc-check-cfg=cfg(\
            need_openssl_init,\
            need_openssl_probe,\
        )"
    );

    /* NOTE: Only-openssl setup so we're always using OpenSSL now, so we always need to check its version.
    *
    * // OpenSSL >= 1.1.0 can be initialized concurrently and is initialized correctly by libcurl.
    * // <= 1.0.2 need locking callbacks, which are provided by openssl_sys::init().
    * let use_openssl = match env::var("DEP_OPENSSL_VERSION_NUMBER") {
    *     Ok(version) => {
    *         let version = u64::from_str_radix(&version, 16).unwrap();
    *         if version < 0x1_01_00_00_0 {
    *             println!("cargo:rustc-cfg=need_openssl_init");
    *         }
    *         true
    *     }
    *     Err(_) => false,
    * };

    * if use_openssl {
    *     // The system libcurl should have the default certificate paths configured.
    *     if env::var_os("DEP_CURL_STATIC").is_some() {
    *         println!("cargo:rustc-cfg=need_openssl_probe");
    *     }
    * }
    */

    // NOTE: This should not fail because we are using vendored OpenSSL.
    let version = env::var("DEP_OPENSSL_VERSION_NUMBER").expect("OpenSSL version not found");
    let version = u64::from_str_radix(&version, 16).unwrap();
    if version < 0x1_01_00_00_0 {
        println!("cargo:rustc-cfg=need_openssl_init");
    }

    // NOTE: We're always using static linking now, so we always need `openssl-probe`.
    println!("cargo:rustc-cfg=need_openssl_probe");
}
