use crate::math::Point;
use super::Data;

pub struct PointData {
    pub points: [Option<Point>; 3],
}

impl Data for PointData {
    type Inner = Option<Point>;

    fn list(&self) -> &[Self::Inner] {
        &self.points
    }
    fn list_mut(&mut self) -> &mut [Self::Inner] {
        &mut self.points
    }
}