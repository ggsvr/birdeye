use crate::math::*;

pub struct Robot {
    pub back: Point, 
    pub front: Point,
}

impl Robot {
    //create new instance from points
    pub fn new(back: Point, front: Point) -> Robot {
        Robot {
            front,
            back
        }
    }

    // get direction vector of robot
    pub fn dir(&self) -> Vector {
        //Vector::from_points(self.back, self.front) // P2-P1 = P1->P2
        self.front-self.back
    }

    // robot length
    pub fn len(&self) -> f64{ 
        self.front.distance(self.back)
    }

    // robot center point
    pub fn center(&self) -> Point { 
        self.front.midpoint(self.back)
    }

    // swap front and back
    pub fn swap(&mut self) {
        let tmp = self.back;
        self.back = self.front;
        self.front = tmp;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    const P1: Point = Point { x: 5., y: 10. };
    const P2: Point = Point { x: 12., y: 4. };
    const P3: Point = Point { x: 20., y: 7. };
    const P4: Point = Point { x: 2., y: 3. };

    #[test]
    fn robot_dir() {
        let robot1 = Robot::new(P1, P2);
        let robot2 = Robot::new(P3, P4);

        assert_eq!(robot1.dir(), Vector::new(7., -6.));
        assert_eq!(robot2.dir(), Vector::new(-18., -4.));
    }

    #[test]
    fn robot_len() {
        let robot1 = Robot::new(P2, P4);
        let robot2 = Robot::new(P3, P1);
        let robot3 = Robot::new(P1, P1);

        assert_eq!(robot1.len().floor(), 10.);
        assert_eq!(robot2.len().floor(), 15.);
        assert_eq!(robot3.len().floor(), 0.);
    }

    #[test]
    fn robot_center() {
        let robot1 = Robot::new(P1, P3);
        let robot2 = Robot::new(P4, P1);

        assert_eq!(robot1.center(), Point::new(12.5, 8.5));
        assert_eq!(robot2.center(), Point::new(3.5, 6.5));
    }
}