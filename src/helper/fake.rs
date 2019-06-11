//! Helper definitions provided by the `fake` crate.
//!
//! This is a central point for all helpers constructed using
//! the `fake` crate. Almost everything in this module will be
//! generated via macros to delegate through to that crate.
use fake::*;
use serde_json::Value;
use tera::GlobalFn;

/// Returns all helpers constructed using the `fake` package.
pub fn helpers() -> Vec<(&'static str, GlobalFn)> {
    vec![
        ("bool", bool()),
        ("city", city()),
        ("company", company()),
        ("domain", domain()),
        ("email", email()),
        ("firstName", first_name()),
        ("industry", industry()),
        ("lastName", last_name()),
        ("latitude", latitude()),
        ("longitude", longitude()),
        ("name", name()),
        ("paragraph", paragraph()),
        ("phone", phone()),
        ("postcode", postcode()),
        ("profession", profession()),
        ("sentence", sentence()),
        ("state", state()),
        ("stateCode", state_code()),
        ("street", street()),
        ("title", title()),
        ("userAgent", user_agent()),
        ("username", username()),
        ("word", word()),
        ("zip", zip()),
    ]
}

// Automatic `fake` delegation
macro_rules! fake_delegate {
    ($name:ident, $delegate:expr) => {
        fn $name() -> GlobalFn {
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
fake_delegate!(username, fake!(Internet.user_name));
fake_delegate!(first_name, fake!(Name.first_name));
fake_delegate!(last_name, fake!(Name.last_name));
fake_delegate!(user_agent, fake!(Internet.user_agent));

// Lorem based information and functionality (random words)
fake_delegate!(word, fake!(Lorem.word));
fake_delegate!(sentence, fake!(Lorem.sentence(4, 6)));
fn paragraph() -> GlobalFn {
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
