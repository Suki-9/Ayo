use radix_fmt::radix_36;
use rand::Rng;

static TIME2000: i64 = 946684800000;

pub fn gen_id(time: &i64) -> String {
  format!(
    "{:>08}{:0>2}",
    format!("{:#}", radix_36(*time - TIME2000)),
    format!("{:#}", radix_36(rand::thread_rng().gen_range(0..1296))),
  )
  .to_lowercase()
}
