use miden_hir::FunctionType;
use miden_hir_type::Type::*;

use crate::miden_abi::{FunctionTypeMap, ModuleFunctionTypeMap};

pub(crate) const PIPE_WORDS_TO_MEMORY: &str = "pipe_words_to_memory";
pub(crate) const PIPE_DOUBLE_WORDS_TO_MEMORY: &str = "pipe_double_words_to_memory";

pub(crate) fn signatures() -> ModuleFunctionTypeMap {
    let mut m: ModuleFunctionTypeMap = Default::default();
    let mut funcs: FunctionTypeMap = Default::default();
    funcs.insert(
        PIPE_WORDS_TO_MEMORY,
        FunctionType::new(
            [
                Felt, // num_words
                Felt, // write_ptr
            ],
            [
                Felt, Felt, Felt, Felt, // HASH
                Felt, // write_ptr'
            ],
        ),
    );
    funcs.insert(
        PIPE_DOUBLE_WORDS_TO_MEMORY,
        FunctionType::new(
            [
                Felt, Felt, Felt, Felt, // C
                Felt, Felt, Felt, Felt, // B
                Felt, Felt, Felt, Felt, // A
                Felt, // write_ptr
                Felt, // end_ptr
            ],
            [
                Felt, Felt, Felt, Felt, // C
                Felt, Felt, Felt, Felt, // B
                Felt, Felt, Felt, Felt, // A
                Felt, // write_ptr
            ],
        ),
    );
    m.insert("miden:prelude/std_mem", funcs);
    m
}
