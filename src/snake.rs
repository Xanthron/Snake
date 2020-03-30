use crate::direction::Direction;
use crate::point::Point;

pub struct Snake {
    pub points: Vec<Point>,
}

impl Snake {
    pub fn move_dir(mut self, dir: Direction) {
        let current = self.points[0];
        let mut next = match dir {
            Up => Point::new(current.x, current.y - 1),
            Down => Point::new(current.x, current.y + 1),
            Left => Point::new(current.x - 1, current.y),
            Right => Point::new(current.x + 1, current.y),
        };

        for i in 0..self.points.len() {
            let temp = self.points[i];
            self.points[i] = next;
            next = temp;
        }
    }

    pub fn eat_apple(mut self) {
        let last = self.points[self.points.len()];
        self.points.push(Point::new(last.x, last.y));
    }

    fn check_eat_self(self) -> bool {
        let hat = self.points[0];
        for i in 1..self.points.len() {
            if self.points[i] == hat {
                return true;
            }
        }
        return false;
    }

    fn check_out_of_boarder(self) -> bool {
        let hat = self.points[0];
        if 
    }
}
