use rand::Rng;

pub trait Metric {
    fn d(&self, p: &Self) -> f32;
}

pub struct Point {
    pub lat: f32,
    pub lon: f32,
}

impl Point {
    pub fn lat(&self) -> f32 {
        self.lat
    }
    pub fn lon(&self) -> f32 {
        self.lon
    }
}

impl Metric for Point {
    fn d(&self, p: &Self) -> f32 {
        let d_x = (self.lat - p.lat) * (self.lat - p.lat);
        let d_y = (self.lon - p.lon) * (self.lon - p.lon);
        let d_squared: f32 = d_x + d_y;
        d_squared.sqrt()
    }
}

pub fn nearest_point<P>(points: Vec<P>, p: P) -> Vec<(P, f32)>
where
    P: Metric,
{
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
