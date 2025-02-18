use std::{collections::BTreeMap, path::PathBuf, str::FromStr};

use expect_test::expect_file;
use miden_core::crypto::hash::RpoDigest;
use miden_frontend_wasm::{ImportMetadata, WasmTranslationConfig};
use miden_hir::{FunctionExportName, InterfaceFunctionIdent, InterfaceIdent, Symbol};

use crate::CompilerTest;

#[test]
fn sdk() {
    let test = CompilerTest::rust_source_cargo_component(
        PathBuf::from_str("../rust-apps-wasm/wit-sdk/sdk").unwrap(),
        Default::default(),
    );
    let artifact_name = test.source.artifact_name();
    test.expect_wasm(expect_file![format!(
        "../../expected/wit_sdk_basic_wallet/{artifact_name}.wat"
    )]);
}

#[test]
fn sdk_basic_wallet() {
    let interface_tx = InterfaceIdent::from_full_ident("miden:base/tx@1.0.0".to_string());
    let create_note_ident = InterfaceFunctionIdent {
        interface: interface_tx.clone(),
        function: Symbol::intern("create-note"),
    };
    let interface_account = InterfaceIdent::from_full_ident("miden:base/account@1.0.0".to_string());
    let add_asset_ident = InterfaceFunctionIdent {
        interface: interface_account.clone(),
        function: Symbol::intern("add-asset"),
    };
    let remove_asset_ident = InterfaceFunctionIdent {
        interface: interface_account.clone(),
        function: Symbol::intern("remove-asset"),
    };
    let import_metadata: BTreeMap<InterfaceFunctionIdent, ImportMetadata> = [
        (
            create_note_ident.clone(),
            ImportMetadata {
                digest: RpoDigest::default(),
            },
        ),
        (
            remove_asset_ident.clone(),
            ImportMetadata {
                digest: RpoDigest::default(),
            },
        ),
        (
            add_asset_ident.clone(),
            ImportMetadata {
                digest: RpoDigest::default(),
            },
        ),
    ]
    .into_iter()
    .collect();
    let expected_exports: Vec<FunctionExportName> =
        vec![Symbol::intern("send-asset").into(), Symbol::intern("receive-asset").into()];
    let config = WasmTranslationConfig {
        import_metadata: import_metadata.clone(),
        ..Default::default()
    };
    let mut test = CompilerTest::rust_source_cargo_component(
        PathBuf::from_str("../rust-apps-wasm/wit-sdk/basic-wallet").unwrap(),
        config,
    );
    let artifact_name = test.source.artifact_name();
    test.expect_wasm(expect_file![format!(
        "../../expected/wit_sdk_basic_wallet/{artifact_name}.wat"
    )]);
    test.expect_ir(expect_file![format!(
        "../../expected/wit_sdk_basic_wallet/{artifact_name}.hir"
    )]);
    let ir = test.hir().unwrap_component();
    for (_, import) in ir.imports() {
        assert!(import_metadata.contains_key(&import.unwrap_canon_abi_import().interface_function));
    }
    for name in expected_exports {
        assert!(ir.exports().contains_key(&name));
    }
}

#[test]
fn sdk_basic_wallet_p2id_note() {
    let interface_account = InterfaceIdent::from_full_ident("miden:base/account@1.0.0".to_string());
    let basic_wallet =
        InterfaceIdent::from_full_ident("miden:basic-wallet/basic-wallet@1.0.0".to_string());
    let core_types = InterfaceIdent::from_full_ident("miden:base/core-types@1.0.0".to_string());
    let note = InterfaceIdent::from_full_ident("miden:base/note@1.0.0".to_string());
    let import_metadata: BTreeMap<InterfaceFunctionIdent, ImportMetadata> = [
        (
            InterfaceFunctionIdent {
                interface: interface_account.clone(),
                function: Symbol::intern("get-id"),
            },
            ImportMetadata {
                digest: RpoDigest::default(),
            },
        ),
        (
            InterfaceFunctionIdent {
                interface: core_types.clone(),
                function: Symbol::intern("account-id-from-felt"),
            },
            ImportMetadata {
                digest: RpoDigest::default(),
            },
        ),
        (
            InterfaceFunctionIdent {
                interface: basic_wallet.clone(),
                function: Symbol::intern("receive-asset"),
            },
            ImportMetadata {
                digest: RpoDigest::default(),
            },
        ),
        (
            InterfaceFunctionIdent {
                interface: note.clone(),
                function: Symbol::intern("get-assets"),
            },
            ImportMetadata {
                digest: RpoDigest::default(),
            },
        ),
        (
            InterfaceFunctionIdent {
                interface: note.clone(),
                function: Symbol::intern("get-inputs"),
            },
            ImportMetadata {
                digest: RpoDigest::default(),
            },
        ),
    ]
    .into_iter()
    .collect();
    let expected_exports: Vec<FunctionExportName> = vec![Symbol::intern("note-script").into()];
    let config = WasmTranslationConfig {
        import_metadata: import_metadata.clone(),
        ..Default::default()
    };
    let mut test = CompilerTest::rust_source_cargo_component(
        PathBuf::from_str("../rust-apps-wasm/wit-sdk/p2id-note").unwrap(),
        config,
    );
    let artifact_name = test.source.artifact_name();
    test.expect_wasm(expect_file![format!(
        "../../expected/wit_sdk_basic_wallet/{artifact_name}.wat"
    )]);
    test.expect_ir(expect_file![format!(
        "../../expected/wit_sdk_basic_wallet/{artifact_name}.hir"
    )]);
    let ir = test.hir().unwrap_component();
    for (_, import) in ir.imports() {
        let canon_abi_import = import.unwrap_canon_abi_import();
        assert!(import_metadata.contains_key(&canon_abi_import.interface_function));
        if ["get-assets", "get-inputs"]
            .contains(&canon_abi_import.interface_function.function.as_str())
        {
            assert!(canon_abi_import.options.realloc.is_some());
        }
    }
    for name in expected_exports {
        assert!(ir.exports().contains_key(&name));
    }
}
