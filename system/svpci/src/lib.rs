#![no_std]

use svcommon::uptr;

pub mod subclass;

#[derive(Clone, Debug, Default)]
pub struct PCIDevice {
    _header: PCIStandardHeader,
}

#[derive(Clone, Debug)]
pub enum PCIHeader {
    Standard(PCIStandardHeader),
    //Bridge(PCIBridgeHeader)
}

#[repr(u8)]
#[derive(Clone, Debug, Default)]
pub enum PCIHeaderType {
    #[default]
    Standard = 0x0,
    Bridge = 0x1,
}

#[derive(Clone, Debug, Default)]
pub struct PCIStandardHeader {
    pub vendor_id: u16,
    pub device_id: u16,
    pub command: u16,
    pub status: u16,
    pub revision_id: u8,
    pub programming_interface: u8,
    pub subclass: u8, // PCI*Subclass
    pub class: PCIClass,
    pub cache_line_size: u8,
    pub latency_timer: u8,
    pub header_type: u8, // PCIHeaderType
    pub bist: u8,
    pub bar0: u32,
    pub bar1: u32,
    pub bar2: u32,
    pub bar3: u32,
    pub bar4: u32,
    pub bar5: u32,
    pub cardbus_ptr: u32,
    pub subsystem_vendor_id: u16,
    pub subsystem_id: u16,
    pub erom_base_addr: u32,
    pub capabilities_ptr: u8,
    pub reserved1: u8,
    pub reserved2: u16,
    pub reserved3: u32,
    pub interrupt_line: u8,
    pub interrupt_pin: u8,
    pub min_grant: u8,
    pub max_grant: u8,
}

// The word "controller" has been omitted from most of these names

#[repr(u8)]
#[derive(Clone, Debug, Default)]
pub enum PCIClass {
    #[default]
    Unclassified = 0x0,
    MassStorage = 0x1,
    Network = 0x2,
    Display = 0x3,
    Multimedia = 0x4,
    Memory = 0x5,
    Bridge = 0x6,
    SimpleCommunication = 0x7,
    BaseSystemPeripheral = 0x8,
    InputDevice = 0x9,
    DockingStation = 0xa,
    Processor = 0xb,
    SerialBus = 0xc,
    Wireless = 0xd,
    Intelligent = 0xe,
    SatelliteCommunication = 0xf,
    Encryption = 0x10,
    SignalProcessing = 0x11,
    ProcessingAccelerator = 0x12,
    NonEssentialInstrumentation = 0x13,
    Coprocessor = 0x40,
    VendorSpecific = 0xff,
}

pub const fn get_device_space(pcie_base: uptr, bus: u8, device: u8, function: u8) -> uptr {
    pcie_base + ((bus as usize * 256) + (device as usize * 8) + function as usize) * 4096
}

// TODO: proper device wrapper struct
pub fn check_device(pcie_base: uptr, bus: u8, device: u8) -> Option<&'static PCIStandardHeader> {
    let function = 0;

    let device_base = get_device_space(pcie_base, bus, device, function);
    let device_header: &'static PCIStandardHeader = unsafe { &*(device_base as *const PCIStandardHeader) };
    if device_header.vendor_id == 0xFFFF {
        return None;
    }

    Some(device_header)
}
