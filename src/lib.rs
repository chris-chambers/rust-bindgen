#![crate_name = "bindgen"]
#![crate_type = "dylib"]
#![feature(globs, quote, phase, plugin_registrar)]

extern crate syntax;
//extern crate rustc;
extern crate libc;
#[phase(plugin, link)] extern crate log;

use std::collections::HashSet;
use std::default::Default;
use std::io::IoResult;

use syntax::ast;
use syntax::codemap::{DUMMY_SP, Span};
use syntax::print::{pp, pprust};
use syntax::print::pp::eof;
use syntax::ptr::P;
//use rustc::plugin::Registry;

use types::Global;

#[link(name="ncurses")]
#[link(name="z")]

#[link(name="clang", kind="static")]
#[link(name="clangARCMigrate", kind="static")]
#[link(name="clangAST", kind="static")]
#[link(name="clangASTMatchers", kind="static")]
#[link(name="clangAnalysis", kind="static")]
#[link(name="clangBasic", kind="static")]
#[link(name="clangCodeGen", kind="static")]
#[link(name="clangDriver", kind="static")]
#[link(name="clangDynamicASTMatchers", kind="static")]
#[link(name="clangEdit", kind="static")]
#[link(name="clangFormat", kind="static")]
#[link(name="clangFrontend", kind="static")]
#[link(name="clangFrontendTool", kind="static")]
#[link(name="clangIndex", kind="static")]
#[link(name="clangLex", kind="static")]
#[link(name="clangParse", kind="static")]
#[link(name="clangRewriteCore", kind="static")]
#[link(name="clangRewriteFrontend", kind="static")]
#[link(name="clangSema", kind="static")]
#[link(name="clangSerialization", kind="static")]
#[link(name="clangStaticAnalyzerCheckers", kind="static")]
#[link(name="clangStaticAnalyzerCore", kind="static")]
#[link(name="clangStaticAnalyzerFrontend", kind="static")]
#[link(name="clangTooling", kind="static")]


#[link(name="stdc++")]
#[link(name="ffi")]
#[link(name="LLVMInstrumentation", kind="static")]
#[link(name="LLVMIRReader", kind="static")]
#[link(name="LLVMAsmParser", kind="static")]
#[link(name="LLVMDebugInfo", kind="static")]
#[link(name="LLVMOption", kind="static")]
#[link(name="LLVMLTO", kind="static")]
#[link(name="LLVMLinker", kind="static")]
#[link(name="LLVMipo", kind="static")]
#[link(name="LLVMVectorize", kind="static")]
#[link(name="LLVMBitWriter", kind="static")]
#[link(name="LLVMBitReader", kind="static")]
#[link(name="LLVMTableGen", kind="static")]
#[link(name="LLVMR600CodeGen", kind="static")]
#[link(name="LLVMR600Desc", kind="static")]
#[link(name="LLVMR600Info", kind="static")]
#[link(name="LLVMR600AsmPrinter", kind="static")]
#[link(name="LLVMSystemZDisassembler", kind="static")]
#[link(name="LLVMSystemZCodeGen", kind="static")]
#[link(name="LLVMSystemZAsmParser", kind="static")]
#[link(name="LLVMSystemZDesc", kind="static")]
#[link(name="LLVMSystemZInfo", kind="static")]
#[link(name="LLVMSystemZAsmPrinter", kind="static")]
#[link(name="LLVMHexagonCodeGen", kind="static")]
#[link(name="LLVMHexagonAsmPrinter", kind="static")]
#[link(name="LLVMHexagonDesc", kind="static")]
#[link(name="LLVMHexagonInfo", kind="static")]
#[link(name="LLVMNVPTXCodeGen", kind="static")]
#[link(name="LLVMNVPTXDesc", kind="static")]
#[link(name="LLVMNVPTXInfo", kind="static")]
#[link(name="LLVMNVPTXAsmPrinter", kind="static")]
#[link(name="LLVMCppBackendCodeGen", kind="static")]
#[link(name="LLVMCppBackendInfo", kind="static")]
#[link(name="LLVMMSP430CodeGen", kind="static")]
#[link(name="LLVMMSP430Desc", kind="static")]
#[link(name="LLVMMSP430Info", kind="static")]
#[link(name="LLVMMSP430AsmPrinter", kind="static")]
#[link(name="LLVMXCoreDisassembler", kind="static")]
#[link(name="LLVMXCoreCodeGen", kind="static")]
#[link(name="LLVMXCoreDesc", kind="static")]
#[link(name="LLVMXCoreInfo", kind="static")]
#[link(name="LLVMXCoreAsmPrinter", kind="static")]
#[link(name="LLVMMipsDisassembler", kind="static")]
#[link(name="LLVMMipsCodeGen", kind="static")]
#[link(name="LLVMMipsAsmParser", kind="static")]
#[link(name="LLVMMipsDesc", kind="static")]
#[link(name="LLVMMipsInfo", kind="static")]
#[link(name="LLVMMipsAsmPrinter", kind="static")]
#[link(name="LLVMARMDisassembler", kind="static")]
#[link(name="LLVMARMCodeGen", kind="static")]
#[link(name="LLVMARMAsmParser", kind="static")]
#[link(name="LLVMARMDesc", kind="static")]
#[link(name="LLVMARMInfo", kind="static")]
#[link(name="LLVMARMAsmPrinter", kind="static")]
#[link(name="LLVMAArch64Disassembler", kind="static")]
#[link(name="LLVMAArch64CodeGen", kind="static")]
#[link(name="LLVMAArch64AsmParser", kind="static")]
#[link(name="LLVMAArch64Desc", kind="static")]
#[link(name="LLVMAArch64Info", kind="static")]
#[link(name="LLVMAArch64AsmPrinter", kind="static")]
#[link(name="LLVMAArch64Utils", kind="static")]
#[link(name="LLVMPowerPCCodeGen", kind="static")]
#[link(name="LLVMPowerPCAsmParser", kind="static")]
#[link(name="LLVMPowerPCDesc", kind="static")]
#[link(name="LLVMPowerPCInfo", kind="static")]
#[link(name="LLVMPowerPCAsmPrinter", kind="static")]
#[link(name="LLVMSparcCodeGen", kind="static")]
#[link(name="LLVMSparcDesc", kind="static")]
#[link(name="LLVMSparcInfo", kind="static")]
#[link(name="LLVMX86Disassembler", kind="static")]
#[link(name="LLVMX86AsmParser", kind="static")]
#[link(name="LLVMX86CodeGen", kind="static")]
#[link(name="LLVMSelectionDAG", kind="static")]
#[link(name="LLVMAsmPrinter", kind="static")]
#[link(name="LLVMX86Desc", kind="static")]
#[link(name="LLVMX86Info", kind="static")]
#[link(name="LLVMX86AsmPrinter", kind="static")]
#[link(name="LLVMX86Utils", kind="static")]
#[link(name="LLVMMCDisassembler", kind="static")]
#[link(name="LLVMMCParser", kind="static")]
#[link(name="LLVMInterpreter", kind="static")]
#[link(name="LLVMMCJIT", kind="static")]
#[link(name="LLVMJIT", kind="static")]
#[link(name="LLVMCodeGen", kind="static")]
#[link(name="LLVMObjCARCOpts", kind="static")]
#[link(name="LLVMScalarOpts", kind="static")]
#[link(name="LLVMInstCombine", kind="static")]
#[link(name="LLVMTransformUtils", kind="static")]
#[link(name="LLVMipa", kind="static")]
#[link(name="LLVMAnalysis", kind="static")]
#[link(name="LLVMRuntimeDyld", kind="static")]
#[link(name="LLVMExecutionEngine", kind="static")]
#[link(name="LLVMTarget", kind="static")]
#[link(name="LLVMMC", kind="static")]
#[link(name="LLVMObject", kind="static")]
#[link(name="LLVMCore", kind="static")]
#[link(name="LLVMSupport", kind="static")]
extern { }

#[allow(dead_code)]
mod types;
mod clangll;
#[allow(dead_code)]
mod clang;
mod gen;
mod parser;
mod macro;

//#[doc(hidden)]
//#[plugin_registrar]
//pub fn plugin_registrar(reg: &mut Registry) {
//    reg.register_macro("bindgen", macro::bindgen_macro);
//}

pub struct BindgenOptions {
    pub match_pat: Vec<String>,
    pub builtins: bool,
    pub links: Vec<(String, LinkType)>,
    pub emit_ast: bool,
    pub fail_on_bitfield: bool,
    pub fail_on_unknown_type: bool,
    pub override_enum_ty: String,
    pub clang_args: Vec<String>,
}

impl Default for BindgenOptions {
    fn default() -> BindgenOptions {
        BindgenOptions {
            match_pat: Vec::new(),
            builtins: false,
            links: Vec::new(),
            emit_ast: false,
            fail_on_bitfield: false,
            fail_on_unknown_type: false,
            override_enum_ty: "".to_string(),
            clang_args: Vec::new()
        }
    }
}

#[deriving(Copy)]
pub enum LinkType {
    Default,
    Static,
    Framework
}

pub trait Logger {
    fn error(&self, msg: &str);
    fn warn(&self, msg: &str);
}

pub struct Bindings
{
    module: ast::Mod
}

impl Bindings
{
    pub fn generate(options: &BindgenOptions, logger: Option<&Logger>, span: Option<Span>) -> Result<Bindings, ()> {
        let l = DummyLogger;
        let logger = match logger {
            Some(l) => l,
            None => &l as &Logger
        };

        let span = match span {
            Some(s) => s,
            None => DUMMY_SP
        };

        let globals = try!(parse_headers(options, logger));

        let module = ast::Mod {
            inner: span,
            view_items: Vec::new(),
            items: gen::gen_mod(options.links.as_slice(), globals, span)
        };

        Ok(Bindings {
            module: module
        })
    }

    pub fn into_ast(self) -> Vec<P<ast::Item>> {
        self.module.items
    }

    pub fn to_string(&self) -> String {
        pprust::to_string(|s| {
            s.s = pp::mk_printer(box Vec::new(), 80);

            try!(s.print_mod(&self.module, &[]));
            s.print_remaining_comments()
        })
    }

    pub fn write(&self, writer: &mut (Writer + 'static)) -> IoResult<()> {
        try!(writer.write("/* automatically generated by rust-bindgen */\n\n".as_bytes()));

        // This is safe as the Box<Writer> does not outlive ps or this function
        // Without this the interface is quite nasty
        let writer = unsafe { ::std::mem::transmute(writer) };
        let mut ps = pprust::rust_printer(writer);
        try!(ps.print_mod(&self.module, &[]));
        try!(ps.print_remaining_comments());
        try!(eof(&mut ps.s));
        ps.s.out.flush()
    }
}


struct DummyLogger;

impl Logger for DummyLogger {
    fn error(&self, _msg: &str) { }
    fn warn(&self, _msg: &str) { }
}

fn parse_headers(options: &BindgenOptions, logger: &Logger) -> Result<Vec<Global>, ()> {
    fn str_to_ikind(s: &str) -> Option<types::IKind> {
        match s {
            "uchar"     => Some(types::IUChar),
            "schar"     => Some(types::ISChar),
            "ushort"    => Some(types::IUShort),
            "sshort"    => Some(types::IShort),
            "uint"      => Some(types::IUInt),
            "sint"      => Some(types::IInt),
            "ulong"     => Some(types::IULong),
            "slong"     => Some(types::ILong),
            "ulonglong" => Some(types::IULongLong),
            "slonglong" => Some(types::ILongLong),
            _           => None,
        }
    }

    let clang_opts = parser::ClangParserOptions {
        builtin_names: builtin_names(),
        builtins: options.builtins,
        match_pat: options.match_pat.clone(),
        emit_ast: options.emit_ast,
        fail_on_bitfield: options.fail_on_bitfield,
        fail_on_unknown_type: options.fail_on_unknown_type,
        override_enum_ty: str_to_ikind(options.override_enum_ty.as_slice()),
        clang_args: options.clang_args.clone(),
    };

    parser::parse(clang_opts, logger)
}

fn builtin_names() -> HashSet<String> {
    let mut names = HashSet::new();
    let keys = [
        "__va_list_tag",
        "__va_list",
    ];

    keys.iter().all(|s| {
        names.insert(s.to_string());
        true
    });

    return names;
}
