use std::os::raw::c_uint;

use llvm_sys::core::{
    LLVMAddFunction, LLVMContextCreate, LLVMContextDispose, LLVMDisposeModule, LLVMDumpModule,
    LLVMFunctionType, LLVMInt8TypeInContext, LLVMModuleCreateWithName, LLVMPointerType,
    LLVMSetFunctionCallConv, LLVMVoidTypeInContext,
};
use llvm_sys::prelude::{LLVMBool, LLVMContextRef, LLVMModuleRef, LLVMTypeRef, LLVMValueRef};
use llvm_sys::LLVMCallConv;

#[macro_export(local_inner_macros)]
macro_rules! char_const_ptr {
    ($str:expr) => {{
        std::format!("{}\0", $str).as_bytes().as_ptr() as *const _
    }};
}

#[macro_export(local_inner_macros)]
macro_rules! llvm_bool {
    ($bool:expr) => {{
        $bool as LLVMBool
    }};
}

fn declare_llvm_va_start(mod_ref: LLVMModuleRef, ctx_ref: LLVMContextRef) -> LLVMValueRef {
    let mut llvm_va_start_types_ref: Vec<LLVMTypeRef> = Vec::new();
    llvm_va_start_types_ref.push(unsafe { LLVMPointerType(LLVMInt8TypeInContext(ctx_ref), 0) });

    let llvm_va_start_fn_type_ref: LLVMTypeRef = unsafe {
        LLVMFunctionType(
            LLVMVoidTypeInContext(ctx_ref),
            llvm_va_start_types_ref.as_mut_ptr(),
            1,
            llvm_bool!(false),
        )
    };
    let llvm_va_start_fn: LLVMValueRef = unsafe {
        LLVMAddFunction(mod_ref, char_const_ptr!("llvm.va_start"), llvm_va_start_fn_type_ref)
    };
    llvm_va_start_fn
}

fn declare_llvm_va_end(mod_ref: LLVMModuleRef, ctx_ref: LLVMContextRef) -> LLVMValueRef {
    let mut llvm_va_end_types_ref: Vec<LLVMTypeRef> = Vec::new();
    llvm_va_end_types_ref.push(unsafe { LLVMPointerType(LLVMInt8TypeInContext(ctx_ref), 0) });

    let llvm_va_end_fn_type_ref: LLVMTypeRef = unsafe {
        LLVMFunctionType(
            LLVMVoidTypeInContext(ctx_ref),
            llvm_va_end_types_ref.as_mut_ptr(),
            1,
            llvm_bool!(false),
        )
    };
    let llvm_va_end_fn: LLVMValueRef = unsafe {
        LLVMAddFunction(mod_ref, char_const_ptr!("llvm.va_end"), llvm_va_end_fn_type_ref)
    };
    unsafe { LLVMSetFunctionCallConv(llvm_va_end_fn, LLVMCallConv::LLVMCCallConv as c_uint) };
    llvm_va_end_fn
}

fn main() {
    let mod_ref: LLVMModuleRef = unsafe { LLVMModuleCreateWithName(char_const_ptr!("Core.Types")) };
    let ctx_ref: LLVMContextRef = unsafe { LLVMContextCreate() };

    declare_llvm_va_start(mod_ref, ctx_ref);
    declare_llvm_va_end(mod_ref, ctx_ref);

    unsafe { LLVMDumpModule(mod_ref) };

    unsafe { LLVMDisposeModule(mod_ref) };
    unsafe { LLVMContextDispose(ctx_ref) };
}
