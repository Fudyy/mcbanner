# MCBanner

A simple library to generate Minecraft banner images.

It uses the [image](https://docs.rs/image/) under the hood for the image generation.
## Usage

Add this to your `Cargo.toml`:
```toml
[dependencies]
mcbanner = "0.1.1"
```
### Example

The `Banner` struct is the main entrypoint of the library.
 ```rust
 use mcbanner::{Banner, Pattern, MCColor};
 
 fn main() {
     let mut banner = Banner::new(MCColor::Lime);
     banner.add_pattern(Pattern::Creeper, MCColor::Black);
     banner.render();
     banner.save("banner.png").unwrap();
 }
 ```
 Running this example code gives you the next result:

<img src=".github/assets/banner.png" height=300/>

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.


