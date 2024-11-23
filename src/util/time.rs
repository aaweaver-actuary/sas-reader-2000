use chrono::{Duration, NaiveDate, NaiveDateTime};

pub fn get_sas_epoch() -> NaiveDateTime {
    NaiveDate::from_ymd_opt(1960, 1, 1)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap()
}

pub fn sas_timestamp_to_datetime(timestamp: f64) -> NaiveDateTime {
    let sas_epoch = get_sas_epoch();
    let seconds = Duration::seconds(timestamp as i64);
    sas_epoch + seconds
}

pub fn datetime_to_sas_timestamp(datetime: NaiveDateTime) -> f64 {
    let sas_epoch = get_sas_epoch();
    let duration = datetime.signed_duration_since(sas_epoch);
    duration.num_seconds() as f64
}

#[cfg(test)]

mod tests {

    use super::*;
    use assert_approx_eq::assert_approx_eq;
    use chrono::Timelike;

    #[test]
    fn can_get_sas_epoch() {
        let sas_epoch = get_sas_epoch();
        assert_approx_eq!(
            sas_epoch.and_utc().timestamp_subsec_nanos() as f64,
            NaiveDate::from_ymd_opt(1960, 1, 1)
                .unwrap()
                .and_hms_milli_opt(0, 0, 0, 0)
                .unwrap()
                .and_utc()
                .timestamp_subsec_nanos() as f64
        );
    }

    #[test]
    fn can_convert_sas_timestamp_to_datetime() {
        let timestamp = 0.0;
        let datetime = sas_timestamp_to_datetime(timestamp);
        assert_approx_eq!(
            datetime.and_utc().timestamp_subsec_nanos() as f64,
            NaiveDate::from_ymd_opt(1960, 1, 1)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap()
                .and_utc()
                .timestamp_subsec_nanos() as f64
        );
    }

    #[test]
    fn can_convert_datetime_to_sas_timestamp() {
        let datetime = NaiveDate::from_ymd_opt(1960, 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        let timestamp = datetime_to_sas_timestamp(datetime);
        assert_eq!(timestamp, 0.0);
    }

    #[test]
    fn can_convert_sas_timestamp_to_datetime_with_time() {
        let one_nano_after_epoch = 0.000000001;
        let datetime = sas_timestamp_to_datetime(one_nano_after_epoch);

        let expected = NaiveDate::from_ymd_opt(1960, 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            + Duration::nanoseconds(1);

        assert_approx_eq!(
            datetime.and_utc().second() as f64,
            expected.and_utc().second() as f64
        );
    }

    #[test]
    fn can_convert_datetime_to_sas_timestamp_with_time() {
        let datetime = NaiveDate::from_ymd_opt(1960, 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            + Duration::nanoseconds(100);
        let timestamp = datetime_to_sas_timestamp(datetime);
        assert_approx_eq!(timestamp, 0.0000001);
    }
}
