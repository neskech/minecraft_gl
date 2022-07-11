
pub enum Scene{
    Minecraft(),
    SphereCraft(),
    MainMenu(),
}

pub trait SceneTrait {
    fn Update(&mut self, timeStep: f32);
    fn Render(&mut self);
}