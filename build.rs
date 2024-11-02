fn main() {
	// Get the API key from the build environment
	if let Ok(_api_key) = std::env::var("LMWR_API_KEY") {
		// Inject the environment variable as a compile-time constant
		// println!("cargo:rustc-env=LMWR_API_KEY={}", api_key);
	} else {
		panic!("LMWR_API_KEY environment variable must be set at compile time.");
	}
}
