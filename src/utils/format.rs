/// 格式化身份证号，隐藏中间部分
pub fn format_id_number_masked(id_number: &str) -> String {
    if id_number.len() != 18 {
        return id_number.to_string();
    }
    
    format!(
        "{}****{}",
        &id_number[0..6],
        &id_number[14..18]
    )
}

/// 格式化手机号，隐藏中间部分
pub fn format_phone_masked(phone: &str) -> String {
    if phone.len() != 11 {
        return phone.to_string();
    }
    
    format!(
        "{}****{}",
        &phone[0..3],
        &phone[7..11]
    )
}

/// 格式化地址，如果太长则截断
pub fn format_address_truncated(address: &str, max_length: usize) -> String {
    if address.len() <= max_length {
        address.to_string()
    } else {
        format!("{}...", &address[0..max_length])
    }
}

/// 格式化姓名，确保正确显示
pub fn format_name(name: &str) -> String {
    name.trim().to_string()
}

/// 格式化数字为中文计数
pub fn format_count_chinese(count: usize) -> String {
    match count {
        0 => "零".to_string(),
        1 => "一".to_string(),
        2 => "二".to_string(),
        3 => "三".to_string(),
        4 => "四".to_string(),
        5 => "五".to_string(),
        6 => "六".to_string(),
        7 => "七".to_string(),
        8 => "八".to_string(),
        9 => "九".to_string(),
        10 => "十".to_string(),
        _ => count.to_string(),
    }
}
