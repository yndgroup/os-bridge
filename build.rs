fn main() {
  #[cfg(target_os = "windows")]
  cc::Build::new()
    .cpp(true)
    .file("addon/src/windows/entry.cpp")
    .compile("entry");

  #[cfg(target_os = "linux")]
  cc::Build::new()
    .cpp(true)
    .file("addon/src/linux/entry.cpp")
    .compile("entry");

  #[cfg(target_os = "macos")]
  cc::Build::new()
    .cpp(true)
    .file("addon/src/macos/entry.cpp")
    .compile("entry");
}
