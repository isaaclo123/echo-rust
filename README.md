cross build --target armv7-linux-androideabi

adb push target/armv7-linux-androideabi/debug/echo-rust /data/local/tmp

added libtinyalso.so into .rustup/toolchains/stable-x86/lib/rustlib/armv7-linbux-androideabi/lib/libtinyalsa
