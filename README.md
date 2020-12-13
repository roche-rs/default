## About

[**📚 Read this template tutorial! 📚**][template-docs]

This template is designed for compiling Rust libraries into docker and 
publishing the resulting package a knative service.

[tutorials]: TBD
[template-docs]: TBD

## 🚴 Usage

### 🐑 Use `cargo roche generate` to Clone this Template

[Learn more about `cargo generate` here.](https://github.com/ashleygwilliams/cargo-generate)

```
roche generate --name my-project
cd my-project
```

### 🛠️ Build with `roche build`

```
roche build
```

### 🔬 Test in docker container with `roche test`

```
roche test --headless --firefox
```
