use super::models::{HouseholdForm, MemberForm};
use chrono::NaiveDate;

pub trait Validate {
    fn validate(&self) -> Result<(), String>;
}

impl Validate for HouseholdForm {
    fn validate(&self) -> Result<(), String> {
        if self.head_name.trim().is_empty() {
            return Err("户主姓名不能为空".to_string());
        }
        
        if self.id_number.trim().is_empty() {
            return Err("身份证号不能为空".to_string());
        }
        
        if self.id_number.len() != 18 {
            return Err("身份证号必须是18位".to_string());
        }
        
        if !validate_id_number(&self.id_number) {
            return Err("身份证号格式不正确".to_string());
        }
        
        if self.address.trim().is_empty() {
            return Err("地址不能为空".to_string());
        }
        
        if !self.phone.trim().is_empty() && !validate_phone(&self.phone) {
            return Err("手机号格式不正确".to_string());
        }
        
        if self.members.is_empty() {
            return Err("至少需要一个家庭成员".to_string());
        }
        
        for (i, member) in self.members.iter().enumerate() {
            if let Err(e) = member.validate() {
                return Err(format!("第{}个成员: {}", i + 1, e));
            }
        }
        
        Ok(())
    }
}

impl Validate for MemberForm {
    fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("姓名不能为空".to_string());
        }
        
        if self.id_number.trim().is_empty() {
            return Err("身份证号不能为空".to_string());
        }
        
        if self.id_number.len() != 18 {
            return Err("身份证号必须是18位".to_string());
        }
        
        if !validate_id_number(&self.id_number) {
            return Err("身份证号格式不正确".to_string());
        }
        
        if self.birth_year < 1900 || self.birth_year > 2024 {
            return Err("出生年份不合理".to_string());
        }
        
        if self.birth_month < 1 || self.birth_month > 12 {
            return Err("出生月份必须在1-12之间".to_string());
        }
        
        if self.birth_day < 1 || self.birth_day > 31 {
            return Err("出生日期必须在1-31之间".to_string());
        }
        
        // 验证日期是否有效
        if NaiveDate::from_ymd_opt(self.birth_year, self.birth_month, self.birth_day).is_none() {
            return Err("无效的出生日期".to_string());
        }
        
        Ok(())
    }
}

fn validate_id_number(id: &str) -> bool {
    // 简单的身份证号验证
    if id.len() != 18 {
        return false;
    }
    
    // 前17位必须是数字
    for (i, c) in id.chars().enumerate() {
        if i < 17 && !c.is_ascii_digit() {
            return false;
        }
    }
    
    // 最后一位可以是数字或X
    let last_char = id.chars().last().unwrap();
    last_char.is_ascii_digit() || last_char == 'X' || last_char == 'x'
}

fn validate_phone(phone: &str) -> bool {
    // 简单的手机号验证
    if phone.len() != 11 {
        return false;
    }
    
    // 必须以1开头
    if !phone.starts_with('1') {
        return false;
    }
    
    // 全部是数字
    phone.chars().all(|c| c.is_ascii_digit())
}
