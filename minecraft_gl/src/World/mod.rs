pub mod crafting;
pub mod block;
pub mod blockBehavior;
pub mod item;
pub mod itemBehavior;

#[derive(Debug)]
struct GenericError{
    Msg: String
}

impl GenericError{
    pub fn New(msg: String) -> Self {
        Self {
            Msg: msg
        }
    }

    pub fn NewBoxed(msg: String) -> Box<Self> {
        Box::new(
            Self {
            Msg: msg
          }
        )
    }
    
}

impl std::error::Error for GenericError {}

impl std::fmt::Display for GenericError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.Msg)
    }
}