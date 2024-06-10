use std::time::{SystemTime, UNIX_EPOCH};
use neon::prelude::*;
use turbosql::{Turbosql, select, execute};

#[derive(Turbosql, Default)]
struct BucketData {
    rowid: Option<i64>, // rowid member required & enforced at compile time
    key: Option<String>,
    age: Option<i64>,
    image_jpg: Option<Vec<u8>>
}

#[derive(Turbosql, Default)]
struct StorageData {
    rowid: Option<i64>, // rowid member required & enforced at compile time
    key: Option<String>,
    value: Option<String>,
    expiry: Option<u32>
}

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello node"))
}

fn setItem(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    let key = cx
       .argument::<JsString>(0)?
       .value();
    let value = cx
        .argument::<JsString>(1)?
        .value();
    println!("KEY {}", key);
    println!("VALUE {}", value);

    let currentTimestamp: u32 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as u32;

    let mut storedData = match select!(StorageData "WHERE key = ?", key) {
        Ok(data) => data,
        Err(_) => StorageData::default(),
    };

    if storedData.rowid.is_none() {
        println!("DOESNT EXIST");
        // Row doesn't exist, insert a new one
        let newData = StorageData {
            key: Some(key),
            value: Some(value),
            expiry: Some(currentTimestamp),
            ..Default::default()
        }.insert();
    } else {
        println!("EXISTS");
        // Row exists, update it
        storedData.expiry = Some(currentTimestamp);
        storedData.update();
    }
    Ok(cx.boolean(true))
}

register_module!(mut cx, {
    cx.export_function("hello", hello);
    cx.export_function("setItem", setItem);
    Ok(())
});