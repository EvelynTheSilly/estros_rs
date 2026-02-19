qemuflags := "-M virt -cpu cortex-a57 -nographic -kernel ./build/kernel.elf -semihosting"

build_kernel_elf opt="debug":
    mkdir -p ./build
    rm -r ./build/*
    
    if [ -f "config.sh" ]; then
    source ./config.sh
    else
    source ./exampleconfig.sh
    fi
    
    cargo build --bin kernel $( [ {{ opt }} = release ] && printf '%s' --release )
    cp ./target/aarch64-none-custom/debug/kernel ./build/kernel.elf

build_img:
    # 1. Create a blank 64MB image file
    dd if=/dev/zero of=build/disk.img bs=1M count=64
    
    # 2. Format it as FAT32
    mkfs.vfat -F 32 build/disk.img
    
    # 3. Create the standard UEFI directory structure
    mmd -i build/disk.img ::/EFI
    mmd -i build/disk.img ::/EFI/BOOT
    
    # 4. Copy the Limine UEFI binary (Rename it to the default boot name)
    mcopy -i build/disk.img limine/BOOTAA64.EFI ::/EFI/BOOT/BOOTAA64.EFI
    
    # 5. Copy your kernel and config to the root
    mcopy -i build/disk.img build/kernel.elf ::/kernel.elf
    mcopy -i build/disk.img limine.conf ::/limine.conf
   
build_init opt="debug" init="minimal_init":
    cargo build --bin {{ init }} $( [ {{ opt }} = release ] && printf '%s' --release )
    cp ./target/aarch64-none-custom/{{opt}}/{{init}} ./build/init.elf
    
build:
    just build_init
    just build_kernel_elf
    just build_img
    
 
run:
    @echo "running vm"
    @echo "exit with ctrl a, then x"
    @echo ""
    @echo ""
    qemu-system-aarch64 {{qemuflags}}

buildrun:
    just build
    just run

debug:
    just build
    @echo "running vm"
    @echo "exit with ctrl a, then x"
    @echo "run mask start_gdb to attatch to the debugger"
    @echo ""
    @echo ""
    qemu-system-aarch64 {{qemuflags}} -S -s

gdb:
    aarch64-none-elf-gdb -ex "target remote :1234" -ex "symbol-file build/kernel.elf"