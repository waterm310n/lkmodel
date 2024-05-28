use aarch64_cpu::registers::MAIR_EL1;

#[repr(usize)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum MemAttr {
    /// Device-nGnRE memory
    Device = 0,
    /// Normal memory
    Normal = 1,
    /// Normal non-cacheable memory
    NormalNonCacheable = 2,
}

impl MemAttr {
    /// The MAIR_ELx register should be set to this value to match the memory
    /// attributes in the descriptors.
    pub const MAIR_VALUE: usize = {
        // Device-nGnRE memory
        let attr0 = MAIR_EL1::Attr0_Device::nonGathering_nonReordering_EarlyWriteAck.value;
        // Normal memory
        let attr1 = MAIR_EL1::Attr1_Normal_Inner::WriteBack_NonTransient_ReadWriteAlloc.value
            | MAIR_EL1::Attr1_Normal_Outer::WriteBack_NonTransient_ReadWriteAlloc.value;
        let attr2 = MAIR_EL1::Attr2_Normal_Inner::NonCacheable.value
            + MAIR_EL1::Attr2_Normal_Outer::NonCacheable.value;
        (attr0 | attr1 | attr2) as usize // 0x44_ff_04
    };
}

bitflags::bitflags! {
    /// Generic page table entry flags that indicate the corresponding mapped
    /// memory region permissions and attributes.
    #[derive(Debug, Clone, Copy)]
    pub struct MappingFlags: usize {
        /// The memory is readable.
        const READ          = 1 << 0;
        /// The memory is writable.
        const WRITE         = 1 << 1;
        /// The memory is executable.
        const EXECUTE       = 1 << 2;
        /// The memory is user accessible.
        const USER          = 1 << 3;
        /// The memory is device memory.
        const DEVICE        = 1 << 4;
        /// The memory is uncached.
        const UNCACHED      = 1 << 5;
    }
}

bitflags::bitflags! {
    /// Memory attribute fields in the VMSAv8-64 translation table format descriptors.
    #[derive(Debug)]
    pub struct DescriptorAttr: usize {
        // Attribute fields in stage 1 VMSAv8-64 Block and Page descriptors:

        /// Whether the descriptor is valid.
        const VALID =       1 << 0;
        /// The descriptor gives the address of the next level of translation table or 4KB page.
        /// (not a 2M, 1G block)
        const NON_BLOCK =   1 << 1;
        /// Memory attributes index field.
        const ATTR_INDX =   0b111 << 2;
        /// Non-secure bit. For memory accesses from Secure state, specifies whether the output
        /// address is in Secure or Non-secure memory.
        const NS =          1 << 5;
        /// Access permission: accessable at EL0.
        const AP_EL0 =      1 << 6;
        /// Access permission: read-only.
        const AP_RO =       1 << 7;
        /// Shareability: Inner Shareable (otherwise Outer Shareable).
        const INNER =       1 << 8;
        /// Shareability: Inner or Outer Shareable (otherwise Non-shareable).
        const SHAREABLE =   1 << 9;
        /// The Access flag.
        const AF =          1 << 10;
        /// The not global bit.
        const NG =          1 << 11;
        /// Indicates that 16 adjacent translation table entries point to contiguous memory regions.
        const CONTIGUOUS =  1 <<  52;
        /// The Privileged execute-never field.
        const PXN =         1 <<  53;
        /// The Execute-never or Unprivileged execute-never field.
        const UXN =         1 <<  54;

        // Next-level attributes in stage 1 VMSAv8-64 Table descriptors:

        /// PXN limit for subsequent levels of lookup.
        const PXN_TABLE =           1 << 59;
        /// XN limit for subsequent levels of lookup.
        const XN_TABLE =            1 << 60;
        /// Access permissions limit for subsequent levels of lookup: access at EL0 not permitted.
        const AP_NO_EL0_TABLE =     1 << 61;
        /// Access permissions limit for subsequent levels of lookup: write access not permitted.
        const AP_NO_WRITE_TABLE =   1 << 62;
        /// For memory accesses from Secure state, specifies the Security state for subsequent
        /// levels of lookup.
        const NS_TABLE =            1 << 63;
    }
}

impl DescriptorAttr {
    pub const fn from_mem_attr(idx: MemAttr) -> Self {
        let mut bits = (idx as usize) << 2;
        if matches!(idx, MemAttr::Normal | MemAttr::NormalNonCacheable) {
            bits |= Self::INNER.bits() | Self::SHAREABLE.bits();
        }
        Self::from_bits_retain(bits)
    }
}

impl From<MappingFlags> for DescriptorAttr {
    fn from(flags: MappingFlags) -> Self {
        let mut attr = if flags.contains(MappingFlags::DEVICE) {
            Self::from_mem_attr(MemAttr::Device)
        } else if flags.contains(MappingFlags::UNCACHED) {
            Self::from_mem_attr(MemAttr::NormalNonCacheable)
        } else {
            Self::from_mem_attr(MemAttr::Normal)
        };
        if flags.contains(MappingFlags::READ) {
            attr |= Self::VALID;
        }
        if !flags.contains(MappingFlags::WRITE) {
            attr |= Self::AP_RO;
        }
        if flags.contains(MappingFlags::USER) {
            attr |= Self::AP_EL0 | Self::PXN;
            if !flags.contains(MappingFlags::EXECUTE) {
                attr |= Self::UXN;
            }
        } else {
            attr |= Self::UXN;
            if !flags.contains(MappingFlags::EXECUTE) {
                attr |= Self::PXN;
            }
        }
        attr
    }
}
