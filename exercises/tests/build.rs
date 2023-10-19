fn main() {
    // In tests7, we should set up an environment variable called `TEST_FOO`.
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // In tests8, we should enable the "pass" feature to make the testcase return early.
    println!("cargo:rustc-cfg=feature=\"pass\"");
}