# type-lang

Type-lang or **ty**. Is a rust/zig/typescript inspired language. There is one main goal of **ty** and that is to become the best high level language without a Garbage Collector. Every decision for syntax, language features, and implementation will point back to that main goal.

#### Build from source

Ty requires a relatively recent version of cargo.

```
cd ~
git clone https://github.com/coffeebe4code/type-lang
cd type-lang
cargo build --release
```

Optionally, you can run the tests with

```
cargo test
```

Add `ty` to your path in your shell of choice.

User Environment Variables if you are on windows. the output was put into the `target/release` directory.

```
export PATH=$HOME/type-lang/target/release:$PATH
```

Right now, the only thing that builds or works is a main function, with simple addition or subtraction. setting variables, and using those variables work. There is no type system yet.

You can view the location of the current main test file here.
[main.ty](./test/main.ty)
That file will have almost every feature once it is supported, so check the file, for possibilities

Future versions of ty will have a much more robust easy to use build system.
For now, compiling a program requires these steps, first generate the object files

`ty obj main.ty,other.ty`

this puts the object files in a directory `.ty`

then link to a final executable.

`ty link my-cli -o .ty/main.o,.ty/other.o`

you can then execute your new binary.

`./my-cli` or `my-cli.exe` on windows

#### Windows

Windows additionally requires a linker, as it is not on every machine. After installing Visual Studio Build tools, you can select the latest c++ build tools.

From there, ensure that you open the developer console for windows where you want to build your application, this way the linker necessary will automatically be included in your path.
