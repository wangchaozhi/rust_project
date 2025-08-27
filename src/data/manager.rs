use super::models::*;
use chrono::NaiveDate;
use uuid::Uuid;

pub struct HouseholdManager {
    households: Vec<Household>,
}

impl HouseholdManager {
    pub fn new() -> Self {
        Self {
            households: Vec::new(),
        }
    }
    
    pub fn add_sample_data(&mut self) {
        let sample1 = Household {
            id: Uuid::new_v4(),
            head_name: "张三".to_string(),
            id_number: "110101199001011234".to_string(),
            address: "北京市朝阳区XXX街道XXX号".to_string(),
            phone: "13800138000".to_string(),
            household_type: HouseholdType::Urban,
            registration_date: chrono::Utc::now().naive_utc(),
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
            registration_date: chrono::Utc::now().naive_utc(),
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
        
        self.households.push(sample1);
        self.households.push(sample2);
    }
    
    pub fn get_households(&self) -> &Vec<Household> {
        &self.households
    }
    
    pub fn get_household(&self, index: usize) -> Option<&Household> {
        self.households.get(index)
    }
    
    pub fn add_household(&mut self, household: Household) {
        self.households.push(household);
    }
    
    pub fn update_household(&mut self, index: usize, household: Household) {
        if index < self.households.len() {
            self.households[index] = household;
        }
    }
    
    pub fn remove_household(&mut self, index: usize) {
        if index < self.households.len() {
            self.households.remove(index);
        }
    }
    
    pub fn search(&self, query: &str) -> Vec<usize> {
        if query.is_empty() {
            (0..self.households.len()).collect()
        } else {
            self.households
                .iter()
                .enumerate()
                .filter(|(_, household)| {
                    household.head_name.contains(query) ||
                    household.id_number.contains(query) ||
                    household.address.contains(query) ||
                    household.phone.contains(query)
                })
                .map(|(i, _)| i)
                .collect()
        }
    }
    
    pub fn count(&self) -> usize {
        self.households.len()
    }
    
    pub fn get_statistics(&self) -> HouseholdStatistics {
        let total = self.households.len();
        let urban_count = self.households.iter()
            .filter(|h| h.household_type == HouseholdType::Urban)
            .count();
        let rural_count = total - urban_count;
        
        let total_members = self.households.iter()
            .map(|h| h.members.len())
            .sum();
        
        HouseholdStatistics {
            total_households: total,
            urban_households: urban_count,
            rural_households: rural_count,
            total_members,
        }
    }
}

#[derive(Debug)]
pub struct HouseholdStatistics {
    pub total_households: usize,
    pub urban_households: usize,
    pub rural_households: usize,
    pub total_members: usize,
}
