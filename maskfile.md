# MaskFile


## build

> Builds my project

~~~bash
mask clean
echo "building project..."
set -e
shopt -s globstar

# Define global paths
export SRC_ROOT=$(pwd)
export BUILD_DIR=$SRC_ROOT/build
export OUT_DIR=$BUILD_DIR/out
export BYPRODUCTS_DIR=$BUILD_DIR/byproducts


# 1. Load Configuration 
if [ -f "config.sh" ]; then
    source ./config.sh
else
    echo "Error: config.sh not found!" && exit 1
fi 

build_dir() {
    local dir=$1
    local rel_path=${dir#$SRC_ROOT/}
    
    # Define unique paths for this component (The "Nix" style)
    export OUT="$OUT_DIR/" #$rel_path"
    export BYPRODUCTS="$BYPRODUCTS_DIR/$rel_path"
    
    mkdir -p "$OUT" "$BYPRODUCTS"
    
    # Run the local build script
    echo "building $dir"
    (cd "$dir" && ./build.sh)
    echo "built $dir"
}

mkdir -p "$BUILD_DIR" "$OUT_DIR" "$BYPRODUCTS_DIR"

build_dir src/kernel
build_dir $INIT
build_dir src/lib/LibEstrogen

echo "out: $OUT_DIR"
echo "out: $BUILD_DIR"
echo "out: $BYPRODUCTS_DIR"
ls -l "$OUT_DIR/kernel.o"
CROSS=aarch64-none-elf

# Link everything
echo "link"
$CROSS-ld -T src/linker.ld ${OUT_DIR#$SRC_ROOT/}/*.o -o $BUILD_DIR/kernel.elf

# Optional: make binary
echo "objcopy"
$CROSS-objcopy -O binary $BUILD_DIR/kernel.elf $BUILD_DIR/kernel.bin
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

## clean

> removes build artifacts from the directory

~~~sh
echo "cleaning"
rm -fr build
~~~

## cbr

> clean, build, run

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
