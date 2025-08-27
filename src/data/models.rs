use chrono::{Datelike, NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Household {
    pub id: Uuid,
    pub head_name: String,
    pub id_number: String,
    pub address: String,
    pub phone: String,
    pub household_type: HouseholdType,
    pub registration_date: NaiveDateTime,
    pub members: Vec<Member>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Member {
    pub name: String,
    pub id_number: String,
    pub relationship: Relationship,
    pub birth_date: NaiveDate,
    pub gender: Gender,
    pub education: Education,
    pub occupation: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum HouseholdType {
    Urban,   // 城镇户口
    Rural,   // 农村户口
}

impl std::fmt::Display for HouseholdType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HouseholdType::Urban => write!(f, "城镇户口"),
            HouseholdType::Rural => write!(f, "农村户口"),
        }
    }
}

impl Default for HouseholdType {
    fn default() -> Self {
        HouseholdType::Urban
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Relationship {
    Head,     // 户主
    Spouse,   // 配偶
    Child,    // 子女
    Parent,   // 父母
    Other,    // 其他
}

impl std::fmt::Display for Relationship {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Relationship::Head => write!(f, "户主"),
            Relationship::Spouse => write!(f, "配偶"),
            Relationship::Child => write!(f, "子女"),
            Relationship::Parent => write!(f, "父母"),
            Relationship::Other => write!(f, "其他"),
        }
    }
}

impl Default for Relationship {
    fn default() -> Self {
        Relationship::Head
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Gender {
    Male,
    Female,
}

impl std::fmt::Display for Gender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Gender::Male => write!(f, "男"),
            Gender::Female => write!(f, "女"),
        }
    }
}

impl Default for Gender {
    fn default() -> Self {
        Gender::Male
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Education {
    Primary,      // 小学
    MiddleSchool, // 初中
    HighSchool,   // 高中
    College,      // 大专
    University,   // 本科
    Graduate,     // 研究生
    Other,        // 其他
}

impl std::fmt::Display for Education {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Education::Primary => write!(f, "小学"),
            Education::MiddleSchool => write!(f, "初中"),
            Education::HighSchool => write!(f, "高中"),
            Education::College => write!(f, "大专"),
            Education::University => write!(f, "本科"),
            Education::Graduate => write!(f, "研究生"),
            Education::Other => write!(f, "其他"),
        }
    }
}

impl Default for Education {
    fn default() -> Self {
        Education::University
    }
}

// 表单数据结构
#[derive(Debug, Clone, Default)]
pub struct HouseholdForm {
    pub head_name: String,
    pub id_number: String,
    pub address: String,
    pub phone: String,
    pub household_type: HouseholdType,
    pub members: Vec<MemberForm>,
}

#[derive(Debug, Clone, Default)]
pub struct MemberForm {
    pub name: String,
    pub id_number: String,
    pub relationship: Relationship,
    pub birth_year: i32,
    pub birth_month: u32,
    pub birth_day: u32,
    pub gender: Gender,
    pub education: Education,
    pub occupation: String,
}

impl HouseholdForm {
    pub fn from_household(household: &Household) -> Self {
        Self {
            head_name: household.head_name.clone(),
            id_number: household.id_number.clone(),
            address: household.address.clone(),
            phone: household.phone.clone(),
            household_type: household.household_type,
            members: household.members.iter().map(MemberForm::from_member).collect(),
        }
    }
    
    pub fn to_household(&self, id: Option<Uuid>) -> Option<Household> {
        let members: Result<Vec<_>, _> = self.members.iter().map(|m| m.to_member()).collect();
        let members = members.ok()?;
        
        Some(Household {
            id: id.unwrap_or_else(Uuid::new_v4),
            head_name: self.head_name.clone(),
            id_number: self.id_number.clone(),
            address: self.address.clone(),
            phone: self.phone.clone(),
            household_type: self.household_type,
            registration_date: chrono::Utc::now().naive_utc(),
            members,
        })
    }
    
    pub fn clear(&mut self) {
        *self = Self::default();
        self.members.push(MemberForm::default());
    }
}

impl MemberForm {
    pub fn from_member(member: &Member) -> Self {
        Self {
            name: member.name.clone(),
            id_number: member.id_number.clone(),
            relationship: member.relationship,
            birth_year: member.birth_date.year(),
            birth_month: member.birth_date.month(),
            birth_day: member.birth_date.day(),
            gender: member.gender,
            education: member.education,
            occupation: member.occupation.clone(),
        }
    }
    
    pub fn to_member(&self) -> Result<Member, String> {
        let birth_date = NaiveDate::from_ymd_opt(
            self.birth_year,
            self.birth_month,
            self.birth_day,
        ).ok_or("无效的出生日期")?;
        
        Ok(Member {
            name: self.name.clone(),
            id_number: self.id_number.clone(),
            relationship: self.relationship,
            birth_date,
            gender: self.gender,
            education: self.education,
            occupation: self.occupation.clone(),
        })
    }
}
