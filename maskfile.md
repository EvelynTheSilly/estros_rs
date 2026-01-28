# MaskFile

## build
> Builds with default optimisations optimisations

~~~bash
echo "building project..."
set -e
shopt -s globstar
mkdir -p ./build

mask build_debug
~~~

## build_release

> Builds with release optimisations

~~~bash
cargo build --release

cp ./target/aarch64-unknown-none/release/kernel ./build/kernel.elf
~~~


## build_debug

> Builds without optimisations

~~~bash
cargo build

cp ./target/aarch64-unknown-none/debug/kernel build/kernel.elf
~~~

## run

> runs the project in qemu

~~~sh
echo "running vm"
echo "exit with ctrl a, then x"
echo ""
echo ""
qemu-system-aarch64  -M virt -cpu cortex-a57 -nographic -kernel ./build/kernel.elf
~~~

## buildrun

> build and run

~~~sh
mask build
mask run
~~~

## debug

> run with debugger

~~~sh
mask build
echo "running vm"
echo "exit with ctrl a, then x"
echo "run mask start_gdb to attatch to the debugger"
echo ""
echo ""
qemu-system-aarch64 -M virt -cpu cortex-a57 -nographic -kernel ./build/kernel.elf -S -s
~~~

## start_gdb

> attatch with debugger

~~~sh
aarch64-none-elf-gdb -ex "target remote :1234" -ex "symbol-file build/kernel.elf"
~~~
