// Copyright 2019-2021 the Deno authors. All rights reserved. MIT license.

//! # Example
//!
//! ```rust
//! let platform = v8::new_default_platform(0, false).make_shared();
//! v8::V8::initialize_platform(platform);
//! v8::V8::initialize();
//!
//! let isolate = &mut v8::Isolate::new(Default::default());
//!
//! let scope = &mut v8::HandleScope::new(isolate);
//! let context = v8::Context::new(scope);
//! let scope = &mut v8::ContextScope::new(scope, context);
//!
//! let code = v8::String::new(scope, "'Hello' + ' World!'").unwrap();
//! println!("javascript code: {}", code.to_rust_string_lossy(scope));
//!
//! let script = v8::Script::compile(scope, code, None).unwrap();
//! let result = script.run(scope).unwrap();
//! let result = result.to_string(scope).unwrap();
//! println!("result: {}", result.to_rust_string_lossy(scope));
//! ```

#![allow(clippy::missing_safety_doc)]

#[macro_use]
extern crate bitflags;

mod array_buffer;
mod array_buffer_view;
mod bigint;
mod context;
pub mod cppgc;
mod data;
mod date;
mod exception;
mod external;
mod external_references;
pub mod fast_api;
mod fixed_array;
mod function;
mod gc;
mod get_property_names_args_builder;
mod handle;
pub mod icu;
mod isolate;
mod isolate_create_params;
mod locker;
mod microtask;
mod module;
mod name;
mod number;
mod object;
mod platform;
mod primitive_array;
mod primitives;
mod private;
mod promise;
mod property_attribute;
mod property_descriptor;
mod property_filter;
mod property_handler_flags;
mod proxy;
mod scope;
mod script;
mod script_or_module;
mod shared_array_buffer;
mod snapshot;
mod string;
mod support;
mod symbol;
mod template;
mod typed_array;
mod unbound_module_script;
mod unbound_script;
mod value;
mod value_deserializer;
mod value_serializer;
mod wasm;

pub mod inspector;
pub mod json;
pub mod script_compiler;
// This module is intentionally named "V8" rather than "v8" to match the
// C++ namespace "v8::V8".
#[allow(non_snake_case)]
pub mod V8;

pub use array_buffer::*;
pub use data::*;
pub use exception::*;
pub use external_references::ExternalReference;
pub use external_references::ExternalReferences;
pub use function::*;
pub use gc::*;
pub use get_property_names_args_builder::*;
pub use handle::Global;
pub use handle::Handle;
pub use handle::Local;
pub use handle::Weak;
pub use isolate::GarbageCollectionType;
pub use isolate::HeapStatistics;
pub use isolate::HostCreateShadowRealmContextCallback;
pub use isolate::HostImportModuleDynamicallyCallback;
pub use isolate::HostInitializeImportMetaObjectCallback;
pub use isolate::Isolate;
pub use isolate::IsolateHandle;
pub use isolate::MemoryPressureLevel;
pub use isolate::MessageCallback;
pub use isolate::MessageErrorLevel;
pub use isolate::MicrotasksPolicy;
pub use isolate::NearHeapLimitCallback;
pub use isolate::OomDetails;
pub use isolate::OomErrorCallback;
pub use isolate::OwnedIsolate;
pub use isolate::PromiseHook;
pub use isolate::PromiseHookType;
pub use isolate::PromiseRejectCallback;
pub use isolate::SharedIsolate;
pub use isolate::WasmAsyncSuccess;
pub use isolate_create_params::CreateParams;
pub use locker::Locker;
pub use microtask::MicrotaskQueue;
pub use module::*;
pub use object::*;
pub use platform::new_default_platform;
pub use platform::new_single_threaded_default_platform;
pub use platform::new_unprotected_default_platform;
pub use platform::Platform;
pub use primitives::*;
pub use promise::{PromiseRejectEvent, PromiseRejectMessage, PromiseState};
pub use property_attribute::*;
pub use property_descriptor::*;
pub use property_filter::*;
pub use property_handler_flags::*;
pub use scope::AllowJavascriptExecutionScope;
pub use scope::CallbackScope;
pub use scope::ContextScope;
pub use scope::DisallowJavascriptExecutionScope;
pub use scope::EscapableHandleScope;
pub use scope::HandleScope;
pub use scope::OnFailure;
pub use scope::TryCatch;
pub use script::ScriptOrigin;
pub use script_compiler::CachedData;
pub use snapshot::FunctionCodeHandling;
pub use snapshot::StartupData;
pub use string::NewStringType;
pub use string::OneByteConst;
pub use string::WriteOptions;
pub use support::SharedPtr;
pub use support::SharedRef;
pub use support::UniquePtr;
pub use support::UniqueRef;
pub use template::*;
pub use value_deserializer::ValueDeserializer;
pub use value_deserializer::ValueDeserializerHelper;
pub use value_deserializer::ValueDeserializerImpl;
pub use value_serializer::ValueSerializer;
pub use value_serializer::ValueSerializerHelper;
pub use value_serializer::ValueSerializerImpl;
pub use wasm::CompiledWasmModule;
pub use wasm::WasmStreaming;

// TODO(piscisaureus): Ideally this trait would not be exported.
pub use support::MapFnTo;
