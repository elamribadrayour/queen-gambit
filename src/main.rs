mod chess;

use chess::Board;

use rand::RngCore;

use nannou::prelude::*;

const NB_QUEENS: usize = 16;
const BOARD_SIZE: usize = 16;

const MUTATION_LIKELIHOOD: f32 = 0.4;
const CROSSOVER_LIKELIHOOD: f32 = 0.75;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    board: Board,
    iteration: usize,
    fitnesses: Vec<f32>,
    rng: Box<dyn RngCore>,
}

fn model(app: &App) -> Model {
    app.new_window().view(view).size(800, 800).build().unwrap();

    let mut rng = Box::new(rand::thread_rng());
    let board = Board::new(
        app.window_rect(),
        &mut *rng,
        BOARD_SIZE,
        NB_QUEENS,
        MUTATION_LIKELIHOOD,
        CROSSOVER_LIKELIHOOD,
    );
    Model {
        board,
        iteration: 0,
        fitnesses: vec![],
        rng,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    if model.board.fitness() == 1.0 {
        return;
    }

    model.iteration += 1;
    model.board.evaluate();
    model.board.crossover(&mut *model.rng);
    model.board.mutate(&mut *model.rng);
    model.fitnesses.push(model.board.fitness());

    // Calculate statistics
    let average_score = model.fitnesses.iter().sum::<f32>() / model.fitnesses.len() as f32;
    let average_score_last_5: f32 = model.fitnesses.iter().rev().take(5).sum::<f32>() / 5.0;
    let best_score = model.fitnesses.iter().cloned().fold(f32::MIN, f32::max);
    let worst_score = model.fitnesses.iter().cloned().fold(f32::MAX, f32::min);

    println!(
        "iteration: {}\t len: {}\t worst: {:.2}\t best: {:.2}\t avg: {:.2}\t avg last 5: {:.2}\t fitness: {:.2}",
        model.iteration, model.board.queens().count(), worst_score, best_score, average_score, average_score_last_5, model.fitnesses.last().unwrap()
    );
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);

    let win = app.window_rect();
    let square_size = win.w().min(win.h()) / model.board.board_size as f32;

    model.board.squares().for_each(|square| {
        draw.rect()
            .x_y(square.x, square.y)
            .w_h(square_size, square_size)
            .color(square.color);
    });

    model.board.queens().for_each(|queen| {
        let position = model.board.position(queen.position());
        draw.texture(&queen.texture(app))
            .x_y(position.0, position.1)
            .w_h(square_size, square_size);
    });

    draw.text(&format!(
        "iteration: {} -  fitness: {:.2}",
        model.iteration,
        model.board.fitness()
    ))
    .x_y(win.left() + 100.0, win.bottom() + 20.0)
    .color(GREEN);

    draw.to_frame(app, &frame).unwrap();
}
