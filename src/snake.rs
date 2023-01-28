use crate::direction::Direction;
use crate::point::Point;

#[derive(Debug)]
pub struct Snake {
    body: Vec<Point>,
    direction: Direction,
    digesting: bool,
}

impl Snake {
    pub fn new(start: Point, length: u16, direction: Direction) -> Self {
        let opposite = direction.opposite();
        let body: Vec<Point> = (0..length)
            .into_iter()
            .map(|i| start.transform(opposite, i))
            .collect();

        Self {body, direction, digesting: false}
    }

    pub fn get_head_point(&self) -> Point {
        self.body.first().unwrap().clone()
    }

    pub fn get_body_points(&self) -> Vec<Point> {
        self.body.clone() // Return a clone of the body field
    }

    pub fn get_direction(&self) -> Direction {
        self.direction.clone() // Return a clone of the direction field 
    }

    pub fn contains_point(&self, point: &Point) -> bool {
        self.body.contains(point) // Tells whether the snake body contains the point referenced in the params
    }

    pub fn slither(&mut self) {
        self.body.insert(0, self.body.first().unwrap().transform(self.direction, 1));
        if !self.digesting {
            self.body.remove(self.body.len() - 1); // Remove one part of the body
        } else {
            self.digesting = false;
        }
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction // Set the direction the snake is moving 
    }

    pub fn grow(&mut self) {
        self.digesting = true // Add another point to the body / increase the snakes size
    }
}