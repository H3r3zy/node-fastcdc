use neon::prelude::*;
use fastcdc::v2020::*;
use neon::types::buffer::TypedArray;

fn get_chunks (mut cx: FunctionContext) -> JsResult<JsArray> {
    // unbox and check arguments
    let bytes_handle = cx.argument::<JsArrayBuffer>(0)?;
    let min_size = cx.argument::<JsNumber>(1)?.value(&mut cx) as u32;
    let avg_size = cx.argument::<JsNumber>(2)?.value(&mut cx) as u32;
    let max_size = cx.argument::<JsNumber>(3)?.value(&mut cx) as u32;

    // read all of the cuts into a temporary vector
    let bytes = bytes_handle.as_slice(&cx);
    let mut cuts = Vec::new();
    let chunker = FastCDC::new(&bytes, min_size, avg_size, max_size);
    for entry in chunker {
        cuts.push(entry.offset);
    }
    cuts.push(bytes.len());

    // copy the cuts into a js array
    let result = JsArray::new(&mut cx, cuts.len());
    for (i, v) in cuts.iter().enumerate() {
        let n = cx.number(*v as f64);
        result.set(&mut cx, i as u32, n)?;
    }
    Ok(result)
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("get_chunks", get_chunks)?;
    Ok(())
}
