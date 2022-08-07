use core::alloc::{GlobalAlloc, Layout};

pub struct NoobAlloc {
    pub ptr: usize,
}

unsafe impl GlobalAlloc for NoobAlloc {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let alloc: *const NoobAlloc = self;
        let alloc = alloc as *mut NoobAlloc;
        let ptr = (*alloc).ptr as *mut u8;
        (*alloc).ptr += layout.size();
        ptr
    }

    #[inline]
    unsafe fn dealloc(&self, _: *mut u8, _: Layout) {
        // freeing is for pussies
    }
}

#[macro_export]
macro_rules! entrypoint {
    ($main:ident) => {
        #[no_mangle]
        pub unsafe extern "C" fn entrypoint(input: *mut u8) -> u64 {
            let mut parser = solana_cum::input::InputParser {
                ptr: input as usize,
            };
            $main(&mut parser)
        }

        #[global_allocator]
        static A: $crate::entrypoint::NoobAlloc = $crate::entrypoint::NoobAlloc {
            ptr: 0x300000000usize,
        };
    };
}
