# OsBridge
> A cross platform API bridge based on Rust language

## examples
- add os_bridge crate to your project
```
cargo add os-bridge
```

- create code
```
use os_bridge::{Bridge, OsBridge};

fn main() {
    let bd = Bridge::new();
    println!(" pid = {:?}", bd.get_pid());
}
```