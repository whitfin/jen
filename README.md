# Jen
[![Build Status](https://img.shields.io/github/actions/workflow/status/whitfin/jen/ci.yml)](https://github.com/whitfin/jen/actions)
[![Crates.io](https://img.shields.io/crates/v/jen.svg)](https://crates.io/crates/jen)

A simple (but extensible) tool for generating large random datasets.

Jen is a combination of a core library and a CLI, used to generate random
datasets based on a template. There are existing tools for this purpose,
but most of them live in a browser and they're naturally insufficient when
it comes to generating large amounts of data. Jen was created to fill the
niche of creating larger amounts of data for things like unit tests and
database state.

Jen's underlying template syntax is drive by [Tera](https://github.com/Keats/tera)
to aid in familiarity and to avoid re-inventing a templating language. On
top of this layer, Jen offers many helpers based around randomizing data.
Several of these helpers are based on [fake](https://github.com/cksac/fake-rs),
with additional helpers provided where there are gaps. You can naturally
attach your own helpers when using Jen programmatically.

### Installation

Jen will be available via [Crates.io](https://crates.io/crates/jen), so it
can be installed from there directly. You can use Jen either as a command
line utility, or directly via the programmatic API.

If you wish to install Jen as a command line utility, you can install it
via an easy one-liner in your terminal:

```shell
$ cargo install jen
```

If you wish to use it as a library, you can add it to your `Cargo.toml`
as a dependency of your application:

```toml
[dependencies]
jen = { version = "1.7", default-features = false }
```

You should disable the default features as it includes several dependencies
which are required for the CLI use case. These dependencies are not included
in your application when these features are disabled.

### Usage

The first step is to construct a template file which Jen will then use when
generating data.  There are many template helpers provided by default, via
the internal `jen::helpers` module. You can check the documentation for the
latest list of helpers, although a fairly up to date table of helpers can
be found [below](#template-helpers). Once you have this template, you can
either use Jen via the CLI, or programmatically.

#### Command Line

The CLI is fairly simple, with a basic structure of:

```shell
$ jen <template>
```

Using this syntax will generate a random document based on the provided
template (which must be a valid Tera template). There is a basic document
you can test with inside the `example` directory.

There are various switches you can provide to adjust the output, including
the following (at the time of writing, there may be more):

```text
USAGE:
    jen [FLAGS] [OPTIONS] <template>

FLAGS:
    -h, --help       Prints help information
    -t, --textual    Treat the input as textual, rather than JSON
    -V, --version    Prints version information

OPTIONS:
    -l, --limit <limit>        An upper limit of documents to generate

ARGS:
    <template>    Template to control JSON generation
```

Although Jen works with any input formatting, it was originally written
with JSON documents in mind. As such, Jen will automically attempt to
parse input (after generation) as JSON, in order to compact and emit
documents once per line. This detection has a fair amount of overhead,
which can be skipped by providing `-t` if you explicitly don't want to
treat incoming data as JSON.

For a complete and up to date list of options, please use `jen -h` in your
terminal.

#### Programmatic API

The programmatic API is pretty simple. Everything is handled through the
use of the `Generator` struct, which implements the `Iterator` trait to
provide continuous (lazy) documents.

Generators are constructed using a path to a template on disk, and you
then generate documents using the `Iterator` methods, as shown below.

```rust
let mut generator = Generator::load("./example/example.tera")
    .expect("provided a value template");

for document in generator.take(5) {
    println!("{}", document);
}
```

This will generate five documents from the provided template and print
them to the terminal.

### Template Helpers

Below is a list of current helpers (at the time of writing). Please see the
documentation uploaded to Crates.io for an up-to-date listing.

| Helper                                 | Description                                           |
|----------------------------------------|-------------------------------------------------------|
| bool()                                 | Generates a random boolean value                      |
| city()                                 | Generates a random city name                          |
| company()                              | Generates a random company name                       |
| domain()                               | Generates a random domain name                        |
| email()                                | Generates a random email address                      |
| firstName()                            | Generates a random first name                         |
| float(start=f64::MIN, end=f64::MAX)    | Generates a random float value between two bounds     |
| index()                                | Retrieves the current index of the generated document |
| industry()                             | Generates a random industry type                      |
| integer(start=i64::MIN, end=i64::MAX)  | Generates a random integer value between two bounds   |
| lastName()                             | Generates a random last name                          |
| latitude()                             | Generates a random latitude location value            |
| longitude()                            | Generates a random longitude location value           |
| name()                                 | Generates a random full name                          |
| nanoid(length=21)                      | Generates a random nanoid of a given length           |
| objectId()                             | Generates a random object identifier                  |
| paragraph()                            | Generates a random paragraph of Lorem Ipsum           |
| phone()                                | Generates a random phone number                       |
| postcode()                             | Generates a random postcode value                     |
| profession()                           | Generates a random job profession                     |
| random(values=["red","blue","yellow"]) | Retrieves a random value from the provided values     |
| sentence()                             | Generates a random sentence of Lorem Ipsum            |
| state()                                | Retrieves a random US state name                      |
| stateCode()                            | Retrieves a random US state code                      |
| street()                               | Generates a random street name                        |
| timestamp()                            | Generates a random timestamp value in seconds         |
| title()                                | Generates a random job title                          |
| userAgent()                            | Generates a random browser user agent                 |
| username()                             | Generates a random account username                   |
| uuid()                                 | Generates a v4 UUID                                   |
| word()                                 | Retrieves a random word of Lorem Ipsum                |
| zip()                                  | Generates a random US zip code                        |
