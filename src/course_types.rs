use std::{str::FromStr, sync::Arc, fmt::Display};

#[derive(Debug)]
pub struct CourseCredit {
    pub course: Arc<String>,
    pub attr_code: Arc<Vec<CourseAttribute>>
}

#[derive(Debug)]
pub struct CourseSection {
    pub campus_code: u32,
    pub term_code: String,
    pub ptrm_code: String,
    pub crn: u32,
    pub cap: u32,
    pub enr: u32,
    pub permission_only: bool,
    pub instructor: String,
    pub schedule: Vec<ClassPeriod>,
    pub location: Vec<ClassLocation>,
    pub note: String,
    pub course: Arc<Course>
}

impl std::hash::Hash for CourseSection {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.crn.hash(state);
    }
}

impl Eq for CourseSection {
    fn assert_receiver_is_total_eq(&self) {}
}

impl PartialEq for CourseSection {
    fn eq(&self, other: &Self) -> bool {
        self.crn == other.crn
    }
}

#[derive(Debug)]
pub struct Course {
    pub sub_group_list: Vec<Department>,
    pub course: Arc<String>,
    pub seq_num: u32,
    pub title: String,
    pub long_title: String,
    pub attr_code: Arc<Vec<CourseAttribute>>,
    pub units: String,
}

impl PartialEq for Course {
    fn eq(&self, other: &Self) -> bool {
        self.course == other.course
    }
}

#[derive(Debug)]
pub enum ClassLocation {
    Classroom(String),
    Unspecified,
}

impl FromStr for ClassLocation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut classroom = String::new();

        let mut depth = 0;
        let mut quote = false;
        for char in s.chars() {
            if quote && char != '"' {
                continue;
            }

            match char {
                '<' => depth += 1,
                '>' => depth -= 1,
                '"' => quote = !quote,
                _ => {
                    if depth == 0 {
                        classroom.push(char)
                    }
                }
            }
        }

        if classroom == "-" {
            Ok(ClassLocation::Unspecified)
        } else {
            Ok(ClassLocation::Classroom(classroom))
        }
    }
}

#[derive(Debug)]
pub struct  ClassPeriod {
    pub days: Vec<CourseDay>,
    pub time: TimeRange
}

#[derive(Debug)]
pub enum ClassTime {
    Specified(TimeRange),
    Unspecified,
}

#[derive(Debug)]
pub struct TimeRange {
    pub start: TimeOfDay,
    pub end: TimeOfDay,
}
impl Display for TimeRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} - {}", self.start, self.end))
    }
}

impl FromStr for ClassTime {
    type Err = ();

    fn from_str(input: &str) -> Result<ClassTime, Self::Err> {
        if input == "-" {
            return Ok(ClassTime::Unspecified);
        }

        let (start, end) = input.split_once("-").ok_or(())?;

        Ok(ClassTime::Specified(TimeRange {
            start: start.trim().parse()?,
            end: end.trim().parse()?,
        }))
    }
}

#[derive(Debug)]
pub struct TimeOfDay {
    pub hour: u32,
    pub minute: u32,
}

impl Display for TimeOfDay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let h = if self.hour < 12 { self.hour } else if self.hour > 12 { self.hour - 12 } else { 12 };

        f.write_fmt(format_args!("{}:{:0>2} {}", h, self.minute, if self.hour < 12 { "AM" } else { "PM" }))
    }
}

impl FromStr for TimeOfDay {
    type Err = ();

    fn from_str(input: &str) -> Result<TimeOfDay, Self::Err> {
        if input.len() != 8 {
            return Err(());
        }

        

        let hour: u32 = input[0..2].parse().map_err(|_| ())?;
        let min: u32 = input[3..5].parse().map_err(|_| ())?;

        let hour_offset = if input.ends_with("PM") && hour < 12 { 12 } else { 0 };

        Ok(TimeOfDay {
            hour: hour + hour_offset,
            minute: min,
        })
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum CourseAttribute {
    AP,
    DI,
    FA,
    GP,
    HP,
    LP,
    POP,
    SP,
    VE,
    VP,
    FYI
}

impl FromStr for CourseAttribute {
    type Err = ();

    fn from_str(input: &str) -> Result<CourseAttribute, Self::Err> {
        match input {
            "AP" => Ok(CourseAttribute::AP),
            "DI" => Ok(CourseAttribute::DI),
            "FA" => Ok(CourseAttribute::FA),
            "GP" => Ok(CourseAttribute::GP),
            "HP" => Ok(CourseAttribute::HP),
            "LP" => Ok(CourseAttribute::LP),
            "POP" => Ok(CourseAttribute::POP),
            "SP" => Ok(CourseAttribute::SP),
            "VE" => Ok(CourseAttribute::VE),
            "VP" => Ok(CourseAttribute::VP),
            "FYI" => Ok(CourseAttribute::FYI),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub enum CourseDays {
    Specified(Vec<CourseDay>),
    Unspecified,
}

impl FromStr for CourseDays {
    type Err = ();

    fn from_str(input: &str) -> Result<CourseDays, Self::Err> {
        match input {
            "-" => Ok(CourseDays::Unspecified),
            _ => Ok(CourseDays::Specified(
                input
                    .chars()
                    .map(|d| d.try_into())
                    .collect::<Result<Vec<CourseDay>, _>>()?
            )),
        }
    }
}

#[derive(Debug)]
pub enum CourseDay {
    M,
    T,
    W,
    R,
    F,
}

impl TryFrom<char> for CourseDay {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'M' => Ok(CourseDay::M),
            'T' => Ok(CourseDay::T),
            'W' => Ok(CourseDay::W),
            'R' => Ok(CourseDay::R),
            'F' => Ok(CourseDay::F),
            _ => Err(()),
        }
    }
}

impl FromStr for CourseDay {
    type Err = ();

    fn from_str(input: &str) -> Result<CourseDay, Self::Err> {
        match input {
            "M" => Ok(CourseDay::M),
            "T" => Ok(CourseDay::T),
            "W" => Ok(CourseDay::W),
            "R" => Ok(CourseDay::R),
            "F" => Ok(CourseDay::F),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub enum Department {
    ACCT,
    AFRC,
    ARAB,
    ARTH,
    ARTS,
    AS,
    ASTR,
    BAN,
    BCMB,
    BCOM,
    BIOL,
    BLAW,
    CHEM,
    CHIN,
    CLAS,
    CMLT,
    CRES,
    CRW,
    CSAC,
    CSCI,
    CYES,
    DSCI,
    ECON,
    EDUC,
    EN,
    ENG,
    ENT,
    EPP,
    FIN,
    FREN,
    GAME,
    GART,
    GCPT,
    GEOG,
    GERM,
    GES,
    GGRA,
    HEBR,
    HGS,
    HIST,
    HS,
    HSS,
    ID,
    IDCE,
    IDND,
    JAPN,
    JS,
    LALS,
    LAS,
    LAT,
    MATH,
    MCA,
    MGMT,
    MKT,
    MUSC,
    NRES,
    PECO,
    PHIL,
    PHYS,
    PSCI,
    PSYC,
    QBUS,
    SCRN,
    SOC,
    SPAN,
    TA,
    UDSC,
    WGS,
    ESL
}

impl FromStr for Department {
    type Err = ();

    fn from_str(input: &str) -> Result<Department, Self::Err> {
        match input {
            "ACCT" => Ok(Department::ACCT),
            "AFRC" => Ok(Department::AFRC),
            "ARAB" => Ok(Department::ARAB),
            "ARTH" => Ok(Department::ARTH),
            "ARTS" => Ok(Department::ARTS),
            "AS" => Ok(Department::AS),
            "ASTR" => Ok(Department::ASTR),
            "BAN" => Ok(Department::BAN),
            "BCMB" => Ok(Department::BCMB),
            "BCOM" => Ok(Department::BCOM),
            "BIOL" => Ok(Department::BIOL),
            "BLAW" => Ok(Department::BLAW),
            "CHEM" => Ok(Department::CHEM),
            "CHIN" => Ok(Department::CHIN),
            "CLAS" => Ok(Department::CLAS),
            "CMLT" => Ok(Department::CMLT),
            "CRES" => Ok(Department::CRES),
            "CRW" => Ok(Department::CRW),
            "CSAC" => Ok(Department::CSAC),
            "CSCI" => Ok(Department::CSCI),
            "CYES" => Ok(Department::CYES),
            "DSCI" => Ok(Department::DSCI),
            "ECON" => Ok(Department::ECON),
            "EDUC" => Ok(Department::EDUC),
            "EN" => Ok(Department::EN),
            "ENG" => Ok(Department::ENG),
            "ENT" => Ok(Department::ENT),
            "EPP" => Ok(Department::EPP),
            "FIN" => Ok(Department::FIN),
            "FREN" => Ok(Department::FREN),
            "GAME" => Ok(Department::GAME),
            "GART" => Ok(Department::GART),
            "GCPT" => Ok(Department::GCPT),
            "GEOG" => Ok(Department::GEOG),
            "GERM" => Ok(Department::GERM),
            "GES" => Ok(Department::GES),
            "GGRA" => Ok(Department::GGRA),
            "HEBR" => Ok(Department::HEBR),
            "HGS" => Ok(Department::HGS),
            "HIST" => Ok(Department::HIST),
            "HS" => Ok(Department::HS),
            "HSS" => Ok(Department::HSS),
            "ID" => Ok(Department::ID),
            "IDCE" => Ok(Department::IDCE),
            "IDND" => Ok(Department::IDND),
            "JAPN" => Ok(Department::JAPN),
            "JS" => Ok(Department::JS),
            "LALS" => Ok(Department::LALS),
            "LAS" => Ok(Department::LAS),
            "LAT" => Ok(Department::LAT),
            "MATH" => Ok(Department::MATH),
            "MCA" => Ok(Department::MCA),
            "MGMT" => Ok(Department::MGMT),
            "MKT" => Ok(Department::MKT),
            "MUSC" => Ok(Department::MUSC),
            "NRES" => Ok(Department::NRES),
            "PECO" => Ok(Department::PECO),
            "PHIL" => Ok(Department::PHIL),
            "PHYS" => Ok(Department::PHYS),
            "PSCI" => Ok(Department::PSCI),
            "PSYC" => Ok(Department::PSYC),
            "QBUS" => Ok(Department::QBUS),
            "SCRN" => Ok(Department::SCRN),
            "SOC" => Ok(Department::SOC),
            "SPAN" => Ok(Department::SPAN),
            "TA" => Ok(Department::TA),
            "UDSC" => Ok(Department::UDSC),
            "WGS" => Ok(Department::WGS),
            "ESL" => Ok(Department::ESL),
            _ => Err(()),
        }
    }
}
