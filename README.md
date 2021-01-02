## About

[**📚 Read this template tutorial! 📚**](https://roche-rs.org/tutorials/project.html)

This template is designed roche and is used for compiling Rust libraries into docker and 
publishing the resulting package as knative service.

## 🚴 Usage

### Use 🐑 `roche init default` to Clone this Template

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
roche test --lib
```

### 🐑 Running integration tests

```
docker run -p 8080:8080 name/of/image
cargo test --test '*'
```
