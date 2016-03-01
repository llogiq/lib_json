lib_json
---

Allocationless parsing of json files in Rust.

How does it work?

Add it to your Cargo.toml

    [dependencies]
    lib_json = { git = "https://github.com/valarauca/lib_json" }

Import it into your source

    extern crate lib_json;
    use lib_json::Json;
  
Call it: (Returns an option none on parsing error)

    let json = Json::new( s: &str ).unwrap();

Its very easy to work with:

    pub enum Json<'a> {
      Var(&'a str),
      Obj(BTreeMap<&'a str,Json<'a>>),
      Vec(Vec<Json<'a>)
    }
