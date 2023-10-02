# type-lang

Type-lang or __ty__. Is a rust/zig/typescript inspired language.

#### Build from source

```
cd ~
git clone https://github.com/coffeebe4code/type-lang
cd type-lang
cargo build --release
```

optionally, you can run the tests with

```
cargo test
```

add `ty` to your path in your shell of choice.

User Environment Variables if you are on windows.

```
export PATH=$HOME/type-lang/target/release:$PATH
```

right now, the only thing that builds or works is a main function, with simple addition or subtraction. setting variables, and using those variables work. There is no type system yet.

location of test file.

[main.ty](./test/main.ty)

to compile a file. first generate the object files

`ty obj main.ty,other.ty`

this puts the object files in a directory `.ty-cache`

then link to a final executable.

`ty link my-cli -o .ty-cache/main.o,.ty-cache/other.o`

you can then execute your new binary.

`./my-cli` or `my-cli.exe` on windows

#### Windows

windows additionally requires a linker, as it is not on every machine. After installing Visual Studio Build tools, you can select the latest c++ build tools.

From there ensure that you open the developer console for windows where you want to build your application, this way every peice necessary will automatically be included in your path.
