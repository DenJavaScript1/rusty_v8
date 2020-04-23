//! Example of wrapping a v8::Isolate to add functionality. This is a pattern we
//! hope to use in deno_core.

use rusty_v8 as v8;
use std::ops::Deref;
use std::ops::DerefMut;
use std::rc::Rc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

static START: std::sync::Once = std::sync::Once::new();

struct Layer1(v8::OwnedIsolate);

struct Layer1State {
  drop_count: Rc<AtomicUsize>,
  i: usize,
}

impl Drop for Layer1State {
  fn drop(&mut self) {
    self.drop_count.fetch_add(1, Ordering::SeqCst);
  }
}

impl Layer1 {
  fn new(drop_count: Rc<AtomicUsize>) -> Layer1 {
    START.call_once(|| {
      v8::V8::initialize_platform(v8::new_default_platform().unwrap());
      v8::V8::initialize();
    });
    let mut isolate = v8::Isolate::new(Default::default());
    let state = Layer1State { drop_count, i: 0 };
    isolate.set_data_2(state);
    Layer1(isolate)
  }

  // Returns false if there was an error.
  fn execute(&mut self, code: &str) -> bool {
    let mut hs = v8::HandleScope::new(&mut self.0);
    let scope = hs.enter();
    let context = v8::Context::new(scope);
    let mut cs = v8::ContextScope::new(scope, context);
    let scope = cs.enter();
    let source = v8::String::new(scope, code).unwrap();
    let mut script = v8::Script::compile(scope, context, source, None).unwrap();
    let r = script.run(scope, context);
    r.is_some()
  }

  fn get_i(&self) -> usize {
    let s = self.0.get_data_2::<Layer1State>().unwrap();
    s.i
  }

  fn set_i(&self, i: usize) {
    let mut s = self.0.get_data_2_mut::<Layer1State>().unwrap();
    s.i = i;
  }
}

impl Deref for Layer1 {
  type Target = v8::Isolate;

  fn deref(&self) -> &v8::Isolate {
    &self.0
  }
}

impl DerefMut for Layer1 {
  fn deref_mut(&mut self) -> &mut v8::Isolate {
    &mut self.0
  }
}

#[test]
fn layer1_test() {
  let drop_count = Rc::new(AtomicUsize::new(0));

  let mut l = Layer1::new(drop_count.clone());
  assert!(l.execute("1 + 1"));
  assert!(!l.execute("throw 'foo'"));
  assert_eq!(0, l.get_i());
  l.set_i(123);
  assert_eq!(123, l.get_i());
  assert_eq!(drop_count.load(Ordering::SeqCst), 0);

  // Check that we can deref Layer1 by running a random v8::Isolate method
  l.run_microtasks();

  drop(l);
  assert_eq!(drop_count.load(Ordering::SeqCst), 1);
}
