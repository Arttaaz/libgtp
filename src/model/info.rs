use std::str::FromStr;

use crate::model::Vertex;

#[derive(Debug, Clone)]
pub struct Info {
    pub explored_moves: Vec<InfoMove>,
    pub ownership: Vec<f32>,
}

impl Info {
    pub const fn new() -> Self {
        Self {
            explored_moves: vec![],
            ownership: vec![],
        }
    }
}

impl FromStr for Info {
    type Err = super::ParseError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let matches = if s.contains("ownership ") {
            s.split_once("ownership ").unwrap()
        } else {
            (s, "")
        };
        if matches.0.is_empty() {
            return Err(super::ParseError::EmptyString)
        }
        let infos = matches.0; //info part
        let ownership = matches.1; //ownership part
        
        let mut matches = infos.split("info");
        matches.next(); //consume empty string
        
        let mut infos: Vec<InfoMove> = Vec::new();
        while let Some(s) = matches.next() {
            infos.push(s.parse()?);
        }
        
        let mut vec: Vec<f32> = vec![];
        if !ownership.is_empty() {
            let matches = ownership.split_ascii_whitespace();
            vec = matches.map(|f| f.parse().unwrap()).collect();
        }

        Ok(Self {
            explored_moves: infos,
            ownership: vec,
        })
    }
}

#[derive(Debug, Clone)]
pub struct InfoMove {
    pub coord: Vertex,
    pub visits: u64,
    pub winrate: f32,
    pub score_mean: f32, //compatibility field, same as score_lead
    pub score_stdev: f32, //estimation of score after this move
    pub score_lead: f32,
    pub score_selfplay: f32,
    pub prior: f32,
    pub utility: f32,
    pub lcb: f32,
    pub utility_lcb: f32,
    pub order: u16, //ranking of the move, max is 361 so u16 is sufficient
    pub pv: Vec<Vertex>,
    pub pv_visits: Vec<u64>,
}

impl FromStr for InfoMove {
    type Err = super::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut coord: Vertex = Vertex::Pass;
        let mut visits: u64 = 0;
        let mut winrate: f32 = 0.0;
        let mut score_mean: f32 = 0.0;
        let mut score_stdev: f32 = 0.0;
        let mut score_lead: f32 = 0.0;
        let mut score_selfplay: f32 = 0.0;
        let mut prior: f32 = 0.0;
        let mut utility: f32 = 0.0;
        let mut lcb: f32 = 0.0;
        let mut utility_lcb: f32 = 0.0;
        let mut order: u16 = 0;
        let mut pv: Vec<Vertex> = vec![];
        let mut pv_visits: Vec<u64> = vec![];
        
        let matches = s.split_whitespace();
        let mut matches = matches.peekable();

        while let Some(s) = matches.next() {
            match s {
                "move" => {
                    coord = matches.next().unwrap().parse()?;
                },
                "visits" => {
                    visits = matches.next().unwrap().parse()?;
                },
                "winrate" => {
                    winrate = matches.next().unwrap().parse()?;
                },
                "scoreMean" => {
                    score_mean = matches.next().unwrap().parse()?;
                },
                "scoreStdev" => {
                    score_stdev = matches.next().unwrap().parse()?;
                },
                "scoreLead" => {
                    score_lead = matches.next().unwrap().parse()?;
                },
                "scoreSelfplay" => {
                    score_selfplay = matches.next().unwrap().parse()?;
                },
                "prior" => {
                    prior = matches.next().unwrap().parse()?;
                },
                "utility" => {
                    utility = matches.next().unwrap().parse()?;
                },
                "lcb" => {
                    lcb = matches.next().unwrap().parse()?;
                },
                "utilityLcb" => {
                    utility_lcb = matches.next().unwrap().parse()?;
                },
                "order" => {
                    order = matches.next().unwrap().parse()?;
                },
                "pv" => {
                    while let Some(s) = matches.peek() {
                        if *s == "pvVisits" {
                            break
                        }
                        pv.push(matches.next().unwrap().parse()?);
                    }
                },
                "pvVisits" => {
                    while let Some(s) = matches.next() {
                        pv_visits.push(s.parse()?);
                    }
                },
                "isSymmetryOf" => { matches.next();},
                _ => return Err(super::ParseError::WrongAlternative),
            }
        }

        
        Ok(Self {
            coord,
            visits,
            winrate,
            score_mean,
            score_stdev,
            score_lead,
            score_selfplay,
            prior,
            utility,
            lcb,
            utility_lcb,
            order,
            pv,
            pv_visits,
        })
    }
}
