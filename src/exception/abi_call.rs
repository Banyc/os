use crate::{print, println, sbi_call};

use super::ExceptionMutContext;

const A0: usize = 10;
const A1: usize = 11;
const A6: usize = 16;
const A7: usize = 17;

pub fn abi_call(mut_context: &mut ExceptionMutContext) {
    println!("Abi call");

    let sbi = sbi_call::decode_sbi_call(
        mut_context.register_context.x[A0],
        mut_context.register_context.x[A1],
        mut_context.register_context.x[A6],
        mut_context.register_context.x[A7],
    );

    match sbi {
        sbi_call::CompatibleSbi::Legacy(ext) => {
            let res = sbi_call::legacy_sbi_call(&ext);
            match res {
                Ok(value) => {
                    mut_context.register_context.x[A0] = value as usize;
                }
                Err(error) => {
                    mut_context.register_context.x[A0] = error as usize;
                }
            }
        }
        sbi_call::CompatibleSbi::Extension(ext) => {
            let res = sbi_call::sbi_call(&ext);
            match res {
                Ok(value) => {
                    mut_context.register_context.x[A0] = 0;
                    mut_context.register_context.x[A1] = value as usize;
                }
                Err(error) => {
                    mut_context.register_context.x[A0] = error as usize;
                }
            }
        }
    }
}
