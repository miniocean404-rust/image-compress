use neon::{
    prelude::{self, Context, FunctionContext, ModuleContext},
    result::{JsResult, NeonResult},
    types::{JsNumber, JsString},
};

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("hello", hello)?;
    Ok(())
}

fn hello(mut cx: FunctionContext) -> JsResult<JsNumber> {
    // JS函数的第一个参数，类型为 string
    let text = cx.argument::<JsString>(0)?.value(&mut cx);
    // JS函数的第二个参数，类型为 string
    let query = cx.argument::<JsString>(1)?.value(&mut cx);

    // 返回一个数字，计算文本中出现单词的次数，将所有单词转小写，然后再根据空格分割，再过滤算出个数

    let num = text.to_lowercase().split(" ").filter(|s| s == &query).count() as f64;

    Ok(cx.number(num))

    // Ok(cx.string("hello node"))
}
