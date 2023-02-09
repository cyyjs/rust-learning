## 安装Rust

`Mac`和`Linux`直接执行以下命令来安装

```bash
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

安装完成后通过以下命令检测是否安装成功

```bash
rustc --version
# rustc 1.67.0 (fc594f156 2023-01-24)
```

## Hello Word!

创建`hello.rs`文件，并输入以下代码

```rust
fn main() {
    println!("Hello, world!");
}
```

### 编译文件
执行如下命令，可完成文件编译，并生成同名的可执行二进制文件

```bash
rustc hello.rs
```

### 执行文件

执行如下命令，可输出程序结果：`Hello, world!`
```bash
./hello
```

## 程序分析

`fn`为`Rust`程序函数声明的关键字，`main`是一个特殊函数名 ，每个`Rust`程序都使用它来作为程序执行入口。

