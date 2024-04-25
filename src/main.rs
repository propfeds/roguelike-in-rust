use rltk::{Rltk, GameState};

struct State {}
impl GameState for State
{
    fn tick(&mut self, ctx: &mut Rltk)
    {
        ctx.cls();
        ctx.print(1, 1, "Hello Fellow Crabbos");
    }
}

fn main() -> rltk::BError
{
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
    .withTitle("Roguelike Tut")
    .build()?;
    let gs = State{ };
    rltk::main_loop(context, gs)
}