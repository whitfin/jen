//! Helper definitions provided by custom implementation.
use fake::fake;
use objectid::ObjectId;
use rand::Rng;
use tera::{GlobalFn, Value};
use uuid::Uuid;

use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

/// Global counter used to track the current document index.
static GLOBAL_INDEX: AtomicUsize = AtomicUsize::new(0);

/// Returns various custom helpers across various domains.
pub fn helpers() -> Vec<(&'static str, GlobalFn)> {
    vec![
        ("float", float()),
        ("index", index()),
        ("integer", integer()),
        ("objectId", object_id()),
        ("random", random()),
        ("timestamp", timestamp()),
        ("uuid", uuid()),
    ]
}

/// Constructs a helper that will return a float between two bounds.
fn float() -> GlobalFn {
    Box::new(|args| {
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
    })
}

/// Constructs a helper to retrieve the current document index.
fn index() -> GlobalFn {
    Box::new(|_| {
        let idx = GLOBAL_INDEX.fetch_add(1, Ordering::SeqCst);
        let val = Value::from(idx);

        Ok(val)
    })
}

/// Constructs a helper that will return an int between two bounds.
fn integer() -> GlobalFn {
    Box::new(|args| {
        let lower = args
            .get("start")
            .and_then(|value| value.as_i64())
            .unwrap_or(std::i64::MIN);
        let upper = args
            .get("end")
            .and_then(|value| value.as_i64())
            .unwrap_or(std::i64::MIN);

        let value = fake!(Number.between(lower, upper));
        let value = Value::from(value);

        Ok(value)
    })
}

/// Constructs a helper to generate a random object identifier.
fn object_id() -> GlobalFn {
    Box::new(|_| {
        ObjectId::new()
            .map_err(|_| unreachable!())
            .map(|id| id.to_string())
            .map(Value::String)
    })
}

/// Constructs a helper to choose a random value from a set.
fn random() -> GlobalFn {
    Box::new(|mut args| {
        let values = args
            .get_mut("values")
            .expect("must provide values alongside random")
            .as_array_mut()
            .expect("must provide values alongside random");

        let rng = rand::thread_rng().gen_range(0, values.len());
        let val = values[rng].take();

        Ok(val)
    })
}

/// Constructs a helper to generate a random timestamp.
fn timestamp() -> GlobalFn {
    Box::new(|_| {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        let value = fake!(Number.between(0, now));
        let value = Value::from(value);

        Ok(value)
    })
}

/// Construct a helper to generate a random UUID v4.
fn uuid() -> GlobalFn {
    Box::new(|_| {
        let uuid = Uuid::new_v4();
        let json = Value::from(uuid.to_string());

        Ok(json)
    })
}
