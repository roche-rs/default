## About

[**📚 Read this template tutorial! 📚**][template-docs]

This template is designed for compiling Rust libraries into docker and 
publishing the resulting package a knative service.

[tutorials]: https://roche-rs.org/tutorials/index.html
[template-docs]: TBD

## 🚴 Usage

### Use 🐑 `roche init default` to Clone this Template
R
Roche uses the excellent cargo-generate under the hood.
[Learn more about `cargo generate` here.](https://github.com/ashleygwilliams/cargo-generate)

```
roche init default --name my-project
cd my-project
```

### 🛠️ Build with `roche build`

```
docker login
roche build
```

### 🔬 Testing the library

```
cargo test --lib
```

### 🐑 Running integration tests

```
docker run -p 8080:8080 name/of/image
cargo test
```
