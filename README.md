<div align="center">

  <h1><code>WRAPPED-SM-FOR-JAVA</code></h1>

  <strong>
  A Rust project using <a href="https://github.com/jni-rs/jni-rs">JNI</a> to wrap 
  <a href="https://github.com/bytesboy/Yet-Another-Rust-Implementation-Of-SM-Algorithms.git">
  Yarism</a> for java.
  </strong>


</div>

## ⚓️ About

This project is designed to compile
<a href="https://github.com/bytesboy/Yet-Another-Rust-Implementation-Of-SM-Algorithms.git"> Yarism </a>
with <a href="https://github.com/jni-rs/jni-rs">JNI</a> bindings into dylib or so lib for java.

## 💡 Attention

1. Define Java class and method. For example: `HelloWorld.java`
2. Execute `javac -h . HelloWorld.java` command to compile `*.h` file.
3. Get the name and type signature from the `.h` file.
4. Define Rust method with the name and type signature

## 🚴 Usage

### 🛠️ Build

```
cargo build --release
```

## 🔋 Relative Projects

* [`Jasm`](https://github.com/bytesboy/jasm.git)