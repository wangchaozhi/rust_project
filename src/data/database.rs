use rusqlite::{Connection, Result, params};
use crate::data::models::*;
use crate::data::manager::HouseholdStatistics;
use chrono::{NaiveDateTime, NaiveDate};
use uuid::Uuid;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        let db = Database { conn };
        db.init_tables()?;
        Ok(db)
    }
    
    fn init_tables(&self) -> Result<()> {
        // 创建户籍表
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS households (
                id TEXT PRIMARY KEY,
                head_name TEXT NOT NULL,
                id_number TEXT NOT NULL,
                address TEXT NOT NULL,
                phone TEXT,
                household_type TEXT NOT NULL,
                registration_date TEXT NOT NULL
            )",
            [],
        )?;
        
        // 创建成员表
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS members (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                household_id TEXT NOT NULL,
                name TEXT NOT NULL,
                id_number TEXT NOT NULL,
                relationship TEXT NOT NULL,
                birth_date TEXT NOT NULL,
                gender TEXT NOT NULL,
                education TEXT NOT NULL,
                occupation TEXT,
                FOREIGN KEY (household_id) REFERENCES households (id)
            )",
            [],
        )?;
        
        Ok(())
    }
    
    // 检查数据库是否为空
    pub fn is_empty(&self) -> Result<bool> {
        let mut stmt = self.conn.prepare("SELECT COUNT(*) FROM households")?;
        let count: i64 = stmt.query_row([], |row| row.get(0))?;
        Ok(count == 0)
    }
    
    // 户籍相关操作
    pub fn insert_household(&self, household: &Household) -> Result<()> {
        self.conn.execute(
            "INSERT INTO households (id, head_name, id_number, address, phone, household_type, registration_date) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                household.id.to_string(),
                household.head_name,
                household.id_number,
                household.address,
                household.phone,
                household.household_type.to_string(),
                household.registration_date.format("%Y-%m-%d %H:%M:%S").to_string(),
            ],
        )?;
        
        // 插入成员
        for member in &household.members {
            self.insert_member(&household.id, member)?;
        }
        
        Ok(())
    }
    
    pub fn update_household(&self, household: &Household) -> Result<()> {
        // 更新户籍信息
        self.conn.execute(
            "UPDATE households SET head_name = ?1, id_number = ?2, address = ?3, phone = ?4, 
             household_type = ?5, registration_date = ?6 WHERE id = ?7",
            params![
                household.head_name,
                household.id_number,
                household.address,
                household.phone,
                household.household_type.to_string(),
                household.registration_date.format("%Y-%m-%d %H:%M:%S").to_string(),
                household.id.to_string(),
            ],
        )?;
        
        // 删除旧成员
        self.conn.execute(
            "DELETE FROM members WHERE household_id = ?1",
            params![household.id.to_string()],
        )?;
        
        // 插入新成员
        for member in &household.members {
            self.insert_member(&household.id, member)?;
        }
        
        Ok(())
    }
    
    pub fn delete_household(&self, household_id: &Uuid) -> Result<()> {
        // 先删除成员
        self.conn.execute(
            "DELETE FROM members WHERE household_id = ?1",
            params![household_id.to_string()],
        )?;
        
        // 再删除户籍
        self.conn.execute(
            "DELETE FROM households WHERE id = ?1",
            params![household_id.to_string()],
        )?;
        
        Ok(())
    }
    
    pub fn get_all_households(&self) -> Result<Vec<Household>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, head_name, id_number, address, phone, household_type, registration_date 
             FROM households ORDER BY registration_date DESC"
        )?;
        
        let household_iter = stmt.query_map([], |row| {
            let id_str: String = row.get(0)?;
            let household_type_str: String = row.get(5)?;
            let registration_date_str: String = row.get(6)?;
            
            let id = Uuid::parse_str(&id_str).map_err(|_| rusqlite::Error::InvalidParameterName("Invalid UUID".to_string()))?;
            let household_type = match household_type_str.as_str() {
                "城镇户口" => HouseholdType::Urban,
                "农村户口" => HouseholdType::Rural,
                _ => HouseholdType::Urban,
            };
            let registration_date = if registration_date_str.contains(' ') {
                NaiveDateTime::parse_from_str(&registration_date_str, "%Y-%m-%d %H:%M:%S")
                    .map_err(|_| rusqlite::Error::InvalidParameterName("Invalid date format".to_string()))?
            } else {
                // 如果只有日期，则使用默认时间
                let date = NaiveDate::parse_from_str(&registration_date_str, "%Y-%m-%d")
                    .map_err(|_| rusqlite::Error::InvalidParameterName("Invalid date format".to_string()))?;
                date.and_hms_opt(0, 0, 0).unwrap()
            };
            
            Ok(Household {
                id,
                head_name: row.get(1)?,
                id_number: row.get(2)?,
                address: row.get(3)?,
                phone: row.get(4)?,
                household_type,
                registration_date,
                members: Vec::new(), // 稍后填充
            })
        })?;
        
        let mut households = Vec::new();
        for household_result in household_iter {
            let mut household = household_result?;
            // 获取成员信息
            household.members = self.get_members_by_household_id(&household.id)?;
            households.push(household);
        }
        
        Ok(households)
    }
    
    pub fn search_households(&self, query: &str) -> Result<Vec<Household>> {
        let search_pattern = format!("%{}%", query);
        let mut stmt = self.conn.prepare(
            "SELECT id, head_name, id_number, address, phone, household_type, registration_date 
             FROM households 
             WHERE head_name LIKE ?1 OR id_number LIKE ?1 OR address LIKE ?1 OR phone LIKE ?1
             ORDER BY registration_date DESC"
        )?;
        
        let household_iter = stmt.query_map(params![search_pattern], |row| {
            let id_str: String = row.get(0)?;
            let household_type_str: String = row.get(5)?;
            let registration_date_str: String = row.get(6)?;
            
            let id = Uuid::parse_str(&id_str).map_err(|_| rusqlite::Error::InvalidParameterName("Invalid UUID".to_string()))?;
            let household_type = match household_type_str.as_str() {
                "城镇户口" => HouseholdType::Urban,
                "农村户口" => HouseholdType::Rural,
                _ => HouseholdType::Urban,
            };
            let registration_date = if registration_date_str.contains(' ') {
                NaiveDateTime::parse_from_str(&registration_date_str, "%Y-%m-%d %H:%M:%S")
                    .map_err(|_| rusqlite::Error::InvalidParameterName("Invalid date format".to_string()))?
            } else {
                // 如果只有日期，则使用默认时间
                let date = NaiveDate::parse_from_str(&registration_date_str, "%Y-%m-%d")
                    .map_err(|_| rusqlite::Error::InvalidParameterName("Invalid date format".to_string()))?;
                date.and_hms_opt(0, 0, 0).unwrap()
            };
            
            Ok(Household {
                id,
                head_name: row.get(1)?,
                id_number: row.get(2)?,
                address: row.get(3)?,
                phone: row.get(4)?,
                household_type,
                registration_date,
                members: Vec::new(),
            })
        })?;
        
        let mut households = Vec::new();
        for household_result in household_iter {
            let mut household = household_result?;
            household.members = self.get_members_by_household_id(&household.id)?;
            households.push(household);
        }
        
        Ok(households)
    }
    
    // 成员相关操作
    fn insert_member(&self, household_id: &Uuid, member: &Member) -> Result<()> {
        self.conn.execute(
            "INSERT INTO members (household_id, name, id_number, relationship, birth_date, gender, education, occupation) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                household_id.to_string(),
                member.name,
                member.id_number,
                member.relationship.to_string(),
                member.birth_date.to_string(),
                member.gender.to_string(),
                member.education.to_string(),
                member.occupation,
            ],
        )?;
        Ok(())
    }
    
    fn get_members_by_household_id(&self, household_id: &Uuid) -> Result<Vec<Member>> {
        let mut stmt = self.conn.prepare(
            "SELECT name, id_number, relationship, birth_date, gender, education, occupation 
             FROM members WHERE household_id = ?1 ORDER BY id"
        )?;
        
        let member_iter = stmt.query_map(params![household_id.to_string()], |row| {
            let relationship_str: String = row.get(2)?;
            let birth_date_str: String = row.get(3)?;
            let gender_str: String = row.get(4)?;
            let education_str: String = row.get(5)?;
            
            let relationship = match relationship_str.as_str() {
                "户主" => Relationship::Head,
                "配偶" => Relationship::Spouse,
                "子女" => Relationship::Child,
                "父母" => Relationship::Parent,
                _ => Relationship::Other,
            };
            
            let birth_date = NaiveDate::parse_from_str(&birth_date_str, "%Y-%m-%d")
                .map_err(|_| rusqlite::Error::InvalidParameterName("Invalid birth date".to_string()))?;
            
            let gender = match gender_str.as_str() {
                "男" => Gender::Male,
                "女" => Gender::Female,
                _ => Gender::Male,
            };
            
            let education = match education_str.as_str() {
                "小学" => Education::Primary,
                "初中" => Education::MiddleSchool,
                "高中" => Education::HighSchool,
                "大专" => Education::College,
                "本科" => Education::University,
                "研究生" => Education::Graduate,
                _ => Education::Other,
            };
            
            Ok(Member {
                name: row.get(0)?,
                id_number: row.get(1)?,
                relationship,
                birth_date,
                gender,
                education,
                occupation: row.get(6)?,
            })
        })?;
        
        let mut members = Vec::new();
        for member_result in member_iter {
            members.push(member_result?);
        }
        
        Ok(members)
    }
    
    // 统计信息
    pub fn get_statistics(&self) -> Result<HouseholdStatistics> {
        let total_households: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM households",
            [],
            |row| row.get(0),
        )?;
        
        let urban_households: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM households WHERE household_type = '城镇户口'",
            [],
            |row| row.get(0),
        )?;
        
        let total_members: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM members",
            [],
            |row| row.get(0),
        )?;
        
        Ok(HouseholdStatistics {
            total_households: total_households as usize,
            urban_households: urban_households as usize,
            rural_households: (total_households - urban_households) as usize,
            total_members: total_members as usize,
        })
    }
}
