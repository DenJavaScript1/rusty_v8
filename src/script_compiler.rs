// Copyright 2019-2021 the Deno authors. All rights reserved. MIT license.
use std::{marker::PhantomData, mem::MaybeUninit};

use crate::Isolate;
use crate::Local;
use crate::Module;
use crate::ScriptOrigin;
use crate::String;
use crate::{HandleScope, UniqueRef};

extern "C" {
  fn v8__ScriptCompiler__Source__CONSTRUCT(
    buf: *mut MaybeUninit<Source>,
    source_string: *const String,
    origin: *const ScriptOrigin,
    cached_data: *mut CachedData,
  );
  fn v8__ScriptCompiler__Source__DESTRUCT(this: *mut Source);
  fn v8__ScriptCompiler__Source__GetCachedData<'a>(
    this: *const Source,
  ) -> *const CachedData<'a>;
  fn v8__ScriptCompiler__CachedData__NEW<'a>(
    data: *const u8,
    length: i32,
  ) -> *mut CachedData<'a>;
  fn v8__ScriptCompiler__CachedData__DELETE<'a>(this: *mut CachedData<'a>);
  fn v8__ScriptCompiler__CompileModule(
    isolate: *mut Isolate,
    source: *mut Source,
    options: CompileOptions,
    no_cache_reason: NoCacheReason,
  ) -> *const Module;
}

/// Source code which can then be compiled to a UnboundScript or Script.
#[repr(C)]
#[derive(Debug)]
pub struct Source([usize; 8]);

/// Compilation data that the embedder can cache and pass back to speed up future
/// compilations. The data is produced if the CompilerOptions passed to the compilation
/// functions in ScriptCompiler contains produce_data_to_cache = true. The data to cache
/// can then can be retrieved from UnboundScript.
#[repr(C)]
#[derive(Debug)]
pub struct CachedData<'a> {
  data: *const u8,
  length: i32,
  rejected: bool,
  buffer_policy: BufferPolicy,
  _phantom: PhantomData<&'a ()>,
}

impl<'a> Drop for CachedData<'a> {
  fn drop(&mut self) {
    unsafe {
      v8__ScriptCompiler__CachedData__DELETE(self);
    }
  }
}

impl<'a> CachedData<'a> {
  pub fn new(data: &'a [u8]) -> UniqueRef<Self> {
    unsafe {
      UniqueRef::from_raw(v8__ScriptCompiler__CachedData__NEW(
        data.as_ptr(),
        data.len() as i32,
      ))
    }
  }
}

impl<'a> std::ops::Deref for CachedData<'a> {
  type Target = [u8];
  fn deref(&self) -> &Self::Target {
    unsafe { std::slice::from_raw_parts(self.data, self.length as usize) }
  }
}

#[repr(C)]
#[derive(Debug)]
enum BufferPolicy {
  BufferNotOwned = 0,
  BufferOwned,
}

impl Source {
  pub fn new(source_string: Local<String>, origin: &ScriptOrigin) -> Self {
    let mut buf = MaybeUninit::<Self>::uninit();
    unsafe {
      v8__ScriptCompiler__Source__CONSTRUCT(
        &mut buf,
        &*source_string,
        origin,
        std::ptr::null_mut(),
      );
      buf.assume_init()
    }
  }

  pub fn new_with_cached_data(
    source_string: Local<String>,
    origin: &ScriptOrigin,
    cached_data: UniqueRef<CachedData>,
  ) -> Self {
    let mut buf = MaybeUninit::<Self>::uninit();
    unsafe {
      v8__ScriptCompiler__Source__CONSTRUCT(
        &mut buf,
        &*source_string,
        origin,
        cached_data.into_raw(), // Source constructor takes ownership.
      );
      buf.assume_init()
    }
  }

  pub fn get_cached_data(&self) -> &CachedData {
    unsafe { &*v8__ScriptCompiler__Source__GetCachedData(self) }
  }
}

impl Drop for Source {
  fn drop(&mut self) {
    unsafe { v8__ScriptCompiler__Source__DESTRUCT(self) }
  }
}

#[repr(C)]
#[derive(Debug)]
pub enum CompileOptions {
  NoCompileOptions = 0,
  ConsumeCodeCache,
  EagerCompile,
}

/// The reason for which we are not requesting or providing a code cache.
#[repr(C)]
#[derive(Debug)]
pub enum NoCacheReason {
  NoReason = 0,
  BecauseCachingDisabled,
  BecauseNoResource,
  BecauseInlineScript,
  BecauseModule,
  BecauseStreamingSource,
  BecauseInspector,
  BecauseScriptTooSmall,
  BecauseCacheTooCold,
  BecauseV8Extension,
  BecauseExtensionModule,
  BecausePacScript,
  BecauseInDocumentWrite,
  BecauseResourceWithNoCacheHandler,
  BecauseDeferredProduceCodeCache,
}

/// Compile an ES module, returning a Module that encapsulates the compiled
/// code.
///
/// Corresponds to the ParseModule abstract operation in the ECMAScript
/// specification.
pub fn compile_module<'s>(
  scope: &mut HandleScope<'s>,
  source: Source,
) -> Option<Local<'s, Module>> {
  compile_module2(
    scope,
    source,
    CompileOptions::NoCompileOptions,
    NoCacheReason::NoReason,
  )
}

/// Same as compile_module with more options.
pub fn compile_module2<'s>(
  scope: &mut HandleScope<'s>,
  mut source: Source,
  options: CompileOptions,
  no_cache_reason: NoCacheReason,
) -> Option<Local<'s, Module>> {
  unsafe {
    scope.cast_local(|sd| {
      v8__ScriptCompiler__CompileModule(
        sd.get_isolate_ptr(),
        &mut source,
        options,
        no_cache_reason,
      )
    })
  }
}
