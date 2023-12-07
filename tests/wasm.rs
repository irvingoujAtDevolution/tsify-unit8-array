use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen_test::wasm_bindgen_test;
#[wasm_bindgen_test]
fn test_convert() {
    #[derive(Debug, PartialEq, Serialize, Deserialize, Tsify)]
    #[tsify(into_wasm_abi, from_wasm_abi)]
    struct Unit;

    let js = Unit.into_js().unwrap();

    if cfg!(feature = "js") {
        assert!(js.is_undefined());
    } else {
        assert!(js.is_null());
    }

    assert_eq!(Unit::from_js(js).unwrap(), Unit);
}

#[cfg(feature = "js")]
mod JsTest {
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
    use serde::{Deserialize, Serialize};
    use tsify::Tsify;
    use wasm_bindgen::prelude::*;
    #[wasm_bindgen_test]
    fn test_byte_buf_convert() {
        // use js eval to call the get example struct function and validate the result is a uint8array
        js_sys::eval(
            r#"
        const example = get_example_struct();
        if (!(example.bytes instanceof Uint8Array)) {
            throw new Error("expected example to be a Uint8Array");
        }

        const passed = pass_example_struct(example);
        if (!(passed.bytes instanceof Uint8Array)) {
            throw new Error("expected passed to be a Uint8Array");
        }
        // check that the bytes are the same
        if (passed.bytes.length !== example.bytes.length) {
            throw new Error("expected passed to be the same length as example");
        }

        for (let i = 0; i < passed.bytes.length; i++) {
            if (passed.bytes[i] !== example.bytes[i]) {
                throw new Error("expected passed to be the same as example");
            }
        }
    "#,
        )
        .unwrap();
    }

    use serde_bytes::ByteBuf;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[derive(Debug, PartialEq, Serialize, Deserialize, Tsify)]
    #[tsify(into_wasm_abi, from_wasm_abi)]
    struct ByteBufStruct {
        bytes: ByteBuf,
    }
    #[wasm_bindgen]
    pub fn get_example_struct() -> ByteBufStruct {
        ByteBufStruct {
            bytes: ByteBuf::from(vec![1, 2, 3, 4, 5]),
        }
    }

    #[wasm_bindgen]
    pub fn pass_example_struct(example: ByteBufStruct) -> ByteBufStruct {
        example
    }
}
