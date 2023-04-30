//! Helper definitions provided by custom implementation.
use super::BoxedHelper;
use bson::oid::ObjectId;
use fake::fake;
use nanoid::nanoid;
use rand::Rng;
use tera::{Result, Value};
use uuid::Uuid;

use std::collections::HashMap;
use std::convert::TryInto;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

/// Global counter used to track the current document index.
static GLOBAL_INDEX: AtomicUsize = AtomicUsize::new(0);

/// Returns various custom helpers across various domains.
pub fn helpers() -> Vec<(&'static str, BoxedHelper)> {
    vec![
        ("float", Box::new(float)),
        ("index", Box::new(index)),
        ("integer", Box::new(integer)),
        ("nanoid", Box::new(nanoid)),
        ("objectId", Box::new(object_id)),
        ("paragraph", Box::new(paragraph)),
        ("random", Box::new(random)),
        ("timestamp", Box::new(timestamp)),
        ("uuid", Box::new(uuid)),
    ]
}

/// Generates a float value between two bounds.
///
/// The bounds can be provided via the "start" and "end" arguments
/// in the provided mapping. If not provided, these values will be
/// set to `f64::MIN` and `f64::MAX`.
fn float(args: &HashMap<String, Value>) -> Result<Value> {
    let lower = args
        .get("start")
        .and_then(|value| value.as_f64())
        .unwrap_or(std::f64::MIN);
    let upper = args
        .get("end")
        .and_then(|value| value.as_f64())
        .unwrap_or(std::f64::MIN);

    let value = fake!(Number.between(lower, upper));
    let value = Value::from(value);

    Ok(value)
}

/// Returns the current document index based on the generated data.
fn index(_args: &HashMap<String, Value>) -> Result<Value> {
    let idx = GLOBAL_INDEX.fetch_add(1, Ordering::SeqCst);
    let val = Value::from(idx);

    Ok(val)
}

/// Generates a float value between two bounds.
///
/// The bounds can be provided via the "start" and "end" arguments
/// in the provided mapping. If not provided, these values will be
/// set to `i64::MIN` and `i64::MAX`.
fn integer(args: &HashMap<String, Value>) -> Result<Value> {
    let lower = args
        .get("start")
        .and_then(|value| value.as_i64())
        .unwrap_or(std::i64::MIN);
    let upper = args
        .get("end")
        .and_then(|value| value.as_i64())
        .unwrap_or(std::i64::MAX);

    let value = fake!(Number.between(lower, upper));
    let value = Value::from(value);

    Ok(value)
}

/// Generates a nanoid as a `String`.
fn nanoid(args: &HashMap<String, Value>) -> Result<Value> {
    let length = args
        .get("length")
        .and_then(|value| value.as_u64())
        .and_then(|value| value.try_into().ok())
        .unwrap_or(21);

    Ok(Value::from(nanoid!(length)))
}

/// Generates an object identifier as a `String`.
fn object_id(_args: &HashMap<String, Value>) -> Result<Value> {
    Ok(Value::from(ObjectId::new().to_hex()))
}

/// Generates a paragraph of textual content.
///
/// This is generally sourced by the `fake` crate, but lives
/// in this module as it's more than just a simple delegate.
fn paragraph(_args: &HashMap<String, Value>) -> Result<Value> {
    let sentences = fake!(Lorem.sentences(7)).join(" ");
    let value = Value::from(sentences);

    Ok(value)
}

/// Chooses a random value from a set of values.
///
/// Values must be provided via the "values" argument. If no
/// values are provided, this function will panic.
fn random(args: &HashMap<String, Value>) -> Result<Value> {
    let values = args
        .get("values")
        .expect("must provide values alongside random")
        .as_array()
        .expect("must provide values alongside random");

    let rng = rand::thread_rng().gen_range(0..values.len());
    let val = values[rng].to_owned();

    Ok(val)
}

/// Generates a random timestamp as a number of seconds.
///
/// This is very similar to `integer`, except that the upper
/// bound is automatically set to the current timestamp.
fn timestamp(_args: &HashMap<String, Value>) -> Result<Value> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    let value = fake!(Number.between(0, now));
    let value = Value::from(value);

    Ok(value)
}

/// Generates a random UUID v4 as a hexidecimal `String`.
fn uuid(_args: &HashMap<String, Value>) -> Result<Value> {
    let uuid = Uuid::new_v4();
    let json = Value::from(uuid.to_string());

    Ok(json)
}
