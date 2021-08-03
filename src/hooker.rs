macro_rules! hook {
    (unsafe fn $real_fn:ident ( $($v:ident : $t:ty),* ) -> $r:ty => $hook_fn:ident $body:block) => {
        #[allow(non_camel_case_types)]
        pub struct $real_fn {__private_field: ()}

        #[allow(non_upper_case_globals)]
        static $real_fn: $real_fn = $real_fn {__private_field: ()};

        impl $real_fn {
            fn get(&self) -> unsafe extern fn ( $($v : $t),* ) -> $r {
                static mut REAL: ::once_cell::unsync::Lazy<*const u8> = once_cell::unsync::Lazy::new(|| unsafe {
                    $crate::hooker::dlsym_next(concat!(stringify!($real_fn), "\0"))
                });

                unsafe { ::std::mem::transmute(*REAL) }
            }

            #[allow(clippy::missing_safety_doc)]
            #[no_mangle]
            pub unsafe extern fn $real_fn ( $($v : $t),* ) -> $r {
                ::std::panic::catch_unwind(|| $hook_fn ( $($v),* )).unwrap_or_else(|_| $real_fn.get() ( $($v),* ))
            }
        }

        #[allow(clippy::missing_safety_doc)]
        pub unsafe fn $hook_fn ( $($v : $t),* ) -> $r {
            $body
        }
    };

    (unsafe fn $real_fn:ident ( $($v:ident : $t:ty),* ) => $hook_fn:ident $body:block) => {
        $crate::hook! { unsafe fn $real_fn ( $($v : $t),* ) -> () => $hook_fn $body }
    };
}

#[macro_export]
macro_rules! real {
    ($real_fn:ident) => {
        $real_fn.get()
    };
}


pub unsafe fn dlsym_next(symbol: &'static str) -> *const u8 {
    let ptr = ::libc::dlsym(::libc::RTLD_NEXT, symbol.as_ptr() as *const ::libc::c_char);
    if ptr.is_null() {
        panic!("Unable to find underlying function for {}", symbol);
    }
    ptr as *const u8
}

