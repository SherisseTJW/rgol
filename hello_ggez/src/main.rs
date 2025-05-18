use ggez::*;

struct State {
    dt: std::time::Duration,
}

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.dt = ctx.time.delta();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        println!("Hello ggez! dt = {}ns", self.dt.as_nanos());
        Ok(())
    }
}

fn main() {
    println!("Hello, world!");

    let state = State {};
    let c = conf::Conf::new();

    let (ctx, event_loop) = ContextBuilder::new("hello_ggez", "awesome_person")
        .default_conf(c)
        .build()
        .unwrap();

    event::run(ctx, event_loop, state);
}
