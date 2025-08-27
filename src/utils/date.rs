use chrono::{Datelike, NaiveDate, NaiveDateTime, Timelike};

pub struct DateUtils;

impl DateUtils {
    /// 格式化日期为中文格式
    pub fn format_date_chinese(date: &NaiveDate) -> String {
        format!("{}年{}月{}日", date.year(), date.month(), date.day())
    }
    
    /// 格式化日期时间为中文格式
    pub fn format_datetime_chinese(datetime: &NaiveDateTime) -> String {
        format!(
            "{}年{}月{}日 {}时{}分",
            datetime.year(),
            datetime.month(),
            datetime.day(),
            datetime.hour(),
            datetime.minute()
        )
    }
    
    /// 计算年龄
    pub fn calculate_age(birth_date: &NaiveDate) -> i32 {
        let today = chrono::Utc::now().naive_utc().date();
        let mut age = today.year() - birth_date.year();
        
        // 如果还没过生日，年龄减1
        if today.month() < birth_date.month() || 
           (today.month() == birth_date.month() && today.day() < birth_date.day()) {
            age -= 1;
        }
        
        age
    }
    
    /// 验证日期是否合理
    pub fn validate_birth_date(year: i32, month: u32, day: u32) -> Result<NaiveDate, String> {
        let today = chrono::Utc::now().naive_utc().date();
        
        if year < 1900 {
            return Err("出生年份不能早于1900年".to_string());
        }
        
        if year > today.year() {
            return Err("出生年份不能晚于当前年份".to_string());
        }
        
        if month < 1 || month > 12 {
            return Err("月份必须在1-12之间".to_string());
        }
        
        if day < 1 || day > 31 {
            return Err("日期必须在1-31之间".to_string());
        }
        
        NaiveDate::from_ymd_opt(year, month, day)
            .ok_or_else(|| "无效的日期".to_string())
    }
}
