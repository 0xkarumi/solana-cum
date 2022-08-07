use crate::pubkey::Pubkey;
use core::marker::PhantomData;

// size: 0x57
#[repr(packed)]
pub struct AccountInput {
    pub is_signer: bool,     // 0x00
    pub is_writable: bool,   // 0x01
    pub is_executable: bool, // 0x02
    pub _padding0: [u8; 4],  // 0x03
    pub pubkey: Pubkey,      // 0x07
    pub owner: Pubkey,       // 0x27
    pub lamports: u64,       // 0x47
    pub size: usize,         // 0x4f

    _marker: PhantomData<()>, // prevent noobs from copying
}

impl AccountInput {
    pub unsafe fn data(&mut self) -> *mut u8 {
        (self as *mut AccountInput as *mut u8).add(0x57)
    }

    pub unsafe fn data_slice(&mut self) -> &mut [u8] {
        core::slice::from_raw_parts_mut(self.data(), self.size)
    }
}

pub struct InputParser {
    pub ptr: usize,
}

impl InputParser {
    /// Returns the number of accounts provided.
    ///
    /// Must be the first call to `InputParser`. Only call once.
    #[inline]
    pub unsafe fn num_accounts(&mut self) -> u64 {
        let ret = *(self.ptr as *const u64);
        self.ptr += 8;
        ret
    }

    /// Returns the next account.
    /// Consumes 12 CU.
    ///
    /// Must be called exactly as many times as indicated by `num_accounts`.
    pub unsafe fn next_account(&mut self) -> Option<&'static mut AccountInput> {
        let is_dup = *(self.ptr as *mut u8);
        if is_dup != 0xFF {
            self.ptr += 8;
            return None;
        }
        let info = (self.ptr + 1) as *mut AccountInput;
        self.ptr += 0x58 + (10 * 1024) + (*info).size;
        self.ptr = (self.ptr + 7) & !7usize;
        info.as_mut()
    }

    /// Returns the account's rent epoch.
    ///
    /// Only valid immediately after a call to `next_account`.
    #[inline]
    pub unsafe fn rent_epoch(&self) -> u64 {
        *((self.ptr - 8) as *const u64)
    }
}
