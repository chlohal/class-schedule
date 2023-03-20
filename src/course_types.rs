use std::str::FromStr;

#[derive(Debug)]
pub struct Course {
    pub campus_code: u32,
    pub term_code: u32,
    pub ptrm_code: u32,
    pub crn: u32,
    pub sub_group_list: Vec<Department>,
    pub course: String,
    pub seq_num: u32,
    pub title: String,
    pub long_title: String,
    pub attr_code: Vec<CourseAttribute>,
    pub units: f64,
    pub cap: u32,
    pub enr: u32,
    pub permission_only: bool,
    pub instructor: String,
    pub days: Vec<CourseDay>,
    pub times: Vec<TimeRange>,
    pub building_abbrev: String,
    pub note: String,
}
#[derive(Debug)]
pub struct TimeRange {
    pub start: ClassTime,
    pub end: ClassTime,
}
impl FromStr for TimeRange {
    type Err = ();

    fn from_str(input: &str) -> Result<TimeRange, Self::Err> {
        let (start, end) = input.split_once("-").ok_or(())?;

        Ok(TimeRange { start: start.trim().parse()?, end: end.trim().parse()? })
    }
}

#[derive(Debug)]
pub struct ClassTime {
    pub hour: u32,
    pub minute: u32,
}

impl FromStr for ClassTime {
    type Err = ();

    fn from_str(input: &str) -> Result<ClassTime, Self::Err> {
        if input.len() != 8 {
            return Err(());
        }

        let hour_offset = if input.ends_with("PM") {12} else {0};

        let hour: u32 = input[0..2].parse().map_err(|_| ())?;
        let min: u32 = input[3..5].parse().map_err(|_| ())?;

        Ok(ClassTime {
            hour: hour + hour_offset,
            minute: min,
        })
    }
}


#[derive(Debug)]
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
            _ => Err(()),
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
            _ => Err(()),
        }
    }
}
