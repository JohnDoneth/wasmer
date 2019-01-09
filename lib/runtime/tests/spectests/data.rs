// Rust test file autogenerated with cargo build (build/spectests.rs).
// Please do NOT modify it by hand, as it will be reset on next build.
// Test based on spectests/data.wast
#![allow(
    warnings,
    dead_code
)]
use wabt::wat2wasm;
use std::{f32, f64};

use wasmer_runtime::types::Value;
use wasmer_runtime::{Instance, module::Module};
use wasmer_clif_backend::CraneliftCompiler;

use crate::spectests::_common::{
    spectest_importobject,
    NaNCheck,
};


// Line 5
fn create_module_1() -> Box<Instance> {
    let module_str = "(module
      (memory (;0;) 1)
      (data (;0;) (i32.const 0) \"\")
      (data (;1;) (i32.const 1) \"abcd\")
      (data (;2;) (i32.const 0) \"\")
      (data (;3;) (i32.const 0) \"abc\")
      (data (;4;) (i32.const 0) \"\")
      (data (;5;) (i32.const 1) \"abcd\")
      (data (;6;) (i32.const 0) \"\")
      (data (;7;) (i32.const 0) \"abc\")
      (data (;8;) (i32.const 0) \"\")
      (data (;9;) (i32.const 1) \"abcd\")
      (data (;10;) (i32.const 0) \"\")
      (data (;11;) (i32.const 0) \"abc\"))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    let module = wasmer_runtime::compile(&wasm_binary[..], &CraneliftCompiler::new()).expect("WASM can't be compiled");
    module.instantiate(&spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_1(instance: &mut Instance) {
    // TODO Review is explicit start needed? Start now called in runtime::Instance::new()
    //instance.start();
}

// Line 23

#[test]
fn test_module_1() {
    let mut instance = create_module_1();
    // We group the calls together
    start_module_1(&mut instance);
}
fn create_module_2() -> Box<Instance> {
    let module_str = "(module
      (memory (;0;) 1)
      (data (;0;) (i32.const 0) \"a\"))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    let module = wasmer_runtime::compile(&wasm_binary[..], &CraneliftCompiler::new()).expect("WASM can't be compiled");
    module.instantiate(&spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_2(instance: &mut Instance) {
    // TODO Review is explicit start needed? Start now called in runtime::Instance::new()
    //instance.start();
}

// Line 27

#[test]
fn test_module_2() {
    let mut instance = create_module_2();
    // We group the calls together
    start_module_2(&mut instance);
}
fn create_module_3() -> Box<Instance> {
    let module_str = "(module
      (import \"spectest\" \"memory\" (memory (;0;) 1))
      (data (;0;) (i32.const 0) \"a\"))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    let module = wasmer_runtime::compile(&wasm_binary[..], &CraneliftCompiler::new()).expect("WASM can't be compiled");
    module.instantiate(&spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_3(instance: &mut Instance) {
    // TODO Review is explicit start needed? Start now called in runtime::Instance::new()
    //instance.start();
}

// Line 32

#[test]
fn test_module_3() {
    let mut instance = create_module_3();
    // We group the calls together
    start_module_3(&mut instance);
}
fn create_module_4() -> Box<Instance> {
    let module_str = "(module
      (memory (;0;) 1)
      (data (;0;) (i32.const 0) \"a\")
      (data (;1;) (i32.const 3) \"b\")
      (data (;2;) (i32.const 100) \"cde\")
      (data (;3;) (i32.const 5) \"x\")
      (data (;4;) (i32.const 3) \"c\"))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    let module = wasmer_runtime::compile(&wasm_binary[..], &CraneliftCompiler::new()).expect("WASM can't be compiled");
    module.instantiate(&spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_4(instance: &mut Instance) {
    // TODO Review is explicit start needed? Start now called in runtime::Instance::new()
    //instance.start();
}

// Line 40

#[test]
fn test_module_4() {
    let mut instance = create_module_4();
    // We group the calls together
    start_module_4(&mut instance);
}
fn create_module_5() -> Box<Instance> {
    let module_str = "(module
      (import \"spectest\" \"memory\" (memory (;0;) 1))
      (data (;0;) (i32.const 0) \"a\")
      (data (;1;) (i32.const 1) \"b\")
      (data (;2;) (i32.const 2) \"cde\")
      (data (;3;) (i32.const 3) \"f\")
      (data (;4;) (i32.const 2) \"g\")
      (data (;5;) (i32.const 1) \"h\"))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    let module = wasmer_runtime::compile(&wasm_binary[..], &CraneliftCompiler::new()).expect("WASM can't be compiled");
    module.instantiate(&spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_5(instance: &mut Instance) {
    // TODO Review is explicit start needed? Start now called in runtime::Instance::new()
    //instance.start();
}

// Line 82

#[test]
fn test_module_5() {
    let mut instance = create_module_5();
    // We group the calls together
    start_module_5(&mut instance);
}
fn create_module_6() -> Box<Instance> {
    let module_str = "(module
      (memory (;0;) 1)
      (data (;0;) (i32.const 0) \"a\")
      (data (;1;) (i32.const 65535) \"b\"))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    let module = wasmer_runtime::compile(&wasm_binary[..], &CraneliftCompiler::new()).expect("WASM can't be compiled");
    module.instantiate(&spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_6(instance: &mut Instance) {
    // TODO Review is explicit start needed? Start now called in runtime::Instance::new()
    //instance.start();
}

// Line 87

#[test]
fn test_module_6() {
    let mut instance = create_module_6();
    // We group the calls together
    start_module_6(&mut instance);
}
fn create_module_7() -> Box<Instance> {
    let module_str = "(module
      (import \"spectest\" \"memory\" (memory (;0;) 1))
      (data (;0;) (i32.const 0) \"a\")
      (data (;1;) (i32.const 65535) \"b\"))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    let module = wasmer_runtime::compile(&wasm_binary[..], &CraneliftCompiler::new()).expect("WASM can't be compiled");
    module.instantiate(&spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_7(instance: &mut Instance) {
    // TODO Review is explicit start needed? Start now called in runtime::Instance::new()
    //instance.start();
}

// Line 93

#[test]
fn test_module_7() {
    let mut instance = create_module_7();
    // We group the calls together
    start_module_7(&mut instance);
}
fn create_module_8() -> Box<Instance> {
    let module_str = "(module
      (memory (;0;) 2)
      (data (;0;) (i32.const 131071) \"a\"))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    let module = wasmer_runtime::compile(&wasm_binary[..], &CraneliftCompiler::new()).expect("WASM can't be compiled");
    module.instantiate(&spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_8(instance: &mut Instance) {
    // TODO Review is explicit start needed? Start now called in runtime::Instance::new()
    //instance.start();
}

// Line 98

#[test]
fn test_module_8() {
    let mut instance = create_module_8();
    // We group the calls together
    start_module_8(&mut instance);
}
fn create_module_9() -> Box<Instance> {
    let module_str = "(module
      (memory (;0;) 0)
      (data (;0;) (i32.const 0) \"\"))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    let module = wasmer_runtime::compile(&wasm_binary[..], &CraneliftCompiler::new()).expect("WASM can't be compiled");
    module.instantiate(&spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_9(instance: &mut Instance) {
    // TODO Review is explicit start needed? Start now called in runtime::Instance::new()
    //instance.start();
}

// Line 102

#[test]
fn test_module_9() {
    let mut instance = create_module_9();
    // We group the calls together
    start_module_9(&mut instance);
}
fn create_module_10() -> Box<Instance> {
    let module_str = "(module
      (import \"spectest\" \"memory\" (memory (;0;) 0))
      (data (;0;) (i32.const 0) \"\"))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    let module = wasmer_runtime::compile(&wasm_binary[..], &CraneliftCompiler::new()).expect("WASM can't be compiled");
    module.instantiate(&spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_10(instance: &mut Instance) {
    // TODO Review is explicit start needed? Start now called in runtime::Instance::new()
    //instance.start();
}

// Line 107

#[test]
fn test_module_10() {
    let mut instance = create_module_10();
    // We group the calls together
    start_module_10(&mut instance);
}
fn create_module_11() -> Box<Instance> {
    let module_str = "(module
      (memory (;0;) 0 0)
      (data (;0;) (i32.const 0) \"\"))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    let module = wasmer_runtime::compile(&wasm_binary[..], &CraneliftCompiler::new()).expect("WASM can't be compiled");
    module.instantiate(&spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_11(instance: &mut Instance) {
    // TODO Review is explicit start needed? Start now called in runtime::Instance::new()
    //instance.start();
}

// Line 112

#[test]
fn test_module_11() {
    let mut instance = create_module_11();
    // We group the calls together
    start_module_11(&mut instance);
}
fn create_module_12() -> Box<Instance> {
    let module_str = "(module
      (memory (;0;) 1)
      (data (;0;) (i32.const 65536) \"\"))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    let module = wasmer_runtime::compile(&wasm_binary[..], &CraneliftCompiler::new()).expect("WASM can't be compiled");
    module.instantiate(&spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_12(instance: &mut Instance) {
    // TODO Review is explicit start needed? Start now called in runtime::Instance::new()
    //instance.start();
}

// Line 117

#[test]
fn test_module_12() {
    let mut instance = create_module_12();
    // We group the calls together
    start_module_12(&mut instance);
}
fn create_module_13() -> Box<Instance> {
    let module_str = "(module
      (memory (;0;) 0)
      (data (;0;) (i32.const 0) \"\"))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    let module = wasmer_runtime::compile(&wasm_binary[..], &CraneliftCompiler::new()).expect("WASM can't be compiled");
    module.instantiate(&spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_13(instance: &mut Instance) {
    // TODO Review is explicit start needed? Start now called in runtime::Instance::new()
    //instance.start();
}

// Line 121

#[test]
fn test_module_13() {
    let mut instance = create_module_13();
    // We group the calls together
    start_module_13(&mut instance);
}
fn create_module_14() -> Box<Instance> {
    let module_str = "(module
      (import \"spectest\" \"memory\" (memory (;0;) 0))
      (data (;0;) (i32.const 0) \"\"))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    let module = wasmer_runtime::compile(&wasm_binary[..], &CraneliftCompiler::new()).expect("WASM can't be compiled");
    module.instantiate(&spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_14(instance: &mut Instance) {
    // TODO Review is explicit start needed? Start now called in runtime::Instance::new()
    //instance.start();
}

// Line 126

#[test]
fn test_module_14() {
    let mut instance = create_module_14();
    // We group the calls together
    start_module_14(&mut instance);
}
fn create_module_15() -> Box<Instance> {
    let module_str = "(module
      (memory (;0;) 0 0)
      (data (;0;) (i32.const 0) \"\"))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    let module = wasmer_runtime::compile(&wasm_binary[..], &CraneliftCompiler::new()).expect("WASM can't be compiled");
    module.instantiate(&spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_15(instance: &mut Instance) {
    // TODO Review is explicit start needed? Start now called in runtime::Instance::new()
    //instance.start();
}

// Line 131

#[test]
fn test_module_15() {
    let mut instance = create_module_15();
    // We group the calls together
    start_module_15(&mut instance);
}
fn create_module_16() -> Box<Instance> {
    let module_str = "(module
      (import \"spectest\" \"memory\" (memory (;0;) 0))
      (data (;0;) (i32.const 0) \"a\"))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    let module = wasmer_runtime::compile(&wasm_binary[..], &CraneliftCompiler::new()).expect("WASM can't be compiled");
    module.instantiate(&spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_16(instance: &mut Instance) {
    // TODO Review is explicit start needed? Start now called in runtime::Instance::new()
    //instance.start();
}

// Line 136

#[test]
fn test_module_16() {
    let mut instance = create_module_16();
    // We group the calls together
    start_module_16(&mut instance);
}
fn create_module_17() -> Box<Instance> {
    let module_str = "(module
      (import \"spectest\" \"memory\" (memory (;0;) 0 3))
      (data (;0;) (i32.const 0) \"a\"))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    let module = wasmer_runtime::compile(&wasm_binary[..], &CraneliftCompiler::new()).expect("WASM can't be compiled");
    module.instantiate(&spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_17(instance: &mut Instance) {
    // TODO Review is explicit start needed? Start now called in runtime::Instance::new()
    //instance.start();
}

// Line 155

#[test]
fn test_module_17() {
    let mut instance = create_module_17();
    // We group the calls together
    start_module_17(&mut instance);
}
fn create_module_18() -> Box<Instance> {
    let module_str = "(module
      (import \"spectest\" \"memory\" (memory (;0;) 0))
      (data (;0;) (i32.const 1) \"a\"))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    let module = wasmer_runtime::compile(&wasm_binary[..], &CraneliftCompiler::new()).expect("WASM can't be compiled");
    module.instantiate(&spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_18(instance: &mut Instance) {
    // TODO Review is explicit start needed? Start now called in runtime::Instance::new()
    //instance.start();
}

// Line 160

#[test]
fn test_module_18() {
    let mut instance = create_module_18();
    // We group the calls together
    start_module_18(&mut instance);
}
fn create_module_19() -> Box<Instance> {
    let module_str = "(module
      (import \"spectest\" \"memory\" (memory (;0;) 0 3))
      (data (;0;) (i32.const 1) \"a\"))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    let module = wasmer_runtime::compile(&wasm_binary[..], &CraneliftCompiler::new()).expect("WASM can't be compiled");
    module.instantiate(&spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_19(instance: &mut Instance) {
    // TODO Review is explicit start needed? Start now called in runtime::Instance::new()
    //instance.start();
}

// Line 168

// Line 176

// Line 184

// Line 192

// Line 200

// Line 217

// Line 226

// Line 233

// Line 241

// Line 249

// Line 257

// Line 264

// Line 272

// Line 279

// Line 289
#[test]
fn c33_l289_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 11, 6, 1, 0, 65, 0, 11, 0];
    let module = wasmer_runtime::compile(&wasm_binary, &CraneliftCompiler::new());
    assert!(module.is_err(), "WASM should not compile as is invalid");
}

// Line 298
#[test]
fn c34_l298_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 5, 3, 1, 0, 1, 11, 6, 1, 0, 66, 0, 11, 0];
    let module = wasmer_runtime::compile(&wasm_binary, &CraneliftCompiler::new());
    assert!(module.is_err(), "WASM should not compile as is invalid");
}

// Line 306
#[test]
fn c35_l306_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 5, 3, 1, 0, 1, 11, 7, 1, 0, 65, 0, 104, 11, 0];
    let module = wasmer_runtime::compile(&wasm_binary, &CraneliftCompiler::new());
    assert!(module.is_err(), "WASM should not compile as is invalid");
}

// Line 314
#[test]
fn c36_l314_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 5, 3, 1, 0, 1, 11, 5, 1, 0, 1, 11, 0];
    let module = wasmer_runtime::compile(&wasm_binary, &CraneliftCompiler::new());
    assert!(module.is_err(), "WASM should not compile as is invalid");
}

// Line 322
#[test]
fn c37_l322_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 5, 3, 1, 0, 1, 11, 7, 1, 0, 1, 65, 0, 11, 0];
    let module = wasmer_runtime::compile(&wasm_binary, &CraneliftCompiler::new());
    assert!(module.is_err(), "WASM should not compile as is invalid");
}

// Line 330
#[test]
fn c38_l330_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 5, 3, 1, 0, 1, 11, 7, 1, 0, 65, 0, 1, 11, 0];
    let module = wasmer_runtime::compile(&wasm_binary, &CraneliftCompiler::new());
    assert!(module.is_err(), "WASM should not compile as is invalid");
}

#[test]
fn test_module_19() {
    let mut instance = create_module_19();
    // We group the calls together
    start_module_19(&mut instance);
}