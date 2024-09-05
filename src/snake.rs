use std::collections::VecDeque;
use crate::point::Point;

#[derive(PartialEq)]
pub enum SnakeDirection {
    Up,
    Down,
    Left,
    Right,
}

pub struct Snake {
    pub body: VecDeque<Point>,
    pub direction: SnakeDirection,
}

impl Snake {
    pub fn new() -> Self {
        return Self {
            body: VecDeque::from(vec![Point::new(0, 8), Point::new(1, 8)]),
            direction: SnakeDirection::Right,
        };
    }

    pub fn get_head_coordinates<'a>(&self) -> Result<&Point, &'a str> {
        return self.body.back().ok_or("Could not get snake coordinates");
    }

    pub fn get_tail_coordinates<'a>(&self) -> Result<&Point, &'a str> {
        return self.body.front().ok_or("Could not get snake coordinates");
    }

    pub fn change_direction(&mut self, direction: SnakeDirection) {
        match direction {
            SnakeDirection::Up => {
                if self.direction != SnakeDirection::Down {
                    self.direction = direction
                }
            }
            SnakeDirection::Down => {
                if self.direction != SnakeDirection::Up {
                    self.direction = direction
                }
            }
            SnakeDirection::Left => {
                if self.direction != SnakeDirection::Right {
                    self.direction = direction
                }
            }
            SnakeDirection::Right => {
                if self.direction != SnakeDirection::Left {
                    self.direction = direction
                }
            }
        }
    }

    pub fn advance<'a>(&mut self) -> Result<Point, &'a str> {
        let old_head = self.get_head_coordinates()?;

        match self.direction {
            SnakeDirection::Up => {
                if old_head.y == 0 {
                    return Err("Snake will go out of bounds");
                }

                self.body.push_back(Point::new(old_head.x, old_head.y - 1));
            },
            SnakeDirection::Down => self.body.push_back(Point::new(old_head.x, old_head.y + 1)),
            SnakeDirection::Left => {
                if old_head.x == 0 {
                    return Err("Snake will go out of bounds");
                }

                self.body.push_back(Point::new(old_head.x - 1, old_head.y));
            },
            SnakeDirection::Right => self.body.push_back(Point::new(old_head.x + 1, old_head.y)),
        }

        self.body.remove(0);

        let new_head = self.get_head_coordinates()?;

        return Ok(*new_head);
    }

    pub fn ate_itself(&self) -> bool {
        let head = self.get_head_coordinates().expect("Error getting snake head");

        return self.body.iter().filter(|p| p.x == head.x && p.y == head.y).count() > 1;
    }
}
