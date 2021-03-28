use csv::Reader;
use lazy_static::lazy_static;

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
    PersonalDps { dps: frames }
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
        let lower = sdps.find_lower(health);
        let upper = sdps.find_upper(health);

        match (lower, upper) {
            // lower than lowest, assume a=0
            (None, Some(upper)) => health / upper.dps(),
            // higher than highest, assume a=0
            (Some(lower), None) => health / lower.dps(),
            // normal situation, assume a=const
            (Some(lower), Some(upper)) => {
                let a = upper.acc(lower);

                lower.time()
                    + ((lower.dps().powi(2) + 2.0 * a * (health - lower.total_damage())).sqrt()
                        - lower.dps())
                        / a
            }

            (None, None) => {
                panic!("one of the dps log is not sorted with time, try to check it");
            }
        }
    }
}

impl PersonalDps {
    fn to_squad_dps(&self, k: f64) -> SquadDps {
        let d: Vec<Dps> = self
            .iter()
            .clone()
            .map(|d| Dps(d.time(), d.dps() * k))
            .collect();
        SquadDps(PersonalDps { dps: d })
    }
}

impl SquadDps {
    fn find_upper(&self, health: f64) -> Option<&Dps> {
        for it in self.iter() {
            if it.total_damage() > health {
                return Some(it);
            }
        }

        None
    }

    fn find_lower(&self, health: f64) -> Option<&Dps> {
        for it in self.iter().rev() {
            if it.total_damage() < health {
                return Some(it);
            }
        }
        None
    }
}

impl std::ops::Deref for SquadDps {
    type Target = PersonalDps;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::Deref for PersonalDps {
    type Target = Vec<Dps>;

    fn deref(&self) -> &Self::Target {
        &self.dps
    }
}

impl Dps {
    fn time(&self) -> f64 {
        self.0
    }

    fn dps(&self) -> f64 {
        self.1
    }

    fn total_damage(&self) -> f64 {
        self.time() * self.dps()
    }

    fn acc(&self, lower: &Self) -> f64 {
        (self.dps() - lower.dps()) / (self.time() - lower.time())
    }
}
