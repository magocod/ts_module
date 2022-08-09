use neon::prelude::*;

pub struct Book {
    pub title: String,
    pub author: String,
    pub year: u32,
}

impl Book {
    pub fn to_object<'a>(&self, cx: &mut impl Context<'a>) -> JsResult<'a, JsObject> {
        let obj = cx.empty_object();

        let title = cx.string(self.title.clone());
        obj.set(cx, "title", title)?;

        let author = cx.string(self.author.clone());
        obj.set(cx, "author", author)?;

        let year = cx.number(self.year.clone());
        obj.set(cx, "year", year)?;

        Ok(obj)
    }
}
