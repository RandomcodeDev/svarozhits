#[repr(u8)]
#[derive(Clone, Debug, Default)]
pub enum PCIUnclassifiedSubclass {
    #[default]
    NonVGACompatible = 0x0,
    VGACompatible = 0x1,
}

#[repr(u8)]
#[derive(Clone, Debug, Default)]
pub enum PCIMassStorageSubclass {
    SCSIBus = 0x0,
    IDE = 0x1,
    FloppyDisk = 0x2,
    IPIBus = 0x3,
    RAID = 0x4,
    ATA = 0x5,
    SATA = 0x6,
    SAS = 0x7,
    NVM = 0x8,
    #[default]
    Other = 0x80,
}

#[repr(u8)]
#[derive(Clone, Debug, Default)]
pub enum PCINetworkSubclass {
    Ethernet = 0x0,
    TokenRing = 0x1,
    FDDI = 0x2,
    ATM = 0x3,
    ISDN = 0x4,
    WorldFip = 0x5,
    PICMG214MC = 0x6,
    InfiniBand = 0x7,
    Fabric = 0x8,
    #[default]
    Other = 0x80,
}

#[repr(u8)]
#[derive(Clone, Debug, Default)]
pub enum PCIDisplaySubclass {
    VGACompatible = 0x0,
    XGA = 0x1,
    VGANonCompat3D = 0x2,
    #[default]
    Other = 0x80,
}

#[repr(u8)]
#[derive(Clone, Debug, Default)]
pub enum PCIMultimediaSubclass {
    Video = 0x0,
    AudioController = 0x1,
    ComputerTelephony = 0x2,
    AudioDevice = 0x3,
    #[default]
    Other = 0x80,
}

#[repr(u8)]
#[derive(Clone, Debug, Default)]
pub enum PCIMemorySubclass {
    RAM = 0x0,
    Flash = 0x1,
    #[default]
    Other = 0x80,
}

#[repr(u8)]
#[derive(Clone, Debug, Default)]
pub enum PCIBridgeSubclass {
    Host = 0x0,
    ISA = 0x1,
    EISA = 0x2,
    MCA = 0x3,
    PCI2PCI1 = 0x4,
    PCMCIA = 0x5,
    NuBus = 0x6,
    CardBus = 0x7,
    RACEway = 0x8,
    PCI2PCI2 = 0x9,
    InfiniBand2PCI = 0xa,
    #[default]
    Other = 0x80,
}

#[repr(u8)]
#[derive(Clone, Debug, Default)]
pub enum PCISimpleCommunicationSubclass {
    Serial = 0x0,
    Parallel = 0x1,
    MultiportSerial = 0x2,
    Modem = 0x3,
    GBIP = 0x4,
    SmartCard = 0x5,
    #[default]
    Other = 0x80,
}

#[repr(u8)]
#[derive(Clone, Debug, Default)]
pub enum PCIBaseSystemPeripheralSubclass {
    PIC = 0x0,
    DMA = 0x1,
    Timer = 0x2,
    RTC = 0x3,
    PCIHotPlug = 0x4,
    SDHost = 0x5,
    IOMMU = 0x6,
    #[default]
    Other = 0x80,
}

#[repr(u8)]
#[derive(Clone, Debug, Default)]
pub enum PCIInputDeviceSubclass {
    Keyboard = 0x0,
    DigitizerPen = 0x1,
    Mouse = 0x2,
    Scanner = 0x3,
    Gameport = 0x4,
    #[default]
    Other = 0x80,
}

#[repr(u8)]
#[derive(Clone, Debug, Default)]
pub enum PCIDockingStationSubClass {
    Generic = 0x0,
    #[default]
    Other = 0x80,
}

#[repr(u8)]
#[derive(Clone, Debug, Default)]
pub enum PCIProcessorSubclass {
    I386 = 0x0,
    I486 = 0x1,
    Pentium = 0x2,
    PentiumPro = 0x3,
    Alpha = 0x10,
    PowerPC = 0x20,
    MIPS = 0x30,
    Coprocessor = 0x40,
    #[default]
    Other = 0x80,
}

#[repr(u8)]
#[derive(Clone, Debug, Default)]
pub enum PCISerialBusSubclass {
    FireWire = 0x0,
    ACCESSBus = 0x1,
    SSA = 0x2,
    USB = 0x3,
    FibreChannel = 0x4,
    SMBus = 0x5,
    InfiniBand = 0x6,
    IPMI = 0x7,
    SERCOS = 0x8,
    CANbus = 0x9,
    #[default]
    Other = 0x80
}

#[repr(u8)]
#[derive(Clone, Debug, Default)]
pub enum PCIWirelessSubclass {
    #[allow(non_camel_case_types)]
    iRDA = 0x0,
    ConsumerIR = 0x1,
    RF = 0x2,
    Bluetooth = 0x3,
    Broadband = 0x4,
    Ethernet8021a = 0x5,
    Ethernet8021b = 0x6,
    #[default]
    Other = 0x80
}

#[repr(u8)]
#[derive(Clone, Debug, Default)]
pub enum PCIIntelligentSubclass {
    #[default]
    I20 = 0x0
}

#[repr(u8)]
#[derive(Clone, Debug, Default)]
pub enum PCISatelliteCommunicationSubclass {
    #[default]
    TV = 0x0,
    Audio = 0x1,
    Voice = 0x2,
    Data = 0x3
}

#[repr(u8)]
#[derive(Clone, Debug, Default)]
pub enum PCIEncryptionSubclass {
    NetworkAndComputing = 0x0,
    Entertainment = 0x1, // ew drm
    #[default]
    Other = 0x80
}

#[repr(u8)]
#[derive(Clone, Debug, Default)]
pub enum PCISignalProcessingSubclass {
    DPIO = 0x0,
    PerformanceCounters = 0x1,
    CommunicationSynchronizer = 0x2,
    SignalProcessingManagement = 0x3,
    #[default]
    Other = 0x80
}
