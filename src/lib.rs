#![crate_name = "bindgen"]
#![crate_type = "dylib"]
#![feature(globs, quote, phase, plugin_registrar)]

extern crate syntax;
extern crate rustc;
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
use rustc::plugin::Registry;

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


#[link(name="LLVMOption", kind="static")]
#[link(name="LLVMIRReader", kind="static")]
#[link(name="LLVMBitReader", kind="static")]
#[link(name="LLVMAsmParser", kind="static")]
#[link(name="LLVMCodeGen", kind="static")]
#[link(name="LLVMObjCARCOpts", kind="static")]
#[link(name="LLVMScalarOpts", kind="static")]
#[link(name="LLVMInstCombine", kind="static")]
#[link(name="LLVMTransformUtils", kind="static")]
#[link(name="LLVMipa", kind="static")]
#[link(name="LLVMAnalysis", kind="static")]
#[link(name="LLVMTarget", kind="static")]
#[link(name="LLVMMC", kind="static")]
#[link(name="LLVMObject", kind="static")]
#[link(name="LLVMCore", kind="static")]
#[link(name="LLVMSupport", kind="static")]
#[link(name="LLVMOption", kind="static")]
#[link(name="LLVMObjCARCOpts", kind="static")]
#[link(name="LLVMTransformUtils", kind="static")]
#[link(name="LLVMipa", kind="static")]
#[link(name="LLVMIRReader", kind="static")]
#[link(name="LLVMBitReader", kind="static")]
#[link(name="LLVMAsmParser", kind="static")]
#[link(name="LLVMAnalysis", kind="static")]
#[link(name="LLVMTarget", kind="static")]
#[link(name="LLVMMC", kind="static")]
#[link(name="LLVMObject", kind="static")]
#[link(name="LLVMCore", kind="static")]
#[link(name="LLVMSupport", kind="static")]
#[link(name="LLVMOption", kind="static")]
#[link(name="LLVMObjCARCOpts", kind="static")]
#[link(name="LLVMIRReader", kind="static")]
#[link(name="LLVMBitReader", kind="static")]
#[link(name="LLVMAsmParser", kind="static")]
#[link(name="LLVMInstrumentation", kind="static")]
#[link(name="LLVMTransformUtils", kind="static")]
#[link(name="LLVMipa", kind="static")]
#[link(name="LLVMAnalysis", kind="static")]
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

#[doc(hidden)]
#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("bindgen", macro::bindgen_macro);
}

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
