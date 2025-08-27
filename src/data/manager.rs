use super::models::*;
use super::database::Database;
use chrono::NaiveDate;
use uuid::Uuid;
use std::collections::HashMap;

pub struct HouseholdManager {
    database: Database,
    households_cache: HashMap<Uuid, Household>,
    cache_dirty: bool,
}

impl HouseholdManager {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let database = Database::new("household_management.db")?;
        Ok(Self {
            database,
            households_cache: HashMap::new(),
            cache_dirty: true,
        })
    }
    
    pub fn is_empty(&self) -> Result<bool, Box<dyn std::error::Error>> {
        Ok(self.database.is_empty()?)
    }
    
    pub fn add_sample_data(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sample1 = Household {
            id: Uuid::new_v4(),
            head_name: "张三".to_string(),
            id_number: "110101199001011234".to_string(),
            address: "北京市朝阳区XXX街道XXX号".to_string(),
            phone: "13800138000".to_string(),
            household_type: HouseholdType::Urban,
            registration_date: chrono::NaiveDateTime::new(
                chrono::NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
                chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap()
            ),
            members: vec![
                Member {
                    name: "张三".to_string(),
                    id_number: "110101199001011234".to_string(),
                    relationship: Relationship::Head,
                    birth_date: NaiveDate::from_ymd_opt(1990, 1, 1).unwrap(),
                    gender: Gender::Male,
                    education: Education::University,
                    occupation: "工程师".to_string(),
                },
                Member {
                    name: "李四".to_string(),
                    id_number: "110101199205051235".to_string(),
                    relationship: Relationship::Spouse,
                    birth_date: NaiveDate::from_ymd_opt(1992, 5, 5).unwrap(),
                    gender: Gender::Female,
                    education: Education::University,
                    occupation: "教师".to_string(),
                },
            ],
        };
        
        let sample2 = Household {
            id: Uuid::new_v4(),
            head_name: "王五".to_string(),
            id_number: "110101198506061236".to_string(),
            address: "北京市海淀区YYY街道YYY号".to_string(),
            phone: "13900139000".to_string(),
            household_type: HouseholdType::Rural,
            registration_date: chrono::NaiveDateTime::new(
                chrono::NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
                chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap()
            ),
            members: vec![
                Member {
                    name: "王五".to_string(),
                    id_number: "110101198506061236".to_string(),
                    relationship: Relationship::Head,
                    birth_date: NaiveDate::from_ymd_opt(1985, 6, 6).unwrap(),
                    gender: Gender::Male,
                    education: Education::HighSchool,
                    occupation: "农民".to_string(),
                },
            ],
        };
        
        self.database.insert_household(&sample1)?;
        self.database.insert_household(&sample2)?;
        self.cache_dirty = true;
        Ok(())
    }
    
    fn refresh_cache(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.cache_dirty {
            match self.database.get_all_households() {
                Ok(households) => {
                    self.households_cache.clear();
                    for household in households {
                        self.households_cache.insert(household.id, household);
                    }
                    self.cache_dirty = false;
                }
                Err(_) => {
                    // 如果数据库为空或出错，清空缓存
                    self.households_cache.clear();
                    self.cache_dirty = false;
                    // 不返回错误，允许继续运行
                }
            }
        }
        Ok(())
    }
    
    pub fn get_households(&mut self) -> Result<Vec<Household>, Box<dyn std::error::Error>> {
        self.refresh_cache()?;
        Ok(self.households_cache.values().cloned().collect())
    }
    
    pub fn get_household(&mut self, index: usize) -> Result<Option<Household>, Box<dyn std::error::Error>> {
        self.refresh_cache()?;
        let households: Vec<_> = self.households_cache.values().cloned().collect();
        Ok(households.get(index).cloned())
    }
    
    pub fn add_household(&mut self, household: Household) -> Result<(), Box<dyn std::error::Error>> {
        self.database.insert_household(&household)?;
        self.cache_dirty = true;
        Ok(())
    }
    
    pub fn update_household(&mut self, household: Household) -> Result<(), Box<dyn std::error::Error>> {
        self.database.update_household(&household)?;
        self.cache_dirty = true;
        Ok(())
    }
    
    pub fn remove_household(&mut self, household_id: &Uuid) -> Result<(), Box<dyn std::error::Error>> {
        self.database.delete_household(household_id)?;
        self.cache_dirty = true;
        Ok(())
    }
    
    pub fn search(&mut self, query: &str) -> Result<Vec<usize>, Box<dyn std::error::Error>> {
        if query.is_empty() {
            self.refresh_cache()?;
            let count = self.households_cache.len();
            Ok((0..count).collect())
        } else {
            let households = self.database.search_households(query)?;
            let all_households = self.get_households()?;
            
            let mut indices = Vec::new();
            for (i, household) in all_households.iter().enumerate() {
                if households.iter().any(|h| h.id == household.id) {
                    indices.push(i);
                }
            }
            Ok(indices)
        }
    }
    
    pub fn count(&mut self) -> Result<usize, Box<dyn std::error::Error>> {
        self.refresh_cache()?;
        Ok(self.households_cache.len())
    }
    
    pub fn get_statistics(&mut self) -> Result<HouseholdStatistics, Box<dyn std::error::Error>> {
        Ok(self.database.get_statistics()?)
    }
}

#[derive(Debug)]
pub struct HouseholdStatistics {
    pub total_households: usize,
    pub urban_households: usize,
    pub rural_households: usize,
    pub total_members: usize,
}
