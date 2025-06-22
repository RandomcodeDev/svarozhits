# Сварожиц RISC-V game console

I've been getting into game consoles lately, so I thought it might be fun to try and make a really simple one myself.

## Hardware

I'm planning to use a cheap RISC-V board and a salvaged GPU chip (possibly the Xbox 360's Xenos GPU), which should be
somehwere between the original Xbox and the 360 in terms of hardware.

## Software

I'm also going to write an OS in Rust. There won't be any separation between kernel and user space, much like older
consoles, because context switches are expensive and can be avoided when there's only ever one program. Or I might
try making a hypervisor and making it super separated like modern Xboxes, not sure. It's going to support some kind
of rendering API, probably more similar to modern APIs like Vulkan/D3D12, because they're closer to the hardware.
It'll also support encryption and other security measures that I can figure out how to implement.

Games will be in some kind of package format, probably compressed and encrypted, and integrated into the VFS.

## Testing

You can run it in QEMU:

```shell
cd system
cargo build
qemu-system-riscv64 -machine virt -m 64M -kernel target/riscv64gc-unknown-none-elf/debug/svboot.elf -serial file:svaro.log
```
