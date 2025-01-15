pub fn init_test_env() {
    std::env::set_var("RUST_LOG", "trace");
    std::env::set_var("AUTH_EXPIRE_SECS", "3600");
    std::env::set_var("REFRESH_EXPIRE_SECS", "3600");
    std::env::set_var("VERIFICATION_EXPIRE_SECS", "6000");
    std::env::set_var(
        "VERIFICATION_LINK_PREFIX",
        "http://localhost:8000/v1/auth/verify?token=",
    );
}
