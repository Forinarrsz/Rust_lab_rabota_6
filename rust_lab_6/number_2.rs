const WIDTH: usize = 80;
const HEIGHT: usize = 30;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum CanvasChar {
    #[default]
    Outer,
    Border,
    Inner,
}

impl CanvasChar {
    fn to_char(&self) -> char {
        match self {
            CanvasChar::Outer => ' ',
            CanvasChar::Border => '#',
            CanvasChar::Inner => '.',
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Canvas {
    buffer: [[CanvasChar; WIDTH]; HEIGHT],
}

impl Canvas {
    fn new() -> Self {
        Canvas {
            buffer: [[CanvasChar::Outer; WIDTH]; HEIGHT],
        }
    }

    fn output(&self) -> String {
        let mut output = String::new();
        for row in &self.buffer {
            for &cell in row {
                output.push(cell.to_char());
            }
            output.push('\n');
        }
        output
    }

    fn draw(&mut self, x: isize, y: isize, buffer: &Vec<Vec<CanvasChar>>) {
        for (i, row) in buffer.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                let canvas_x = x + j as isize;
                let canvas_y = y + i as isize;

                if canvas_x >= 0 && canvas_x < WIDTH as isize && canvas_y >= 0 && canvas_y < HEIGHT as isize {
                    let canvas_x = canvas_x as usize;
                    let canvas_y = canvas_y as usize;

                    if self.buffer[canvas_y][canvas_x] == CanvasChar::Outer && (cell == CanvasChar::Border || cell == CanvasChar::Inner) {
                        self.buffer[canvas_y][canvas_x] = cell;
                    } else if self.buffer[canvas_y][canvas_x] == CanvasChar::Border && cell == CanvasChar::Inner {
                        self.buffer[canvas_y][canvas_x] = cell;
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Rectangle {
    pub width: usize,
    pub height: usize,
}

impl Rectangle {
    pub fn new(width: usize, height: usize) -> Self {
        Rectangle { width, height }
    }
}

impl Renderable for Rectangle {
    fn render(&self) -> Vec<Vec<CanvasChar>> {
        let mut buffer = vec![vec![CanvasChar::Inner; self.width]; self.height];

        for i in 0..self.height {
            buffer[i][0] = CanvasChar::Border;
            buffer[i][self.width - 1] = CanvasChar::Border;
        }

        for j in 0..self.width {
            buffer[0][j] = CanvasChar::Border;
            buffer[self.height - 1][j] = CanvasChar::Border;
        }

        buffer
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub radius: usize,
}

impl Circle {
    pub fn new(radius: usize) -> Self {
        Circle { radius }
    }
}

impl Renderable for Circle {
    fn render(&self) -> Vec<Vec<CanvasChar>> {
        let diameter = 2 * self.radius + 1;
        let mut buffer = vec![vec![CanvasChar::Outer; diameter]; diameter];
        let r_squared = (self.radius * self.radius) as f64;

        for y in 0..diameter {
            for x in 0..diameter {
                let dx = x as f64 - self.radius as f64;
                let dy = y as f64 - self.radius as f64;
                let dist_squared = dx * dx + dy * dy;

                if dist_squared < r_squared - self.radius as f64 {
                    buffer[y][x] = CanvasChar::Inner;
                } else if dist_squared >= r_squared - self.radius as f64 && dist_squared <= r_squared + self.radius as f64{
                    buffer[y][x] = CanvasChar::Border;
                }

            }
        }
        buffer
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    width: usize,
    height: usize,
}

impl Triangle {
    pub fn new(width: usize, height: usize) -> Self {
        Triangle { width, height }
    }
}

impl Renderable for Triangle {
    fn render(&self) -> Vec<Vec<CanvasChar>> {
        let mut buffer = vec![vec![CanvasChar::Outer; self.width]; self.height];

        for y in 0..self.height {
            for x in 0..self.width {
                if y as f64 >= (self.height as f64 / self.width as f64) * (self.width as f64 - 1.0 - x as f64)
                    && y as f64 >= (self.height as f64 / self.width as f64) * x as f64
                    || y == self.height -1 {
                        if y == self.height - 1 {
                            buffer[y][x] = CanvasChar::Border
                        } else {
                    buffer[y][x] = CanvasChar::Inner;
                        }
                }
             }
         }

        for i in 0 .. self.width {
             buffer[self.height - 1][i] = CanvasChar::Border;
        }

                let slope = self.height as f64 / (self.width as f64 / 2.0);

        for y_iter in 0..self.height {
            let x1 = self.width as f64 / 2.0;
            let y1 = 0.0;
            let dy = (self.height - 1) as f64;
            let dx = (self.width /2 ) as f64;
            for x_iter in 0..self.width {
                 let y_alg = (dy / dx) * ((x_iter) as f64 - x1) + y1;

                if  (y_alg as i32)  == (y_iter as i32) { 
                    buffer[y_iter][x_iter] = CanvasChar::Border;
                }
            }
        }

        buffer
    }
}

pub trait Renderable {
    fn render(&self) -> Vec<Vec<CanvasChar>>;
}

pub fn main() {
    let mut canvas = Canvas::new();

    let circle = Circle::new(7);
    let rectangle = Rectangle::new(40, 7);
    let triangle = Triangle::new(20, 7);

    canvas.draw(40, 5, &circle.render());
    canvas.draw(20, 9, &rectangle.render());
    canvas.draw(20, 5, &triangle.render());
    canvas.draw(25, 17, &triangle.render());

    println!("{}", canvas.output());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn count_chars(buffer: &Vec<Vec<CanvasChar>>, target: CanvasChar) -> usize {
        buffer.iter().flatten().filter(|&&c| c == target).count()
    }

    #[test]
    fn test_rectangle_render() {
        let rect = Rectangle::new(5, 4);
        let buf = rect.render();

        assert_eq!(buf.len(), 4);
        assert_eq!(buf[0].len(), 5);

        for x in 0..5 {
            assert_eq!(buf[0][x], CanvasChar::Border);
            assert_eq!(buf[3][x], CanvasChar::Border);
        }
        for y in 0..4 {
            assert_eq!(buf[y][0], CanvasChar::Border);
            assert_eq!(buf[y][4], CanvasChar::Border);
        }

        assert_eq!(buf[1][1], CanvasChar::Inner);
        assert_eq!(buf[2][3], CanvasChar::Inner);

        assert_eq!(count_chars(&buf, CanvasChar::Border), 14);
        assert_eq!(count_chars(&buf, CanvasChar::Inner), 6);
    }

    #[test]
    fn test_circle_render_basic() {
        let circle = Circle::new(3);
        let buf = circle.render();

        assert_eq!(buf.len(), 7);
        assert_eq!(buf[0].len(), 7);

        let c = buf[3][3];
        assert_eq!(c, CanvasChar::Inner);

        assert!(count_chars(&buf, CanvasChar::Border) > 0);
        assert!(count_chars(&buf, CanvasChar::Inner) > 0);
    }

    #[test]
    fn test_triangle_render_dimensions() {
        let tri = Triangle::new(7, 5);
        let buf = tri.render();

        assert_eq!(buf.len(), 5);
        assert_eq!(buf[0].len(), 7);

        for c in &buf[4] {
            assert_eq!(*c, CanvasChar::Border);
        }
    }

    #[test]
    fn test_canvas_new_and_output_empty() {
        let c = Canvas::new();
        let s = c.output();

        let lines: Vec<_> = s.lines().collect();
        assert_eq!(lines.len(), HEIGHT);
        assert!(lines.iter().all(|l| l.len() == WIDTH));

        assert!(s.chars().all(|ch| ch == ' ' || ch == '\n'));
    }

    #[test]
    fn test_canvas_draw_rectangle() {
        let mut canvas = Canvas::new();
        let rect = Rectangle::new(5, 3);
        let buf = rect.render();

        canvas.draw(2, 2, &buf);

        let out = canvas.output();

        let lines: Vec<&str> = out.lines().collect();
        let mut border_count = 0;
        for y in 2..5 {
            for x in 2..7 {
                if lines[y].chars().nth(x).unwrap() == '#' {
                    border_count += 1;
                }
            }
        }

        assert!(border_count > 0);
    }

    #[test]
    fn test_canvas_draw_overlapping_shapes() {
        let mut canvas = Canvas::new();
        let rect = Rectangle::new(7, 5);
        let circle = Circle::new(4);

        let rect_buf = rect.render();
        let circle_buf = circle.render();

        canvas.draw(0, 0, &rect_buf);
        canvas.draw(2, 1, &circle_buf);

        let out = canvas.output();

        let buffer = out
            .split('\n')
            .map(|str| str.to_string())
            .collect::<Vec<String>>();

        assert_eq!(buffer[2].chars().nth(6).unwrap(), '.');
        assert_eq!(buffer[4].chars().nth(6).unwrap(), '.');
    }

    #[test]
    fn test_canvas_draw_out_of_bounds() {
        let mut canvas = Canvas::new();
        let rect = Rectangle::new(10, 5);
        let buf = rect.render();

        canvas.draw(-3, -2, &buf);
        let out = canvas.output();

        assert!(out.contains('#'));
    }

    #[test]
    fn test_combined_shapes_in_canvas() {
        let mut canvas = Canvas::new();

        let rect = Rectangle::new(8, 4);
        let circle = Circle::new(3);
        let tri = Triangle::new(9, 5);

        canvas.draw(1, 1, &rect.render());
        canvas.draw(10, 2, &circle.render());
        canvas.draw(20, 3, &tri.render());

        let output = canvas.output();

        assert!(output.contains('#'));
        assert!(output.contains('.'));
        assert!(output.lines().count() == HEIGHT);
    }
}
