use std::time::{SystemTime, UNIX_EPOCH};
use neon::prelude::*;
use turbosql::{Turbosql, select, execute};

#[derive(Turbosql, Default)]
struct BucketData {
    rowid: Option<i64>, // rowid member required & enforced at compile time
    key: Option<String>
}

#[derive(Turbosql, Default)]
struct StorageData {
    rowid: Option<i64>, // rowid member required & enforced at compile time
    key: Option<String>,
    value: Option<String>,
    expiry: Option<u32>
}

fn setItem(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    let key = cx
       .argument::<JsString>(0)?
       .value();
    let value = cx
        .argument::<JsString>(1)?
        .value();
    let ttl = cx
        .argument::<JsNumber>(2)?
        .value();

    let currentTimestamp: u32 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as u32;

    let mut storedData = match select!(StorageData "WHERE key = ?", key) {
        Ok(data) => data,
        Err(_) => StorageData::default(),
    };

    if storedData.rowid.is_none() {
        let newData = StorageData {
            key: Some(key),
            value: Some(value),
            expiry: Some(currentTimestamp + ttl as u32),
            ..Default::default()
        }.insert();
    } else {
        storedData.expiry = Some(currentTimestamp + ttl as u32);
        storedData.value = Some(value);
        storedData.update();
    }
    Ok(cx.boolean(true))
}

fn getItem(mut cx: FunctionContext) -> JsResult<JsString> {
    let key = cx
        .argument::<JsString>(0)?
        .value();
    let currentTimestamp: u32 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as u32;

    let storedData = match select!(StorageData "WHERE key = ?", key) {
        Ok(data) => data,
        Err(_) => StorageData::default(),
    };

    if storedData.rowid.is_none() {
        return Ok(cx.string(""));
    }

    if storedData.expiry.is_some() && storedData.expiry.unwrap() < currentTimestamp {
        // Expired
        storedData.delete();
        return Ok(cx.string(""));
    }

    Ok(cx.string(storedData.value.unwrap()))
}

fn removeItem(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let key = cx
        .argument::<JsString>(0)?
        .value();

    let storedData = match select!(StorageData "WHERE key = ?", key) {
        Ok(data) => data,
        Err(_) => StorageData::default(),
    };

    if storedData.rowid.is_some() {
        storedData.delete();
    }

    Ok(cx.undefined())
}

fn deleteExpiredItems(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let currentTimestamp: u32 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as u32;

    let _ = execute!("DELETE FROM StorageData WHERE expiry < ?", currentTimestamp);

    Ok(cx.undefined())
}

register_module!(mut cx, {
    cx.export_function("setItem", setItem);
    cx.export_function("getItem", getItem);
    cx.export_function("deleteExpiredItems", deleteExpiredItems);
    cx.export_function("removeItem", removeItem);
    Ok(())
});