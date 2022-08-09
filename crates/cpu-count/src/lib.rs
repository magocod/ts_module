use neon::prelude::*;

pub mod book;

use book::Book;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello node"))
}

fn get_num_cpus(mut cx: FunctionContext) -> JsResult<JsNumber> {
    Ok(cx.number(num_cpus::get() as f64))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    let book = Book {
        title: "A".to_string(),
        author: "B".to_string(),
        year: 2009,
    };

    cx.export_function("hello", hello)?;
    cx.export_function("get", get_num_cpus)?;

    let obj = book.to_object(&mut cx)?;
    cx.export_value("book", obj)?;
    Ok(())
}
