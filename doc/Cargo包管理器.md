## Cargo 介绍
`Cargo` 是 `Rust` 的构建系统和包管理器，类似于`npm`和`nodejs`的关系，可以通过`Cargo`内置的命令来创建项目、编译、打包等。

当通过上篇文章介绍的安装方式，安装了`Rust`后，就会自动安装`Carog`,可以通过以下命令查看是否安装成功：

```bash
cargo --version
# cargo 1.67.0 (8ecd4f20a 2023-01-10)
```

## 创建项目

```bash
cargo new hello_cargo
```
通过上述命令，会自动创一个名称为`hello_cargo`的项目，并在当前目录下创建同名的文件夹, 目录结构如下：

```bash
└── hello_cargo
    ├── .gitignore
    ├── Cargo.toml
    └── src
        └── main.rs
```

- 创建完项目会自动初始化`git`仓库，生成`.gitignore`文件，里面默认排除了`/target`文件，此文件为`Rust`编译后生成的代码文件。
- `Cargo.toml`文件类似于`npm`管理项目的`package.json`文件，其中声明了项目包信息和依赖项，[查看toml文件类型介绍](https://toml.io/)。
- `src`目录为源代码目录，里面有一个`main.rs`文件，并初始了`hello word`示例程序。

## 构建运行项目

在项目根目录下执行`cargo build`命令，将对项目进行编译，编译完成后生成`/target`目录，该目录用于存放编译后的文件，在目录下有一个`debug`目录，该目录里存放编译后的项目文件，并且包含一个可执行文件`hello_cargo`。

运行`hello_cargo`文件可输出`Hello, world!`结果

```bash
./target/debug/hello_cargo
```

通常情况下，会直接使用`cargo run`命令来进行编译和运行，它等同于执行`cargo build`后并运行`./target/debug/hello_cargo`

## 代码检查

当开发一个项目时，随着代码量的变多，我们并不会经常运行编译程序，我们只需要知道代码能不能通过编译即可，此时可通过运行`cargo check`命令来检查代码确保能编译成功，而不会生成编译后的代码，它的速度比运行`cargo build`快的多。

## 发布构建

项目准备好发布时，使用`cargo build --release`来优化编译代码，他会在`/target`目录下生成`release`目录，与`cargo build`的区别是，生成的代码是经过优化的，运行速度会更快，但同时构建速度会相对来说慢一些。

## 总结

- `cargo new project` 创建一个项目
- `cargo build` 构建项目生成可执行文件
- `cargo run` 构建项目并执行可执行文件
- `cargo build --release` 构建发布版本
