//! https://www.jianshu.com/p/ebfb2c0325ab
//! https://blog.csdn.net/qq_42570601/article/details/108007986

use std::os::raw::{c_uint, c_ulonglong};
use std::str::FromStr;

use llvm_sys::core::{
    LLVMAddFunction, LLVMAddGlobal, LLVMConstInt, LLVMContextCreate, LLVMContextDispose,
    LLVMDisposeModule, LLVMDumpModule, LLVMFunctionType, LLVMGetNamedGlobal,
    LLVMInt64TypeInContext, LLVMInt8TypeInContext, LLVMModuleCreateWithName, LLVMPointerType,
    LLVMSetAlignment, LLVMSetFunctionCallConv, LLVMSetGlobalConstant, LLVMSetInitializer,
    LLVMSetLinkage, LLVMSetVisibility, LLVMVoidTypeInContext,
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

fn describe_bits(
    mod_ref: LLVMModuleRef,
    ctx_ref: LLVMContextRef,
    number: &str,
) -> Option<LLVMValueRef> {
    let number_name = char_const_ptr!(format!("Core.Types.Bits({}).this.number", number));
    let named_global = unsafe { LLVMGetNamedGlobal(mod_ref, number_name) };

    if named_global.is_null() {
        let bits_number: LLVMValueRef =
            unsafe { LLVMAddGlobal(mod_ref, LLVMInt64TypeInContext(ctx_ref), number_name) };

        unsafe { LLVMSetLinkage(bits_number, llvm_sys::LLVMLinkage::LLVMPrivateLinkage) };
        unsafe { LLVMSetAlignment(bits_number, core::mem::size_of::<c_ulonglong>() as c_uint) };
        unsafe { LLVMSetVisibility(bits_number, llvm_sys::LLVMVisibility::LLVMHiddenVisibility) };

        unsafe { LLVMSetGlobalConstant(bits_number, llvm_bool!(true)) };

        unsafe {
            LLVMSetInitializer(
                bits_number,
                LLVMConstInt(
                    LLVMInt64TypeInContext(ctx_ref),
                    c_ulonglong::from_str(number).unwrap(),
                    llvm_bool!(false),
                ),
            );
        };

        return Some(bits_number);
    }

    None
}

fn main() {
    let mod_ref: LLVMModuleRef = unsafe { LLVMModuleCreateWithName(char_const_ptr!("Core.Types")) };
    let ctx_ref: LLVMContextRef = unsafe { LLVMContextCreate() };

    declare_llvm_va_start(mod_ref, ctx_ref);
    declare_llvm_va_end(mod_ref, ctx_ref);

    describe_bits(mod_ref, ctx_ref, "1");

    unsafe { LLVMDumpModule(mod_ref) };

    unsafe { LLVMDisposeModule(mod_ref) };
    unsafe { LLVMContextDispose(ctx_ref) };
}

/* LLVMValueRef llvmGenLocalStringVar(const char* data, int len)
{
    LLVMValueRef glob = LLVMAddGlobal(mod, LLVMArrayType(LLVMInt8Type(), len), "string");

    // set as internal linkage and constant
    LLVMSetLinkage(glob, LLVMInternalLinkage);
    LLVMSetGlobalConstant(glob, TRUE);

    // Initialize with string:
    LLVMSetInitializer(glob, LLVMConstString(data, len, TRUE));

    return glob;
}  */
