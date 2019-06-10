use fake::*;
use objectid::ObjectId;
use rand::Rng;
use tera::{Result, Value};
use uuid::Uuid;

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use std::sync::atomic::{AtomicUsize, Ordering};

static GLOBAL_INDEX: AtomicUsize = AtomicUsize::new(0);

pub type Helper = Box<Fn(HashMap<String, Value>) -> Result<Value> + Send + Sync>;

pub fn all() -> Vec<(&'static str, Helper)> {
    vec![
        ("bool", bool()),
        ("city", city()),
        ("company", company()),
        ("domain", domain()),
        ("email", email()),
        ("firstName", first_name()),
        ("float", float()),
        ("index", index()),
        ("industry", industry()),
        ("integer", integer()),
        ("lastName", last_name()),
        ("latitude", latitude()),
        ("longitude", longitude()),
        ("name", name()),
        ("objectId", id()),
        ("paragraph", paragraph()),
        ("phone", phone()),
        ("postcode", postcode()),
        ("profession", profession()),
        ("random", random()),
        ("sentence", sentence()),
        ("state", state()),
        ("stateCode", state_code()),
        ("street", street()),
        ("timestamp", timestamp()),
        ("title", title()),
        ("word", word()),
        ("username", user_name()),
        ("userAgent", user_agent()),
        ("uuid", uuid()),
        ("zip", zip()),
    ]
}

macro_rules! fake_delegate {
    ($name:ident, $delegate:expr) => {
        pub fn $name() -> Helper {
            Box::new(|_| Ok(Value::from($delegate)))
        }
    };
}

// Generic functionality
fake_delegate!(bool, fake!(Boolean.boolean));

// Company based information and functionality
fake_delegate!(domain, fake!(Internet.domain_suffix));
fake_delegate!(company, fake!(Company.name));
fake_delegate!(industry, fake!(Company.industry));
fake_delegate!(profession, fake!(Company.profession));

// User based information and functionality
fake_delegate!(name, fake!(Name.name));
fake_delegate!(title, fake!(Name.title));
fake_delegate!(email, fake!(Internet.free_email));
fake_delegate!(phone, fake!(PhoneNumber.cell_number));
fake_delegate!(first_name, fake!(Name.first_name));
fake_delegate!(last_name, fake!(Name.last_name));
fake_delegate!(user_name, fake!(Internet.user_name));
fake_delegate!(user_agent, fake!(Internet.user_agent));

// Lorem based information and functionality (random words)
fake_delegate!(word, fake!(Lorem.word));
fake_delegate!(sentence, fake!(Lorem.sentence(4, 6)));
fn paragraph() -> Helper {
    Box::new(|_| {
        let sentences = fake!(Lorem.sentences(7)).join(" ");
        let value = Value::from(sentences);

        Ok(value)
    })
}

// Location based information and functionality
fake_delegate!(city, fake!(Address.city));
fake_delegate!(latitude, fake!(Address.latitude));
fake_delegate!(longitude, fake!(Address.longitude));
fake_delegate!(postcode, fake!(Address.postcode));
fake_delegate!(state, fake!(Address.state));
fake_delegate!(state_code, fake!(Address.state_abbr));
fake_delegate!(street, fake!(Address.street_name));
fake_delegate!(zip, fake!(Address.zip));

fn float() -> Helper {
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

fn id() -> Helper {
    Box::new(|_| {
        ObjectId::new()
            .map_err(|_| unreachable!())
            .map(|id| id.to_string())
            .map(Value::String)
    })
}

fn index() -> Helper {
    Box::new(|_| {
        let idx = GLOBAL_INDEX.fetch_add(1, Ordering::SeqCst);
        let val = Value::from(idx);

        Ok(val)
    })
}

fn integer() -> Helper {
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

fn random() -> Helper {
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

fn timestamp() -> Helper {
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

fn uuid() -> Helper {
    Box::new(|_| {
        let uuid = Uuid::new_v4();
        let json = Value::from(uuid.to_string());

        Ok(json)
    })
}
