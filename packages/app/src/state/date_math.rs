use std::sync::atomic::{AtomicBool, AtomicI64, Ordering};

pub const MS_PER_DAY: i64 = 86_400_000;
pub const MS_PER_HOUR: i64 = 3_600_000;
pub const MS_PER_MINUTE: i64 = 60_000;

static LOCAL_OFFSET_MS: AtomicI64 = AtomicI64::new(0);
static LOCAL_OFFSET_READY: AtomicBool = AtomicBool::new(false);

pub fn set_local_offset_ms(offset_ms: i64) {
  LOCAL_OFFSET_MS.store(offset_ms, Ordering::Relaxed);
  LOCAL_OFFSET_READY.store(true, Ordering::Relaxed);
}

#[cfg(not(target_arch = "wasm32"))]
pub fn local_offset_ready() -> bool {
  LOCAL_OFFSET_READY.load(Ordering::Relaxed)
}

pub fn local_offset_ms() -> i64 {
  LOCAL_OFFSET_MS.load(Ordering::Relaxed)
}

pub fn offset_ms_from_timezone_offset_minutes(minutes: i64) -> i64 {
  -minutes * MS_PER_MINUTE
}

pub fn utc_ms_to_local_ms(utc_ms: i64) -> i64 {
  utc_ms + local_offset_ms()
}

pub fn days_from_civil(y: i32, m: u32, d: u32) -> i64 {
  let y = y as i64 - if m <= 2 { 1 } else { 0 };
  let era = if y >= 0 { y } else { y - 399 } / 400;
  let yoe = y - era * 400;
  let mp = (m as i64 + if m > 2 { -3 } else { 9 }) % 12;
  let doy = (153 * mp + 2) / 5 + d as i64 - 1;
  let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
  era * 146097 + doe - 719468
}

pub fn civil_from_days(z: i64) -> (i32, u32, u32) {
  let z = z + 719468;
  let era = if z >= 0 { z } else { z - 146096 } / 146097;
  let doe = z - era * 146097;
  let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
  let y = yoe + era * 400;
  let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
  let mp = (5 * doy + 2) / 153;
  let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
  let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
  ((if m <= 2 { y + 1 } else { y }) as i32, m, d)
}

pub fn weekday_index(days_since_epoch: i64) -> u32 {
  (((days_since_epoch + 3) % 7 + 7) % 7) as u32
}

pub fn ymdhm_to_date_ms(y: i32, m: u32, d: u32, h: u32, min: u32) -> i64 {
  days_from_civil(y, m, d) * MS_PER_DAY + h as i64 * MS_PER_HOUR + min as i64 * MS_PER_MINUTE
}

pub fn date_ms_to_ymdhm(ms: i64) -> (i32, u32, u32, u32, u32) {
  let days = ms.div_euclid(MS_PER_DAY);
  let ms_of_day = ms.rem_euclid(MS_PER_DAY);
  let (y, m, d) = civil_from_days(days);
  let h = (ms_of_day / MS_PER_HOUR) as u32;
  let min = ((ms_of_day % MS_PER_HOUR) / MS_PER_MINUTE) as u32;
  (y, m, d, h, min)
}

pub fn add_days(date_ms: i64, delta_days: i64) -> i64 {
  date_ms + delta_days * MS_PER_DAY
}

pub fn day_key(date_ms: i64) -> i64 {
  date_ms.div_euclid(MS_PER_DAY)
}

pub fn day_key_to_ms(day_key: i64) -> i64 {
  day_key * MS_PER_DAY
}

pub fn days_in_month(y: i32, m: u32) -> u32 {
  let this_month_first = days_from_civil(y, m, 1);
  let next_month_first = if m == 12 { days_from_civil(y + 1, 1, 1) } else { days_from_civil(y, m + 1, 1) };
  (next_month_first - this_month_first) as u32
}

pub fn date_ms_to_date_string(ms: i64) -> String {
  let (y, m, d) = civil_from_days(day_key(ms));
  format!("{y:04}-{m:02}-{d:02}")
}

pub fn date_ms_to_time_string(ms: i64) -> String {
  let (_, _, _, h, min) = date_ms_to_ymdhm(ms);
  format!("{h:02}:{min:02}")
}

pub fn date_and_time_strings_to_ms(date: &str, time: &str) -> Option<i64> {
  let mut date_parts = date.splitn(3, '-');
  let y: i32 = date_parts.next()?.parse().ok()?;
  let m: u32 = date_parts.next()?.parse().ok()?;
  let d: u32 = date_parts.next()?.parse().ok()?;

  let mut time_parts = time.splitn(2, ':');
  let h: u32 = time_parts.next()?.parse().ok()?;
  let min: u32 = time_parts.next()?.parse().ok()?;

  Some(ymdhm_to_date_ms(y, m, d, h, min))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn civil_round_trip_across_boundaries() {
    let cases = [
      (1970, 1, 1),
      (2000, 2, 29),
      (2024, 2, 29),
      (2023, 12, 31),
      (2024, 1, 1),
      (1969, 12, 31),
      (1900, 1, 1),
      (2100, 3, 1),
    ];
    for (y, m, d) in cases {
      let days = days_from_civil(y, m, d);
      assert_eq!(civil_from_days(days), (y, m, d));
    }
  }

  #[test]
  fn epoch_fixed_point() {
    assert_eq!(days_from_civil(1970, 1, 1), 0);
  }

  #[test]
  fn weekday_matches_known_thursday() {
    assert_eq!(weekday_index(days_from_civil(1970, 1, 1)), 3);
    assert_eq!(weekday_index(days_from_civil(2024, 1, 1)), 0);
  }

  #[test]
  fn ymdhm_round_trip() {
    let ms = ymdhm_to_date_ms(2026, 7, 29, 14, 35);
    assert_eq!(date_ms_to_ymdhm(ms), (2026, 7, 29, 14, 35));
  }

  #[test]
  fn date_time_string_round_trip() {
    let ms = date_and_time_strings_to_ms("2026-07-29", "09:05").unwrap();
    assert_eq!(date_ms_to_date_string(ms), "2026-07-29");
    assert_eq!(date_ms_to_time_string(ms), "09:05");
  }

  #[test]
  fn timezone_offset_minutes_invert_to_signed_milliseconds() {
    assert_eq!(offset_ms_from_timezone_offset_minutes(300), -5 * MS_PER_HOUR);
    assert_eq!(offset_ms_from_timezone_offset_minutes(-60), MS_PER_HOUR);
    assert_eq!(offset_ms_from_timezone_offset_minutes(0), 0);
    assert_eq!(offset_ms_from_timezone_offset_minutes(-330), 5 * MS_PER_HOUR + 30 * MS_PER_MINUTE);
  }

  #[test]
  fn utc_converts_to_the_matching_local_wall_clock() {
    set_local_offset_ms(offset_ms_from_timezone_offset_minutes(300));
    assert_eq!(local_offset_ms(), -5 * MS_PER_HOUR);

    let utc_ms = ymdhm_to_date_ms(2026, 8, 8, 14, 0);
    assert_eq!(date_ms_to_ymdhm(utc_ms_to_local_ms(utc_ms)), (2026, 8, 8, 9, 0));
  }
}
