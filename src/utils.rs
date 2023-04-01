use chrono::NaiveTime;

/// 00時00分00秒のNaiveTimeを返します
///
pub(crate) fn naive_time_from_zero() -> NaiveTime {
    NaiveTime::from_hms_opt(0, 0, 0).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 意図通りの値を取得できるか
    #[test]
    fn get_naive_time_from_zero() {
        let zero = naive_time_from_zero();
        assert_eq!(zero.format("%H:%M:%S").to_string(), "00:00:00");
    }
}
