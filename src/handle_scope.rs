use std::mem::MaybeUninit;

use crate::isolate::Isolate;
use crate::scope::Scope;
use crate::scope::Scoped;
use crate::InIsolate;
use crate::Local;
use crate::Value;

extern "C" {
  fn v8__HandleScope__CONSTRUCT(
    buf: &mut MaybeUninit<HandleScope>,
    isolate: &Isolate,
  );
  fn v8__HandleScope__DESTRUCT(this: &mut HandleScope);
  fn v8__HandleScope__GetIsolate<'sc>(
    this: &'sc HandleScope,
  ) -> &'sc mut Isolate;

  fn v8__EscapableHandleScope__CONSTRUCT(
    buf: &mut MaybeUninit<EscapableHandleScope>,
    isolate: &Isolate,
  );
  fn v8__EscapableHandleScope__DESTRUCT(this: &mut EscapableHandleScope);
  fn v8__EscapableHandleScope__Escape(
    this: &mut EscapableHandleScope,
    value: *mut Value,
  ) -> *mut Value;
  fn v8__EscapableHandleScope__GetIsolate<'sc>(
    this: &'sc EscapableHandleScope,
  ) -> &'sc mut Isolate;
}

#[repr(C)]
pub struct HandleScope([usize; 3]);

impl HandleScope {
  pub fn new(scope: &mut impl InIsolate) -> Scope<Self> {
    Scope::new(scope.isolate())
  }
}

unsafe impl<'s> Scoped<'s> for HandleScope {
  type Args = &'s mut Isolate;

  fn enter_scope(buf: &mut MaybeUninit<Self>, isolate: &mut Isolate) {
    unsafe { v8__HandleScope__CONSTRUCT(buf, isolate) };
  }
}

impl Drop for HandleScope {
  fn drop(&mut self) {
    unsafe { v8__HandleScope__DESTRUCT(self) }
  }
}

impl AsRef<HandleScope> for HandleScope {
  fn as_ref(&self) -> &Self {
    self
  }
}

impl AsMut<HandleScope> for HandleScope {
  fn as_mut(&mut self) -> &mut Self {
    self
  }
}

impl AsRef<Isolate> for HandleScope {
  fn as_ref(&self) -> &Isolate {
    unsafe { v8__HandleScope__GetIsolate(self) }
  }
}

impl AsMut<Isolate> for HandleScope {
  fn as_mut(&mut self) -> &mut Isolate {
    unsafe { v8__HandleScope__GetIsolate(self) }
  }
}

/// A HandleScope which first allocates a handle in the current scope
/// which will be later filled with the escape value.
#[repr(C)]
pub struct EscapableHandleScope([usize; 4]);

impl EscapableHandleScope {
  pub fn new(scope: &mut impl InIsolate) -> Scope<Self> {
    Scope::new(scope.isolate())
  }

  /// Pushes the value into the previous scope and returns a handle to it.
  /// Cannot be called twice.
  pub fn escape<'parent, T>(&mut self, value: Local<T>) -> Local<'parent, T> {
    unsafe {
      Local::from_raw_(v8__EscapableHandleScope__Escape(
        self,
        value.as_ptr() as *mut Value,
      ) as *mut T)
    }
    .unwrap()
  }
}

unsafe impl<'s> Scoped<'s> for EscapableHandleScope {
  type Args = &'s mut Isolate;

  fn enter_scope(buf: &mut MaybeUninit<Self>, isolate: &mut Isolate) {
    unsafe { v8__EscapableHandleScope__CONSTRUCT(buf, isolate) };
  }
}

impl Drop for EscapableHandleScope {
  fn drop(&mut self) {
    unsafe { v8__EscapableHandleScope__DESTRUCT(self) }
  }
}

impl AsRef<EscapableHandleScope> for EscapableHandleScope {
  fn as_ref(&self) -> &Self {
    self
  }
}

impl AsMut<EscapableHandleScope> for EscapableHandleScope {
  fn as_mut(&mut self) -> &mut Self {
    self
  }
}

impl AsRef<Isolate> for EscapableHandleScope {
  fn as_ref(&self) -> &Isolate {
    unsafe { v8__EscapableHandleScope__GetIsolate(self) }
  }
}

impl AsMut<Isolate> for EscapableHandleScope {
  fn as_mut(&mut self) -> &mut Isolate {
    unsafe { v8__EscapableHandleScope__GetIsolate(self) }
  }
}

use crate::scope::Entered;

impl<'s> InIsolate for Entered<'s, HandleScope> {
  fn isolate(&mut self) -> &mut Isolate {
    unsafe { v8__HandleScope__GetIsolate(self) }
  }
}

impl<'s> InIsolate for Entered<'s, EscapableHandleScope> {
  fn isolate(&mut self) -> &mut Isolate {
    unsafe { v8__EscapableHandleScope__GetIsolate(self) }
  }
}

pub trait ToLocal<'sc>: InIsolate {
  unsafe fn to_local<T>(&mut self, ptr: *mut T) -> Option<Local<'sc, T>> {
    crate::Local::<'sc, T>::from_raw_(ptr)
  }
}

impl<'s> ToLocal<'s> for crate::scope::Entered<'s, HandleScope> {}
impl<'s> ToLocal<'s> for crate::scope::Entered<'s, EscapableHandleScope> {}
