#[derive(Debug, Clone, Copy)]
pub struct Rectangle {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub angle: f64,
}

impl Rectangle {
    pub fn new(x: f64, y: f64, width: f64, height: f64, angle: f64) -> Self {
        Rectangle {
            x,
            y,
            width,
            height,
            angle,
        }
    }

    pub fn vertices(&self) -> Vec<Corner> {
        let offsets = [
            (self.width / 2., self.height / 2.),
            (self.width / 2., -self.height / 2.),
            (-self.width / 2., -self.height / 2.),
            (-self.width / 2., self.height / 2.),
        ];

        let (cx, cy) = (self.x, self.y);

        offsets
            .iter()
            .map(|&(x, y)| {
                let (x, y) = (
                    x * self.angle.cos() - y * self.angle.sin(),
                    x * self.angle.sin() + y * self.angle.cos(),
                );
                Corner {
                    x: cx + x,
                    y: cy + y,
                }
            })
            .collect::<Vec<Corner>>()
    }

    pub fn edges(&self) -> Vec<Edge> {
        let mut edges = Vec::new();
        let vertices = self.vertices();

        for i in 0..vertices.len() {
            let edge = Edge {
                start: vertices[i],
                end: vertices[(i + 1) % vertices.len()],
            };
            edges.push(edge);
        }
        edges
    }

    pub fn intersects(&self, rect: &Rectangle) -> bool {
        let mut min_a: f64 = f64::INFINITY;
        let mut max_a: f64 = f64::NEG_INFINITY;
        let mut min_b: f64 = f64::INFINITY;
        let mut max_b: f64 = f64::NEG_INFINITY;

        for edge in self.edges() {
            let normal = edge.normal();
            for vertex in self.vertices() {
                let projected = normal.x * vertex.x + normal.y * vertex.y;
                min_a = min_a.min(projected);
                max_a = max_a.max(projected);
            }
            for vertex in rect.vertices() {
                let projected = normal.x * vertex.x + normal.y * vertex.y;
                min_b = min_b.min(projected);
                max_b = max_b.max(projected);
            }

            if max_a < min_b || max_b < min_a {
                return false;
            }
        }
        for edge in rect.edges() {
            let normal = edge.normal();
            for vertex in self.vertices() {
                let projected = normal.x * vertex.x + normal.y * vertex.y;
                min_a = min_a.min(projected);
                max_a = max_a.max(projected);
            }
            for vertex in rect.vertices() {
                let projected = normal.x * vertex.x + normal.y * vertex.y;
                min_b = min_b.min(projected);
                max_b = max_b.max(projected);
            }

            if max_a < min_b || max_b < min_a {
                return false;
            }
        }
        true
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Corner {
    pub x: f64,
    pub y: f64,
}

impl std::fmt::Display for Corner {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Corner({:.4}, {:.4})", self.x, self.y)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Edge {
    pub start: Corner,
    pub end: Corner,
}

impl Edge {
    pub fn slope(&self) -> f64 {
        if (self.start.x - self.end.x).abs() < 1e-7 {
            f64::INFINITY
        } else {
            (self.end.y - self.start.y) / (self.end.x - self.start.x)
        }
    }
    pub fn normal(&self) -> Corner {
        Corner {
            y: self.end.y - self.start.y,
            x: self.start.x - self.end.x,
        }
    }

    pub fn y_intercept(&self) -> f64 {
        let slope = self.slope();
        if slope == f64::INFINITY {
            f64::INFINITY
        } else {
            self.start.y - slope * self.start.x
        }
    }
}

impl std::fmt::Display for Edge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.slope() == f64::INFINITY {
            write!(
                f,
                "Edge({{ start: {:.4}, end: {:.4} }}) (Line: x = {:.4})",
                self.start, self.end, self.start.x
            )
        } else {
            write!(
                f,
                "Edge({{ start: {:.4}, end: {:.4}, normal: {} }}) (Line: y = {:.4} * x +  {:.4})",
                self.start,
                self.end,
                self.normal(),
                self.slope(),
                self.y_intercept()
            )
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn test_oriented_rectangle_collision() {
        let rect1 = Rectangle::new(0., 0., 10., 6., 0.);
        let rect2 = Rectangle::new(0., 0., 6., 10., std::f64::consts::TAU);
        assert!(rect1.intersects(&rect2));
    }
}
