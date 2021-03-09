//! Helper definitions provided by the `fake` crate.
//!
//! This is a central point for all helpers constructed using
//! the `fake` crate. Almost everything in this module will be
//! generated via macros to delegate through to that crate.
use super::BoxedHelper;
use fake::*;
use tera::{Result, Value};

use std::collections::HashMap;

/// Returns all helpers constructed using the `fake` package.
pub fn helpers() -> Vec<(&'static str, BoxedHelper)> {
    vec![
        ("bool", Box::new(bool)),
        ("city", Box::new(city)),
        ("company", Box::new(company)),
        ("domain", Box::new(domain)),
        ("email", Box::new(email)),
        ("firstName", Box::new(first_name)),
        ("industry", Box::new(industry)),
        ("lastName", Box::new(last_name)),
        ("latitude", Box::new(latitude)),
        ("longitude", Box::new(longitude)),
        ("name", Box::new(name)),
        ("phone", Box::new(phone)),
        ("postcode", Box::new(postcode)),
        ("profession", Box::new(profession)),
        ("sentence", Box::new(sentence)),
        ("state", Box::new(state)),
        ("stateCode", Box::new(state_code)),
        ("street", Box::new(street)),
        ("title", Box::new(title)),
        ("userAgent", Box::new(user_agent)),
        ("username", Box::new(username)),
        ("word", Box::new(word)),
        ("zip", Box::new(zip)),
    ]
}

// Automatic `fake` delegation
macro_rules! fake_delegate {
    ($name:ident, $delegate:expr) => {
        fn $name(_args: &HashMap<String, Value>) -> Result<Value> {
            Ok(Value::from($delegate))
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

// Location based information and functionality
fake_delegate!(city, fake!(Address.city));
fake_delegate!(latitude, fake!(Address.latitude));
fake_delegate!(longitude, fake!(Address.longitude));
fake_delegate!(postcode, fake!(Address.postcode));
fake_delegate!(state, fake!(Address.state));
fake_delegate!(state_code, fake!(Address.state_abbr));
fake_delegate!(street, fake!(Address.street_name));
fake_delegate!(zip, fake!(Address.zip));
