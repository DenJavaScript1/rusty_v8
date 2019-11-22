use std::marker::PhantomData;
use std::mem::MaybeUninit;

use crate::isolate::CxxIsolate;
use crate::isolate::Isolate;
use crate::isolate::LockedIsolate;

// class Locker {
//  public:
//    explicit Locker(Isolate* isolate);
//    ~Locker();
//    static bool IsLocked(Isolate* isolate);
//    static bool IsActive();
// }

extern "C" {
  fn v8__Locker__CONSTRUCT(buf: &mut MaybeUninit<Locker>, isolate: &CxxIsolate);
  fn v8__Locker__DESTRUCT(this: &mut Locker);
}

#[repr(C)]
pub struct Locker<'a> {
  has_lock: bool,
  top_level: bool,
  isolate: &'a mut CxxIsolate,
  phantom: PhantomData<&'a Isolate>,
}

impl<'a> Locker<'a> {
  pub fn new(isolate: &CxxIsolate) -> Self {
    let mut buf = MaybeUninit::<Self>::uninit();
    unsafe {
      v8__Locker__CONSTRUCT(&mut buf, isolate);
      buf.assume_init()
    }
  }
}

impl<'a> Drop for Locker<'a> {
  fn drop(&mut self) {
    unsafe { v8__Locker__DESTRUCT(self) }
  }
}

impl<'a> LockedIsolate for Locker<'a> {
  fn cxx_isolate(&mut self) -> &mut CxxIsolate {
    self.isolate
  }
}
