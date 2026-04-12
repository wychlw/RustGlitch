use std::{
    error::Error,
    ffi::{CStr, CString, OsStr},
    os::raw::c_int,
    path::Path,
    ptr::null_mut,
    str::FromStr,
};

use clang_sys::{
    CXChildVisitResult, CXClientData, CXCursor, CXTranslationUnit_None, clang_createIndex,
    clang_disposeIndex, clang_disposeTranslationUnit, clang_getCString, clang_getCursorExtent,
    clang_getCursorKind, clang_getCursorSpelling, clang_getFileLocation, clang_getRangeEnd,
    clang_getRangeStart, clang_getSpellingLocation, clang_getTranslationUnitCursor,
    clang_parseTranslationUnit, clang_visitChildren,
};
use walkdir::WalkDir;

use crate::{conf::Args, fuzz::fuzzbase::Fuzzer};

#[derive(Clone)]
pub struct CppMutater {}

impl CppMutater {
    fn travel_file(&self, cursor: CXCursor, _tu_code: &str) -> Result<(), Box<dyn Error>> {
        extern "C" fn visitor(
            cursor: CXCursor,
            _parent: CXCursor,
            client_data: CXClientData,
        ) -> CXChildVisitResult {
            unsafe {
                let kind = clang_getCursorKind(cursor);

                let spelling = clang_getCursorSpelling(cursor);
                let spelling_str = clang_getCString(spelling);
                let spelling_ss = {
                    if spelling_str.is_null() {
                        String::new()
                    } else {
                        CStr::from_ptr(spelling_str).to_string_lossy().into_owned()
                    }
                };

                let range = clang_getCursorExtent(cursor);
                let start = clang_getRangeStart(range);
                let end = clang_getRangeEnd(range);
                clang_getSpellingLocation(start, null_mut(), null_mut(), null_mut(), null_mut());
                clang_getSpellingLocation(end, null_mut(), null_mut(), null_mut(), null_mut());
                let mut start_offset = 0;
                let mut end_offset = 0;
                clang_getFileLocation(start, null_mut(), null_mut(), null_mut(), &mut start_offset);
                clang_getFileLocation(end, null_mut(), null_mut(), null_mut(), &mut end_offset);

                println!(
                    "Cursor: {:?}, Spelling: {}, Kind: {:?}, Offsets: {}-{}",
                    cursor, spelling_ss, kind, start_offset, end_offset
                );

                clang_visitChildren(cursor, visitor, client_data) as CXChildVisitResult
            }
        }

        unsafe {
            clang_visitChildren(cursor, visitor, null_mut());
        }
        Ok(())
    }
    fn parse_one_file(&self, fname: &Path) -> Result<(), Box<dyn Error>> {
        // let parser = Clang::new()?;
        // let index = Index::new(&clang, false, false);

        // let tu = index
        //     .parser(fname)
        //     .arguments(&["-std=c++11", "-I/usr/include/c++/v1"])
        //     .parse()?;

        // let entity = tu.get_entity();
        // entity.visit_children(|child| {
        //     println!("KI: {}", child.get_kind());
        //     println!("PR: {}", entity.get_pretty_printer().print());
        // });
        let code = std::fs::read_to_string(fname)?;
        unsafe {
            let file_name = fname.to_str().ok_or("Invalid file name")?;
            let file_name = CString::from_str(file_name)?;

            let args = [
                CString::new("-std=c++17")?,
                CString::new("-I/usr/include/c++/15.1.1/")?,
            ];
            let args_p = args.iter().map(|s| s.as_ptr()).collect::<Vec<_>>();

            let index = clang_createIndex(0, 0);
            if index.is_null() {
                return Err("Failed to create Clang index".into());
            }
            let tu = clang_parseTranslationUnit(
                index,
                file_name.as_ptr(),
                args_p.as_ptr(),
                args_p.len() as c_int,
                null_mut(),
                0,
                CXTranslationUnit_None,
            );
            if tu.is_null() {
                return Err("Failed to parse translation unit".into());
            }

            let cursor = clang_getTranslationUnitCursor(tu);

            self.travel_file(cursor, &code)?;

            clang_disposeTranslationUnit(tu);
            clang_disposeIndex(index);
        }
        Ok(())
    }
}

impl Fuzzer for CppMutater {
    fn new(conf: &Args) -> Result<Box<dyn Fuzzer>, Box<dyn Error>> {
        let res = CppMutater {};
        let dirs = &conf.input;
        for dir in dirs {
            for f in WalkDir::new(dir) {
                let entry = f.map_err(|e| format!("Error walking dir {}, {}", dir.display(), e))?;
                if !entry.file_type().is_file() {
                    continue;
                }
                if entry.path().extension() != Some(OsStr::new("cpp"))
                    && entry.path().extension() != Some(OsStr::new("c"))
                {
                    continue;
                }
                res.parse_one_file(entry.path())?;
            }
        }
        Ok(Box::new(res))
    }
    fn generate(&mut self) -> Result<Vec<u8>, Box<dyn Error>> {
        unimplemented!()
    }
    fn inform_ice(&mut self, _code: &[u8], _is_dup: bool) -> Result<(), Box<dyn Error>> {
        unimplemented!()
    }
}
