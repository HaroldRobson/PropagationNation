use num_traits::ToPrimitive;
use rand::Rng;
use rand::distr::weighted;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::ops::Add;
use std::ops::Mul;
use std::ops::Sub;

pub trait Metric {
    fn d(&self, p: &Self) -> f32;
}

#[derive(Copy, Clone)]
pub struct Point<T, U>
where
    T: Copy,
{
    pub lat: T,
    pub lon: T,
    pub data: U, // this could be a struct containing all the data needed? or perhaps the key for a
                 // hashmap containing that data.
}

impl<T: PartialEq + Copy, U> PartialEq for Point<T, U> {
    fn eq(&self, other: &Self) -> bool {
        self.lat == other.lat && self.lon == other.lon
    }
}

impl<T: Copy, U: Copy> Point<T, U> {
    pub fn lat(&self) -> T {
        self.lat
    }
    pub fn lon(&self) -> T {
        self.lon
    }
}

impl<T, U> Metric for Point<T, U>
where
    T: Sub<Output = T> + Mul<Output = T> + Add<Output = T> + Copy + ToPrimitive,
{
    fn d(&self, p: &Self) -> f32 {
        let d_x = (self.lat.to_f32().unwrap_or(0f32) - p.lat.to_f32().unwrap_or(0f32))
            * (self.lat.to_f32().unwrap_or(0f32) - p.lat.to_f32().unwrap_or(0f32));
        let d_y = (self.lon.to_f32().unwrap_or(0f32) - p.lon.to_f32().unwrap_or(0f32))
            * (self.lon.to_f32().unwrap_or(0f32) - p.lon.to_f32().unwrap_or(0f32));
        let d_squared: f32 = d_x + d_y;
        d_squared.sqrt() as f32
    }
}

pub fn nearest_points<P>(points: Vec<P>, p: P) -> Vec<(P, f32)>
where
    P: Metric + Copy,
{
    let mut pd_vec: Vec<(P, f32)> = Vec::new();
    for x in points.into_iter() {
        let dist = (&x).d(&p);
        pd_vec.push((x, dist));
    }

    sort(pd_vec)
}

pub fn sort<P>(pd_vec: Vec<(P, f32)>) -> Vec<(P, f32)>
where
    P: Copy,
{
    let mut lt: Vec<(P, f32)> = Vec::new();
    let mut et: Vec<(P, f32)> = Vec::new();
    let mut gt: Vec<(P, f32)> = Vec::new();
    if pd_vec.len() <= 1 {
        return pd_vec;
    }
    let (_, pivot) = pd_vec[(pd_vec.len() / 2) as usize];
    for (x, d) in pd_vec.into_iter() {
        match (&d).partial_cmp(&pivot) {
            Some(Ordering::Less) => lt.push((x, d)),
            Some(Ordering::Equal) => et.push((x, d)),
            Some(Ordering::Greater) => gt.push((x, d)),
            None => {}
        };
    }
    lt = sort(lt);
    gt = sort(gt);
    lt.append(&mut et);
    lt.append(&mut gt);
    lt
}
#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    // Custom struct for the U generic parameter
    #[derive(Debug, Clone, Copy, PartialEq)]
    struct UserData {
        id: u32,
        score: f32,
        category: u8,
    }

    #[test]
    fn test_small_set_i32() {
        let origin = Point {
            lat: 0i32,
            lon: 0i32,
            data: UserData {
                id: 0,
                score: 0.0,
                category: 0,
            },
        };

        let points = vec![
            Point {
                lat: 3,
                lon: 4,
                data: UserData {
                    id: 1,
                    score: 1.0,
                    category: 1,
                },
            },
            Point {
                lat: 1,
                lon: 1,
                data: UserData {
                    id: 2,
                    score: 2.0,
                    category: 2,
                },
            },
            Point {
                lat: 6,
                lon: 8,
                data: UserData {
                    id: 3,
                    score: 3.0,
                    category: 3,
                },
            },
        ];

        let result = nearest_points(points, origin);

        // Check that results are sorted by distance
        for i in 0..result.len() - 1 {
            assert!(result[i].1 <= result[i + 1].1, "Not sorted properly");
        }

        // Verify specific distances (3,4) should be distance 5, (1,1) should be sqrt(2), (6,8) should be 10
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_small_set_f32() {
        let origin = Point {
            lat: 0.0f32,
            lon: 0.0f32,
            data: UserData {
                id: 0,
                score: 0.0,
                category: 0,
            },
        };

        let points = vec![
            Point {
                lat: 1.5,
                lon: 2.5,
                data: UserData {
                    id: 1,
                    score: 10.5,
                    category: 1,
                },
            },
            Point {
                lat: 0.5,
                lon: 0.5,
                data: UserData {
                    id: 2,
                    score: 20.3,
                    category: 2,
                },
            },
            Point {
                lat: 3.0,
                lon: 4.0,
                data: UserData {
                    id: 3,
                    score: 30.7,
                    category: 3,
                },
            },
        ];

        let result = nearest_points(points, origin);

        for i in 0..result.len() - 1 {
            assert!(result[i].1 <= result[i + 1].1, "Not sorted properly");
        }

        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_large_random_set_i32() {
        let mut rng = rand::thread_rng();

        let origin = Point {
            lat: 0i32,
            lon: 0i32,
            data: UserData {
                id: 0,
                score: 0.0,
                category: 0,
            },
        };

        // Generate 1000 random points
        let mut points = Vec::new();
        for i in 0..1000 {
            points.push(Point {
                lat: rng.gen_range(-1000..1000),
                lon: rng.gen_range(-1000..1000),
                data: UserData {
                    id: i,
                    score: rng.gen_range(0.0..100.0),
                    category: rng.gen_range(0..10),
                },
            });
        }

        let result = nearest_points(points, origin);

        // Verify sorted
        for i in 0..result.len() - 1 {
            assert!(
                result[i].1 <= result[i + 1].1,
                "Large set not sorted at index {}",
                i
            );
        }

        assert_eq!(result.len(), 1000);
    }

    #[test]
    fn test_large_random_set_f64() {
        let mut rng = rand::thread_rng();

        let origin = Point {
            lat: 0.0f64,
            lon: 0.0f64,
            data: UserData {
                id: 0,
                score: 0.0,
                category: 0,
            },
        };

        // Generate 500 random points with f64
        let mut points = Vec::new();
        for i in 0..500000 {
            points.push(Point {
                lat: rng.gen_range(-100.0..100.0),
                lon: rng.gen_range(-100.0..100.0),
                data: UserData {
                    id: i,
                    score: rng.gen_range(0.0..1000.0),
                    category: rng.gen_range(0..5),
                },
            });
        }

        let result = nearest_points(points, origin);

        // Verify sorted
        for i in 0..result.len() - 1 {
            assert!(
                result[i].1 <= result[i + 1].1,
                "f64 set not sorted at index {}",
                i
            );
        }

        assert_eq!(result.len(), 500000);
    }

    #[test]
    fn test_duplicate_distances() {
        let origin = Point {
            lat: 0i32,
            lon: 0i32,
            data: UserData {
                id: 0,
                score: 0.0,
                category: 0,
            },
        };

        // Points on a circle - all same distance
        let points = vec![
            Point {
                lat: 3,
                lon: 4,
                data: UserData {
                    id: 1,
                    score: 1.0,
                    category: 1,
                },
            },
            Point {
                lat: -3,
                lon: -4,
                data: UserData {
                    id: 2,
                    score: 2.0,
                    category: 2,
                },
            },
            Point {
                lat: 4,
                lon: 3,
                data: UserData {
                    id: 3,
                    score: 3.0,
                    category: 3,
                },
            },
            Point {
                lat: -4,
                lon: -3,
                data: UserData {
                    id: 4,
                    score: 4.0,
                    category: 4,
                },
            },
        ];

        let result = nearest_points(points, origin);

        // All should have distance 5
        for (_, dist) in &result {
            assert!(
                (dist - 5.0).abs() < 0.01,
                "Expected distance ~5.0, got {}",
                dist
            );
        }

        assert_eq!(result.len(), 4);
    }

    #[test]
    fn test_single_point() {
        let origin = Point {
            lat: 5i32,
            lon: 10i32,
            data: UserData {
                id: 0,
                score: 0.0,
                category: 0,
            },
        };

        let points = vec![Point {
            lat: 7,
            lon: 12,
            data: UserData {
                id: 1,
                score: 99.9,
                category: 9,
            },
        }];

        let result = nearest_points(points, origin);

        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_empty_vec() {
        let origin = Point {
            lat: 0i32,
            lon: 0i32,
            data: UserData {
                id: 0,
                score: 0.0,
                category: 0,
            },
        };

        let points: Vec<Point<i32, UserData>> = vec![];

        let result = nearest_points(points, origin);

        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_very_large_coordinates() {
        let origin = Point {
            lat: 0i32,
            lon: 0i32,
            data: UserData {
                id: 0,
                score: 0.0,
                category: 0,
            },
        };

        let points = vec![
            Point {
                lat: 10000,
                lon: 10000,
                data: UserData {
                    id: 1,
                    score: 1.0,
                    category: 1,
                },
            },
            Point {
                lat: -10000,
                lon: -10000,
                data: UserData {
                    id: 2,
                    score: 2.0,
                    category: 2,
                },
            },
            Point {
                lat: 5000,
                lon: 5000,
                data: UserData {
                    id: 3,
                    score: 3.0,
                    category: 3,
                },
            },
        ];

        let result = nearest_points(points, origin);

        for i in 0..result.len() - 1 {
            assert!(result[i].1 <= result[i + 1].1);
        }
    }

    #[test]
    fn test_stress_with_i16() {
        let mut rng = rand::thread_rng();

        let origin = Point {
            lat: 0i16,
            lon: 0i16,
            data: UserData {
                id: 0,
                score: 0.0,
                category: 0,
            },
        };

        let mut points = Vec::new();
        for i in 0..500 {
            points.push(Point {
                lat: rng.gen_range(-1000..1000),
                lon: rng.gen_range(-1000..1000),
                data: UserData {
                    id: i,
                    score: rng.gen_range(0.0..50.0),
                    category: rng.gen_range(0..3),
                },
            });
        }

        let result = nearest_points(points, origin);

        for i in 0..result.len() - 1 {
            assert!(result[i].1 <= result[i + 1].1);
        }
        assert_eq!(result.len(), 500);
    }

    #[test]
    fn test_stress_with_f32_negative() {
        let mut rng = rand::thread_rng();

        let origin = Point {
            lat: -50.0f32,
            lon: -50.0f32,
            data: UserData {
                id: 999,
                score: 100.0,
                category: 9,
            },
        };

        let mut points = Vec::new();
        for i in 0..300 {
            points.push(Point {
                lat: rng.gen_range(-200.0..200.0),
                lon: rng.gen_range(-200.0..200.0),
                data: UserData {
                    id: i,
                    score: rng.gen_range(-50.0..50.0),
                    category: rng.gen_range(0..20),
                },
            });
        }

        let result = nearest_points(points, origin);

        for i in 0..result.len() - 1 {
            assert!(
                result[i].1 <= result[i + 1].1,
                "Failed at index {} with distances {} and {}",
                i,
                result[i].1,
                result[i + 1].1
            );
        }

        assert_eq!(result.len(), 300);
    }
}
