# Сварожиц RISC-V game console

I've been getting into game consoles lately, so I thought it might be fun to try and make a really simple one myself.

## Hardware

I'm planning to use a cheap RISC-V board and a salvaged GPU chip (possibly the Xbox 360's Xenos GPU), which should be
somehwere between the original Xbox and the 360 in terms of hardware.

## Software

I'm also going to write an OS in Rust. I'm making a custom executable format for it. There won't be any separation
between kernel and user space, much like older consoles, because context switches are expensive and can be avoided
when there's only ever one program. It's going to support some kind of rendering API, probably more similar to modern
APIs like Vulkan/D3D12, because they're closer to the hardware. It should support encryption and other security
measures as well.

Games will be in some kind of package format
