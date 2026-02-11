# MaskFile

## build
> Builds with default optimisations optimisations

~~~bash
rm build -r
echo "building project..."
set -e
shopt -s globstar
mkdir -p ./build

if [ -f "config.sh" ]; then
    source ./config.sh
else
    source ./exampleconfig.sh
fi

cargo build --bin $INIT -Z json-target-spec $( [ "$OPT" = release ] && printf '%s' --release )
cp ./target/aarch64-none-custom/$OPT/$INIT ./build/init.elf

mask build_kernel_debug
#aarch64-none-elf-objcopy -O binary ./build/kernel.elf ./build/kernel.elf
~~~

## build_kernel_release

> Builds with release optimisations

~~~bash
set -e
cargo build --bin kernel --release -Z json-target-spec

cp ./target/aarch64-none-custom/release/kernel ./build/kernel.elf
~~~


## build_kernel_debug

> Builds without optimisations

~~~bash
set -e
cargo build --bin kernel -Z json-target-spec

cp ./target/aarch64-none-custom/debug/kernel build/kernel.elf
~~~

## run

> runs the project in qemu

~~~sh
echo "running vm"
echo "exit with ctrl a, then x"
echo ""
echo ""
qemu-system-aarch64  -M virt -cpu cortex-a57 -nographic -kernel ./build/kernel.elf -semihosting
~~~

## buildrun

> build and run

~~~sh
set -e
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
qemu-system-aarch64 -M virt -cpu cortex-a57 -nographic -kernel ./build/kernel.elf -S -s -semihosting
~~~

## start_gdb

> attatch with debugger

~~~sh
aarch64-none-elf-gdb -ex "target remote :1234" -ex "symbol-file build/kernel.elf"
~~~
