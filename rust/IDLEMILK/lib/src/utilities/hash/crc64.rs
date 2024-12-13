use core::{cell::UnsafeCell, sync::atomic::{AtomicBool, Ordering}};

const CRC64_ECMA_182: u64 = 0x42F0E1EBA9EA3693;

static CRC64_LOOKUP_TABLE: CRC64LookupTable = CRC64LookupTable::new();
static CRC64_INITIALIZED: AtomicBool = AtomicBool::new(false);

///
///
///
struct CRC64LookupTable(UnsafeCell<[u64; 256]>);

impl CRC64LookupTable {
    const fn new() -> Self {
        Self(UnsafeCell::new([0_u64; 256]))
    }

    fn get(&self) -> *mut [u64; 256] {
        self.0.get()
    }
}

unsafe impl Sync for CRC64LookupTable {}

///
///
///
pub trait CRC64String {
    fn to_bytes(&self) -> &[u8];
}

impl CRC64String for [u8] {
    fn to_bytes(&self) -> &[u8] {
        self
    }
}

impl CRC64String for [u16] {
    fn to_bytes(&self) -> &[u8] {
        unsafe {
            core::slice::from_raw_parts(
                self.as_ptr() as *const u8,
                self.len().wrapping_mul(2)
            )
        }
    }
}

///
///
///
pub struct CRC64;

impl CRC64 {
    pub fn new<T>(src: &T) -> u64
    where
        T: CRC64String + ?Sized
    {
        let mut crc = 0_u64;
        let bytes = src.to_bytes();

        if !CRC64_INITIALIZED.load(Ordering::Acquire) {
            Self::initialize();
        }

        let table = unsafe { &mut * CRC64_LOOKUP_TABLE.get() };

        for &byte in bytes {
            crc = table[((crc >> 56) ^ (byte as u64)) as usize] ^ (crc << 8);
        }

        crc
    }

    fn initialize() {
        let table = unsafe { &mut *CRC64_LOOKUP_TABLE.get() };

        for i in 0..256 {
            let mut crc = 0_u64;
            let mut mask = (i as u64) << 56;

            for _ in 0..8 {
                if (crc ^ mask) & 0x8000000000000000 != 0 {
                    crc = (crc << 1) ^ CRC64_ECMA_182;
                } else {
                    crc <<= 1;
                }
                mask <<= 1;
            }

            table[i] = crc;
        }
    }
}
