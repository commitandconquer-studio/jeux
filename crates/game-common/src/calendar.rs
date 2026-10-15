//! Calendrier des fêtes, commun à tous les jeux : date du jour, Pâques et
//! toutes les fêtes qui en découlent, fêtes fixes.
//!
//! Pour essayer une autre date : `CC_DATE=2026-12-25 cargo run -p …`.

/// Une date du calendrier grégorien.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Date {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}

impl Date {
    pub fn new(year: i32, month: u32, day: u32) -> Self {
        Date { year, month, day }
    }

    /// Aujourd'hui (heure locale approximée : UTC), ou `CC_DATE` si défini.
    pub fn today() -> Self {
        if let Some(d) = std::env::var("CC_DATE").ok().and_then(|s| Date::parse(&s)) {
            return d;
        }
        let secs = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
        Date::from_days(secs as i64 / 86_400)
    }

    /// `AAAA-MM-JJ`.
    pub fn parse(s: &str) -> Option<Date> {
        let mut it = s.trim().split('-');
        let d = Date::new(it.next()?.parse().ok()?, it.next()?.parse().ok()?, it.next()?.parse().ok()?);
        (1..=12).contains(&d.month).then_some(d)
    }

    /// Nombre de jours depuis le 1ᵉʳ janvier 1970 (algorithme de H. Hinnant).
    pub fn days(self) -> i64 {
        let y = self.year as i64 - (self.month <= 2) as i64;
        let era = y.div_euclid(400);
        let yoe = y - era * 400;
        let m = self.month as i64;
        let doy = (153 * (m + if m > 2 { -3 } else { 9 }) + 2) / 5 + self.day as i64 - 1;
        let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
        era * 146_097 + doe - 719_468
    }

    pub fn from_days(z: i64) -> Date {
        let z = z + 719_468;
        let era = z.div_euclid(146_097);
        let doe = z - era * 146_097;
        let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
        let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
        let mp = (5 * doy + 2) / 153;
        let day = (doy - (153 * mp + 2) / 5 + 1) as u32;
        let month = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
        Date::new((yoe + era * 400 + (month <= 2) as i64) as i32, month, day)
    }

    pub fn plus(self, days: i64) -> Date {
        Date::from_days(self.days() + days)
    }

    /// 0 = lundi … 6 = dimanche.
    pub fn weekday(self) -> usize {
        (self.days() + 3).rem_euclid(7) as usize
    }

    /// « jeudi 25 décembre 2026 ».
    pub fn french(self) -> String {
        const DAYS: [&str; 7] = ["lundi", "mardi", "mercredi", "jeudi", "vendredi", "samedi", "dimanche"];
        const MONTHS: [&str; 12] =
            ["janvier", "février", "mars", "avril", "mai", "juin", "juillet", "août", "septembre", "octobre", "novembre", "décembre"];
        let d = if self.day == 1 { "1ᵉʳ".to_string() } else { self.day.to_string() };
        format!("{} {d} {} {}", DAYS[self.weekday()], MONTHS[self.month as usize - 1], self.year)
    }
}

/// Dimanche de Pâques (calendrier grégorien, algorithme « anonyme »).
pub fn easter(year: i32) -> Date {
    let a = year % 19;
    let b = year / 100;
    let c = year % 100;
    let d = b / 4;
    let e = b % 4;
    let f = (b + 8) / 25;
    let g = (b - f + 1) / 3;
    let h = (19 * a + b - d - g + 15) % 30;
    let i = c / 4;
    let k = c % 4;
    let l = (32 + 2 * e + 2 * i - h - k) % 7;
    let m = (a + 11 * h + 22 * l) / 451;
    let month = (h + l - 7 * m + 114) / 31;
    let day = (h + l - 7 * m + 114) % 31 + 1;
    Date::new(year, month as u32, day as u32)
}

/// Les fêtes et périodes du calendrier.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Feast {
    Epiphany,
    Candlemas,
    MardiGras,
    AshWednesday,
    Lent,
    Annunciation,
    PalmSunday,
    /// Jeudi, vendredi et samedi saints : les cloches se taisent.
    HolyTriduum,
    Easter,
    Ascension,
    Pentecost,
    CorpusChristi,
    Assumption,
    Halloween,
    AllSaints,
    SaintMartin,
    /// Avent : le nombre de bougies allumées (1 à 4).
    Advent(u8),
    SaintNicholas,
    Christmas,
}

impl Feast {
    pub fn name(self) -> &'static str {
        match self {
            Feast::Epiphany => "Épiphanie",
            Feast::Candlemas => "Chandeleur",
            Feast::MardiGras => "Mardi gras",
            Feast::AshWednesday => "Mercredi des Cendres",
            Feast::Lent => "Carême",
            Feast::Annunciation => "Annonciation",
            Feast::PalmSunday => "Rameaux",
            Feast::HolyTriduum => "Semaine sainte",
            Feast::Easter => "Pâques",
            Feast::Ascension => "Ascension",
            Feast::Pentecost => "Pentecôte",
            Feast::CorpusChristi => "Fête-Dieu",
            Feast::Assumption => "Assomption",
            Feast::Halloween => "Halloween",
            Feast::AllSaints => "Toussaint",
            Feast::SaintMartin => "Saint-Martin",
            Feast::Advent(_) => "Avent",
            Feast::SaintNicholas => "Saint-Nicolas",
            Feast::Christmas => "Noël",
        }
    }

    /// Petit message de bienvenue.
    pub fn greeting(self) -> &'static str {
        match self {
            Feast::Epiphany => "Bonne Épiphanie ! Qui aura la fève ?",
            Feast::Candlemas => "Bonne Chandeleur ! Les crêpes sont prêtes.",
            Feast::MardiGras => "Joyeux Mardi gras !",
            Feast::AshWednesday | Feast::Lent => "Temps du Carême : le concile se fait sobre.",
            Feast::Annunciation => "Bonne fête de l'Annonciation.",
            Feast::PalmSunday => "Bon dimanche des Rameaux !",
            Feast::HolyTriduum => "Les cloches sont parties à Rome : silence au clocher.",
            Feast::Easter => "Joyeuses Pâques ! Les cloches sont revenues.",
            Feast::Ascension => "Bonne fête de l'Ascension !",
            Feast::Pentecost => "Bonne Pentecôte !",
            Feast::CorpusChristi => "Bonne Fête-Dieu !",
            Feast::Assumption => "Bonne fête de l'Assomption !",
            Feast::Halloween => "Joyeux Halloween ! Même les évêques ont peur.",
            Feast::AllSaints => "Toussaint : une pensée pour tous les saints.",
            Feast::SaintMartin => "Bonne Saint-Martin !",
            Feast::Advent(_) => "Temps de l'Avent : Noël approche.",
            Feast::SaintNicholas => "Bonne Saint-Nicolas ! Il était évêque, lui aussi.",
            Feast::Christmas => "Joyeux Noël !",
        }
    }

    /// Les cloches sonnent-elles ? (Non pendant les jours saints.)
    pub fn bells_silent(self) -> bool {
        self == Feast::HolyTriduum
    }
}

/// La fête (ou période) d'une date, s'il y en a une.
pub fn feast_on(date: Date) -> Option<Feast> {
    let y = date.year;
    let e = easter(y);
    let off = date.days() - e.days();
    let fixed = |m: u32, d: u32| date.month == m && date.day == d;
    // Fêtes mobiles, liées à Pâques
    let movable = match off {
        -47 => Some(Feast::MardiGras),
        -46 => Some(Feast::AshWednesday),
        -7 => Some(Feast::PalmSunday),
        -3..=-1 => Some(Feast::HolyTriduum),
        0 | 1 => Some(Feast::Easter),
        39 => Some(Feast::Ascension),
        49 | 50 => Some(Feast::Pentecost),
        60 => Some(Feast::CorpusChristi),
        _ => None,
    };
    if movable.is_some() {
        return movable;
    }
    // Fêtes fixes
    if fixed(1, 6) {
        return Some(Feast::Epiphany);
    }
    if fixed(2, 2) {
        return Some(Feast::Candlemas);
    }
    if fixed(3, 25) {
        return Some(Feast::Annunciation);
    }
    if fixed(8, 15) {
        return Some(Feast::Assumption);
    }
    if fixed(10, 31) {
        return Some(Feast::Halloween);
    }
    if fixed(11, 1) || fixed(11, 2) {
        return Some(Feast::AllSaints);
    }
    if fixed(11, 11) {
        return Some(Feast::SaintMartin);
    }
    if fixed(12, 6) {
        return Some(Feast::SaintNicholas);
    }
    if date.month == 12 && (24..=26).contains(&date.day) {
        return Some(Feast::Christmas);
    }
    // Avent : du 4ᵉ dimanche avant Noël jusqu'au 23 décembre
    let christmas = Date::new(y, 12, 25);
    let last_sunday = christmas.plus(-(christmas.weekday() as i64 + 1)); // dimanche avant Noël
    let first_sunday = last_sunday.plus(-21);
    if date >= first_sunday && date < Date::new(y, 12, 24) {
        let week = ((date.days() - first_sunday.days()) / 7 + 1).clamp(1, 4) as u8;
        return Some(Feast::Advent(week));
    }
    // Carême : entre les Cendres et les Rameaux
    if (-45..-7).contains(&off) {
        return Some(Feast::Lent);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dates_de_paques_connues() {
        assert_eq!(easter(2024), Date::new(2024, 3, 31));
        assert_eq!(easter(2025), Date::new(2025, 4, 20));
        assert_eq!(easter(2026), Date::new(2026, 4, 5));
        assert_eq!(easter(2027), Date::new(2027, 3, 28));
    }

    #[test]
    fn jours_et_dates_font_l_aller_retour() {
        for z in [-1000, 0, 19_000, 20_741, 30_000] {
            assert_eq!(Date::from_days(z).days(), z);
        }
        assert_eq!(Date::new(1970, 1, 1).weekday(), 3); // jeudi
        assert_eq!(Date::new(2026, 10, 15).weekday(), 3); // jeudi
    }

    #[test]
    fn quelques_fetes() {
        assert_eq!(feast_on(Date::new(2026, 12, 25)), Some(Feast::Christmas));
        assert_eq!(feast_on(Date::new(2026, 4, 5)), Some(Feast::Easter));
        assert_eq!(feast_on(Date::new(2026, 3, 29)), Some(Feast::PalmSunday));
        assert_eq!(feast_on(Date::new(2026, 5, 14)), Some(Feast::Ascension));
        assert_eq!(feast_on(Date::new(2026, 10, 31)), Some(Feast::Halloween));
        assert_eq!(feast_on(Date::new(2026, 4, 3)), Some(Feast::HolyTriduum));
        // Avent 2026 : 1ᵉʳ dimanche le 29 novembre
        assert_eq!(feast_on(Date::new(2026, 11, 29)), Some(Feast::Advent(1)));
        assert_eq!(feast_on(Date::new(2026, 12, 20)), Some(Feast::Advent(4)));
        assert_eq!(feast_on(Date::new(2026, 7, 7)), None);
    }
}
