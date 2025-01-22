use crossterm::{
    cursor, execute, style, terminal,
    event::{self, Event, KeyCode},
};
use std::{io::{stdout, Write}, collections::VecDeque};
use std::time::Duration;
use rand::Rng;

const WIDTH: u16 = 20;
const HEIGHT: u16 = 20;

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq)]
struct Point {
    x: u16,
    y: u16,
}

struct SnakeGame {
    snake: VecDeque<Point>,
    direction: Direction,
    food: Point,
    score: u32,
    game_over: bool,
}

impl SnakeGame {
    fn new() -> Self {
        let mut snake = VecDeque::new();
        snake.push_back(Point { x: WIDTH / 2, y: HEIGHT / 2 });

        SnakeGame {
            snake,
            direction: Direction::Right,
            food: Point { x: 5, y: 5 },
            score: 0,
            game_over: false,
        }
    }

    fn move_snake(&mut self) {
        if self.game_over {
            return;
        }

        let head = *self.snake.front().unwrap();
        let new_head = match self.direction {
            Direction::Up => Point { x: head.x, y: head.y - 1 },
            Direction::Down => Point { x: head.x, y: head.y + 1 },
            Direction::Left => Point { x: head.x - 1, y: head.y },
            Direction::Right => Point { x: head.x + 1, y: head.y },
        };

        // Verificar colisiones con los bordes
        if new_head.x == 0 || new_head.x >= WIDTH || new_head.y == 0 || new_head.y >= HEIGHT {
            self.game_over = true;
            return;
        }

        // Verificar colisión consigo mismo
        if self.snake.contains(&new_head) {
            self.game_over = true;
            return;
        }

        // Agregar la nueva cabeza
        self.snake.push_front(new_head);

        // Comer la comida
        if new_head == self.food {
            self.score += 1;
            self.food = Point {
                x: rand::thread_rng().gen_range(1..WIDTH),
                y: rand::thread_rng().gen_range(1..HEIGHT),
            };
        } else {
            // Eliminar la cola si no se comió
            self.snake.pop_back();
        }
    }

    fn change_direction(&mut self, new_direction: Direction) {
        if self.game_over {
            return;
        }

        // Evitar moverse en dirección opuesta
        self.direction = match (self.direction, new_direction) {
            (Direction::Up, Direction::Down) | (Direction::Down, Direction::Up)
            | (Direction::Left, Direction::Right) | (Direction::Right, Direction::Left) => self.direction,
            _ => new_direction,
        };
    }

    fn draw_borders(&self) {
        let mut stdout = stdout();

        // Dibujar bordes superiores e inferiores
        for x in 0..=WIDTH + 1 {
            execute!(stdout, cursor::MoveTo(x, 0), style::Print("#")).unwrap();
            execute!(stdout, cursor::MoveTo(x, HEIGHT + 1), style::Print("#")).unwrap();
        }

        // Dibujar bordes laterales
        for y in 0..=HEIGHT + 1 {
            execute!(stdout, cursor::MoveTo(0, y), style::Print("#")).unwrap();
            execute!(stdout, cursor::MoveTo(WIDTH + 1, y), style::Print("#")).unwrap();
        }
    }

    fn draw(&self) {
        let mut stdout = stdout();
        execute!(stdout, terminal::Clear(terminal::ClearType::All)).unwrap();

        // Dibujar bordes
        self.draw_borders();

        // Dibujar comida
        execute!(
            stdout,
            cursor::MoveTo(self.food.x, self.food.y),
            style::Print("🍎")
        )
            .unwrap();

        // Dibujar snake
        for point in &self.snake {
            execute!(
                stdout,
                cursor::MoveTo(point.x, point.y),
                style::Print("⬛")
            )
                .unwrap();
        }

        // Dibujar puntaje
        execute!(
            stdout,
            cursor::MoveTo(0, HEIGHT + 2),
            style::Print(format!("Score: {}", self.score))
        )
            .unwrap();

        stdout.flush().unwrap();
    }
}

fn main() {
    let mut stdout = stdout();
    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide).unwrap();
    terminal::enable_raw_mode().unwrap();

    let mut game = SnakeGame::new();

    loop {
        // Manejo de eventos
        if let Ok(true) = event::poll(Duration::from_millis(1000)) {
            if let Event::Key(key_event) = event::read().unwrap() {
                match key_event.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Up => game.change_direction(Direction::Up),
                    KeyCode::Down => game.change_direction(Direction::Down),
                    KeyCode::Left => game.change_direction(Direction::Left),
                    KeyCode::Right => game.change_direction(Direction::Right),
                    _ => {}
                }

                // Mover la serpiente solo después de presionar una tecla de dirección
                game.move_snake();
                game.draw();
            }
        }

        if game.game_over {
            execute!(
                stdout,
                cursor::MoveTo(0, HEIGHT + 3),
                style::Print("¡Juego terminado! Presiona 'q' para salir.")
            )
                .unwrap();
        }
    }

    terminal::disable_raw_mode().unwrap();
    execute!(stdout, cursor::Show, terminal::LeaveAlternateScreen).unwrap();
}

