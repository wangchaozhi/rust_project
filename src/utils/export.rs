use crate::data::models::Household;
use chrono::Datelike;
use std::fs::File;
use std::io::Write;

pub struct ExportUtils;

impl ExportUtils {
    /// 导出户籍数据到CSV文件
    pub fn export_to_csv(households: &[Household], file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = File::create(file_path)?;
        
        // 写入CSV头部
        writeln!(file, "户主姓名,身份证号,户口类型,联系电话,家庭地址,登记日期,成员数量")?;
        
        // 写入数据
        for household in households {
            writeln!(
                file,
                "{},{},{},{},{},{},{}",
                household.head_name,
                household.id_number,
                household.household_type,
                household.phone,
                household.address.replace(',', "，"), // 替换逗号避免CSV格式问题
                household.registration_date.format("%Y-%m-%d"),
                household.members.len()
            )?;
        }
        
        Ok(())
    }
    
    /// 导出成员数据到CSV文件
    pub fn export_members_to_csv(households: &[Household], file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = File::create(file_path)?;
        
        // 写入CSV头部
        writeln!(file, "户主姓名,成员姓名,身份证号,关系,性别,出生日期,学历,职业")?;
        
        // 写入数据
        for household in households {
            for member in &household.members {
                writeln!(
                    file,
                    "{},{},{},{},{},{},{},{}",
                    household.head_name,
                    member.name,
                    member.id_number,
                    member.relationship,
                    member.gender,
                    member.birth_date.format("%Y-%m-%d"),
                    member.education,
                    member.occupation.replace(',', "，")
                )?;
            }
        }
        
        Ok(())
    }
    
    /// 导出统计报告到文本文件
    pub fn export_statistics_report(
        households: &[Household],
        file_path: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = File::create(file_path)?;
        
        let total_households = households.len();
        let urban_count = households.iter()
            .filter(|h| matches!(h.household_type, crate::data::models::HouseholdType::Urban))
            .count();
        let rural_count = total_households - urban_count;
        let total_members: usize = households.iter().map(|h| h.members.len()).sum();
        
        writeln!(file, "户籍管理系统统计报告")?;
        writeln!(file, "========================")?;
        writeln!(file, "生成时间: {}", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S"))?;
        writeln!(file)?;
        writeln!(file, "基本统计:")?;
        writeln!(file, "  总户数: {}", total_households)?;
        writeln!(file, "  城镇户口: {}", urban_count)?;
        writeln!(file, "  农村户口: {}", rural_count)?;
        writeln!(file, "  总人数: {}", total_members)?;
        writeln!(file, "  平均每户人数: {:.2}", if total_households > 0 { total_members as f64 / total_households as f64 } else { 0.0 })?;
        writeln!(file)?;
        
        // 年龄分布统计
        let mut age_groups = [0; 7]; // 0-10, 11-20, 21-30, 31-40, 41-50, 51-60, 60+
        let today = chrono::Utc::now().naive_utc().date();
        
        for household in households {
            for member in &household.members {
                let age = today.year() - member.birth_date.year();
                let group_index = match age {
                    0..=10 => 0,
                    11..=20 => 1,
                    21..=30 => 2,
                    31..=40 => 3,
                    41..=50 => 4,
                    51..=60 => 5,
                    _ => 6,
                };
                age_groups[group_index] += 1;
            }
        }
        
        writeln!(file, "年龄分布:")?;
        writeln!(file, "  0-10岁: {}", age_groups[0])?;
        writeln!(file, "  11-20岁: {}", age_groups[1])?;
        writeln!(file, "  21-30岁: {}", age_groups[2])?;
        writeln!(file, "  31-40岁: {}", age_groups[3])?;
        writeln!(file, "  41-50岁: {}", age_groups[4])?;
        writeln!(file, "  51-60岁: {}", age_groups[5])?;
        writeln!(file, "  60岁以上: {}", age_groups[6])?;
        
        Ok(())
    }
}
