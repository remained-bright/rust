use crate::args::DIR;
use ini::Ini;
use std::collections::BTreeMap;
use std::path::Path;

use static_init::dynamic;

/*
#[derive(Clone, Default, Deserialize, Serialize, Debug)]
pub struct Config {
  pub udp: String,
  pub ws: String,
}

type TomlDb = FileDatabase<Config, Toml>;

#[dynamic]
pub static CONFIG: TomlDb = futures::executor::block_on(async {
  TomlDb::load_from_path_or_default(p).await.unwrap()
});
*/
/*
pub struct Config<'a> {
  map: &'a mut BTreeMap<String, String>,
}

impl<'a> Config<'a> {
  pub fn get(&self, key: &str) -> Option<&String> {
    self.map.get(key)
  }
  pub fn get_or_default(&self, key: &str, create: fn() -> String) -> String {
    match self.get(key) {
      Some(v) => v.clone(),
      _ => create(),
    }
  }
  pub fn get_or_create(&self, key: &str, create: fn() -> String) -> String {
    match self.get(key) {
      Some(v) => v.clone(),
      _ => {
        let r = create();
        self.map.insert(key.to_string(), r.clone());
        let p = Path::new(&*DIR).join("config.ini");

        r
      }
    }
  }
}
*/

#[dynamic]
pub static mut CONFIG: BTreeMap<String, String> = {
  let mut bt = BTreeMap::new();
  if let Ok(conf) = Ini::load_from_file(Path::new(&*DIR).join("config.ini")) {
    if let Some(section) = conf.section(None::<String>) {
      for (k, v) in section.iter() {
        bt.insert(k.to_owned(), v.to_owned());
      }
    }
  }

  bt
};

pub mod get {
  use crate::args::DIR;
  use ini::Ini;
  use std::path::Path;

  macro_rules! config_get {
    ($name: ident, $key: ident, $create: ident, $created: expr) => {
      pub fn $name(key: &str, create: fn() -> String) -> String {
        let $key = key;
        let val = (match crate::config::CONFIG.read().get(key) {
          Some(v) => v,
          _ => "",
        })
        .to_owned();
        if val.is_empty() {
          let $create = create();
          $created;
          $create
        } else {
          val
        }
      }
    };
  }

  //config_get!(default, _k, v, {});
  config_get!(create, k, val, {
    {
      crate::config::CONFIG
        .write()
        .insert(k.to_owned(), val.to_owned())
    };
    let p = Path::new(&*DIR).join("config.ini");
    let mut conf = match Ini::load_from_file(&p) {
      Ok(v) => v,
      _ => Ini::new(),
    };
    conf.with_section(None::<String>).set(k, &val);
    conf.write_to_file(&p).unwrap();
  });
}

#[macro_export]
macro_rules! config_get {
  /*
  ($func:ident, $key:expr, $default: block) => {
    crate::config::get::$func(stringify!($key), || $default)
  };
  */
  ($key:expr, $default: block) => {
    crate::config::get::create(stringify!($key), || $default)
  };
}
