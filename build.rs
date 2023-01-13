extern crate cc;

fn main() {
  let mut build = cc::Build::new();
  let tool = build.try_get_compiler().unwrap();
  let is_debug = std::env::var("DEBUG").ok().is_some();
  let is_msvc = tool.is_like_msvc();

  build.include("src/sokol/c/");

  if cfg!(target_os = "macos") {
    build.flag("-fobjc-arc").file("src/sokol/c/sokol.m");
  } else {
    build.file("src/sokol/c/sokol.c");
  }
  build.flag_if_supported("-Wno-unused-parameter");

  if cfg!(target_os = "windows") && is_msvc {
    build.flag("-DSOKOL_D3D11");
    println!("cargo:rustc-cfg=gfx=\"d3d11\"");
  } else if cfg!(target_os = "macos") {
    build.flag("-DSOKOL_METAL");
    println!("cargo:rustc-cfg=gfx=\"metal\"");
    println!("cargo:rustc-link-lib=framework=Cocoa");
    println!("cargo:rustc-link-lib=framework=QuartzCore");
    println!("cargo:rustc-link-lib=framework=Metal");
    println!("cargo:rustc-link-lib=framework=MetalKit");
    println!("cargo:rustc-link-lib=framework=AudioToolbox");
  } else {
    build.flag("-DSOKOL_GLCORE33");
    println!("cargo:rustc-cfg=gfx=\"glcore33\"");
  }
  if cfg!(target_os = "linux") {
    println!("cargo:rustc-link-lib=dylib=GL");
    println!("cargo:rustc-link-lib=dylib=X11");
    println!("cargo:rustc-link-lib=dylib=asound");
  }

  if cfg!(target_os = "windows") {
    if !is_msvc {
      build
        .flag("-D_WIN32_WINNT=0x0601")
        .flag_if_supported("-Wno-cast-function-type")
        .flag_if_supported("-Wno-sign-compare")
        .flag_if_supported("-Wno-unknown-pragmas");

      println!("cargo:rustc-link-lib=static=gdi32");
      println!("cargo:rustc-link-lib=static=ole32");
    }
  }

  if is_debug {
    build.flag("-D_DEBUG").flag("-DSOKOL_DEBUG");
  }

  build.compile("sokol");
}