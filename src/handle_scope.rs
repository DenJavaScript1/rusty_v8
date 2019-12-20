use std::marker::PhantomData;
use std::mem::MaybeUninit;

use crate::isolate::Isolate;

extern "C" {
  fn v8__HandleScope__CONSTRUCT(
    buf: &mut MaybeUninit<HandleScope>,
    isolate: &Isolate,
  );
  fn v8__HandleScope__DESTRUCT(this: &mut HandleScope);
  fn v8__HandleScope__GetIsolate<'sc>(
    this: &'sc HandleScope,
  ) -> &'sc mut Isolate;
}

#[repr(C)]
pub struct HandleScope<'sc>([usize; 3], PhantomData<&'sc mut ()>);

impl<'sc> HandleScope<'sc> {
  pub fn enter(
    isolate: &Isolate,
    mut f: impl FnMut(&mut HandleScope<'_>) -> (),
  ) {
    let mut scope: MaybeUninit<Self> = MaybeUninit::uninit();
    unsafe { v8__HandleScope__CONSTRUCT(&mut scope, isolate) };
    let scope = unsafe { &mut *(scope.as_mut_ptr()) };
    f(scope);
    unsafe { v8__HandleScope__DESTRUCT(scope) };
  }

  fn get_isolate(&self) -> &Isolate {
    unsafe { v8__HandleScope__GetIsolate(self) }
  }
}
