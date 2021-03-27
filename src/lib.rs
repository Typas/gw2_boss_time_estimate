use lazy_static::lazy_static;
use csv::Reader;

pub struct BossPhase {
    index: String,
    health: f64,
    coeff: f64,
    power_coeff: f64,
    num_dps: f64,
}

struct Dps(f64, f64);

struct PersonalDps {
    dps: Vec<Dps>,
}

struct SquadDps(PersonalDps);

fn get_dps(dps_type: &str) -> PersonalDps {
    let prefix = "dps/";
    let suffix = ".csv";
    let path = prefix.to_string() + dps_type + suffix;
    let expect_prefix = "file \"";
    let expect_suffix = "\" not found";
    let expection = expect_prefix.to_string() + &path + expect_suffix;
    let mut rd = Reader::from_path(&path).expect(&expection);
    let mut frames: Vec<Dps> = Vec::new();
    for result in rd.records() {
        let r = result.expect("not a valid dps format");
        let t: f64 = r[0].parse().expect("not a valid time format");
        let d: f64 = r[1].parse().expect("not a valid dps format");

        frames.push(Dps(t, d));
    }
    PersonalDps {
        dps: frames
    }
}

lazy_static! {
    static ref POWER: PersonalDps = get_dps("power");
    static ref SEMI: PersonalDps = get_dps("semi");
    static ref CONDI: PersonalDps = get_dps("condi");
}

impl BossPhase {
    pub fn init(index: String, health: f64, coeff: f64, power: f64, num: f64) -> BossPhase {
        BossPhase {
            index,
            health,
            coeff,
            power_coeff: power,
            num_dps: num,
        }
    }

    pub fn power_time(&self) -> f64 {
        self.get_time(self.health / self.power_coeff / self.coeff, &POWER)
    }

    pub fn semi_time(&self) -> f64 {
        self.get_time(self.health / self.coeff, &SEMI)
    }

    pub fn condi_time(&self) -> f64 {
        self.get_time(self.health / self.coeff, &CONDI)
    }

    pub fn phase(&self) -> &str {
        &self.index
    }

    fn get_time(&self, health: f64, pdps: &PersonalDps) -> f64 {
        let sdps = pdps.to_squad_dps(self.num_dps);
        let i_lower = sdps.index_lower(health);
        let i_upper = sdps.index_upper(health);

        if i_upper == 0 {
            // assuming a = 0, use longest dps
            health / sdps.dps.last().unwrap().1
        } else if i_lower == sdps.dps.len() {
            // assuming a = 0, use shortest dps
            health / sdps.dps.first().unwrap().1
        } else {
            let a = sdps.dps[i_upper].acc(&sdps.dps[i_lower]);

            // assuming a = const, use interpolation
            // normal situation: [-b+sqrt(b^2-4ac)]/2a

            sdps.dps[i_lower].0 as f64
                + (sdps.dps[i_lower].1.powi(2)
                    + 2.0 * a * (health - sdps.dps[i_lower].total_damage())
                    - sdps.dps[i_lower].1)
                    .sqrt()
                    / a
        }
    }
}

impl PersonalDps {
    fn to_squad_dps(&self, k: f64) -> SquadDps {
        let d: Vec<Dps> = self.dps.iter().clone().map(|d| Dps(d.0, d.1 * k)).collect();
        SquadDps(PersonalDps { dps: d })
    }
}

impl SquadDps {
    fn index_upper(&self, health: f64) -> usize {
        for i in self.dps.len()-1..=0 {
            if self.dps[i].total_damage() < health {
                return i + 1;
            }
        }

        0 // health too small
    }

    fn index_lower(&self, health: f64) -> usize {
        for i in 0..self.dps.len() {
            if self.dps[i].total_damage() > health {
                return i - 1;
            }
        }

        self.dps.len() // health too large
    }
}

impl std::ops::Deref for SquadDps {
    type Target = PersonalDps;

    fn deref(&self) -> &PersonalDps {
        &self.0
    }
}

impl Dps {
    fn total_damage(&self) -> f64 {
        (self.0 as f64) * self.1
    }

    fn acc(&self, lower: &Self) -> f64 {
        (self.1 - lower.1) / (self.0 - lower.0) as f64
    }
}
