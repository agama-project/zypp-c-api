use tokio::sync::{Mutex, MutexGuard, TryLockError};



// Mutex for the Zypp C API which is not thread safe.
// TODO: guard some (global) context instead of ().
//static ZYPP_MUTEX: Mutex<*mut zypp_agama_sys::Zypp> = Mutex::new(std::ptr::null_mut());
static ZYPP_MUTEX: Mutex<()> = Mutex::const_new(());
pub struct Zypp<'a> {
    // underscore prevents
    //     warning: field `guard` is never read
    _guard: MutexGuard<'a, ()>,
    inner: *mut zypp_agama_sys::Zypp,
}

impl<'a> Zypp<'a> {
    // Create Self, locking the Mutex (delaying and retrying a few times before panicking).
    //
    // Usage:
    // ```
    // let mut zypp = Zypp::lock();
    // let inner_zypp = zypp_agama_sys::init_target(...);
    // zypp.set(inner_zypp);
    // ```
    //
    // Using `lock()`+`set(inner)` instead of `new(inner)` lets us
    // avoid locks on the C side
    pub fn lock() -> Self {
        let mut delay = std::time::Duration::from_millis(10);
        let mut tries = 8;
        // Exponential backoff, will wait at most (2**(tries-1) - 1) * delay
        // Which is 1.27s for now. Increase *tries* if needed.
        loop {
            let result = ZYPP_MUTEX.try_lock();
            if let Ok(guard) = result {
                // println!("creating Zypp");
                return Self {
                    _guard: guard,
                    inner: std::ptr::null_mut(),
                };
            }

            match result.unwrap_err() {
                TryLockError => {
                    panic!("Another thread had the ZYPP_MUTEX, and panicked.")
                }
            }

            // println!("delaying {:?}", delay);
            std::thread::sleep(delay);
            delay = delay.mul_f64(2.0);
        }
    }

    pub fn set(&mut self, inner: *mut zypp_agama_sys::Zypp) {
        self.inner = inner;
    }

    pub fn inner(&self) -> *mut zypp_agama_sys::Zypp {
        assert!(!self.inner.is_null());
        self.inner
    }
}

impl Drop for Zypp<'_> {
    fn drop(&mut self) {
        // println!("dropping Zypp");
        unsafe {
            if !self.inner.is_null() {
                zypp_agama_sys::free_zypp(self.inner);
            }
        }
    }
}
