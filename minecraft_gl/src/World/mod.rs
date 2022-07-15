use self::item::Item;

pub mod crafting;
pub mod block;
pub mod blockBehavior;
pub mod item;
pub mod itemBehavior;
pub mod chunk;
pub mod world;

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

#[derive(Clone)]
pub enum State {
    Container((Vec<Item>, u32, u32)),
    DynamicContainer(Vec<Item>),
    FloatAttribute(f32),
    IntAttribute(i32),
    BoolAttribute(bool)
}


impl State{
    pub fn AsInt(&self) -> Option<&i32>{
        if let Self::IntAttribute(val) = self {
            return Some(val);
        }
        None
    }

    pub fn AsFloat(&self) -> Option<&f32>{
        if let Self::FloatAttribute(val) = self {
            return Some(val);
        }
        None
    }

    pub fn AsArray(&self) -> Option<(&Vec<Item>, u32, u32)>{
        if let Self::Container(val) = self {
            //Vector, Rows, Columns
            return Some((&val.0, val.1, val.2));
        }
        None
    }

    pub fn AsBool(&self) -> Option<&bool>{
        if let Self::BoolAttribute(val) = self {
            return Some(val);
        }
        None
    }

    pub fn StateType(stateType: &str) -> Result<State, String> {
        match stateType {
            "DynamicContainer" => return Ok(State::DynamicContainer(Vec::new())),
            "Int" => return Ok(State::IntAttribute(0)),
            "Float" => return Ok(State::FloatAttribute(0f32)),
            "Bool" => return Ok(State::BoolAttribute(false)),
            _ => return Err(format!("Error! State type string of {} is invalid!", stateType))
        }
    }
}

pub enum StateType {
    Container = 0,
    FloatAttribute,
    IntAttribute,
    BoolAttribute
}