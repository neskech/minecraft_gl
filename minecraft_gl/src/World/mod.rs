pub mod crafting;
pub mod block;
pub mod blockBehavior;
pub mod item;
pub mod itemBehavior;
pub mod chunk;
pub mod world;
use self::{item::{ItemRegistry, ItemStack, ItemID}, block::{BlockRegistry, Block}, crafting::CraftingRegistry};

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//// //////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//// //////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////


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

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// //////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// //////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// //////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub enum State {
    Container((Vec<ItemStack>, u32, u32)),
    DynamicContainer(Vec<ItemStack>),
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

    pub fn AsArray(&self) -> Option<(&Vec<ItemStack>, u32, u32)>{
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
            "Dynamic Container" => return Ok(State::DynamicContainer(Vec::new())),
            "Int" => return Ok(State::IntAttribute(0)),
            "Float" => return Ok(State::FloatAttribute(0f32)),
            "Bool" => return Ok(State::BoolAttribute(false)),
            _ => return Err(format!("Error! State type string of {} is invalid!", stateType))
        }
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//// //////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//// //////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[macro_export]
macro_rules! CreateBinding {
    (
        $bindingFunctionName:ident,
        $(  
            pub fn $funcName:ident ($( $arg_name:ident : $arg_ty:ty ),* $(,)?)
             { $($code:tt)* } 
        )*
    ) => {

         $(
            pub fn $funcName ($($arg_name : $arg_ty),*) 
            { $($code)* }
            
         )*

         pub fn $bindingFunctionName (registry: &mut BlockRegistry) {
            $(
                $funcName (registry);
            )*
         }
    };
}

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//// //////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//// //////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn ReadAttributes(blockRegistry: &mut BlockRegistry, itemRegistry: &mut ItemRegistry, craftingRegistry: &mut CraftingRegistry) -> Result<(), Box<dyn std::error::Error>>{
    let dataBlock = blockRegistry.ReadBlockAttributes()?;
    let dataItem = itemRegistry.ReadItemAttributes(craftingRegistry)?;

    for dropItem in dataBlock.0 {
        if itemRegistry.HasItem(dropItem.1.as_str()) {
            let id = itemRegistry.NameToID(dropItem.1.as_str());
            let attrib = blockRegistry.BlocksAttributes.get_mut(&dropItem.0).unwrap();
            attrib.DropItem = Some(ItemID::New(id));
        }
        else {
            return Err(GenericError::NewBoxed(
            format!("Invalid drop item for block {} of id {}. Item registry has no item called '{}'", 
            blockRegistry.GetAttributesOfID(dropItem.0).Name, dropItem.0, dropItem.1)));
        }
    }

    for effectiveTool in dataBlock.1 {
        if itemRegistry.HasItem(effectiveTool.1.as_str()) {
            let id = itemRegistry.NameToID(effectiveTool.1.as_str());
            let attrib = blockRegistry.BlocksAttributes.get_mut(&effectiveTool.0).unwrap();
            attrib.EffectiveTool = Some(ItemID::New(id));
        }
        else {
            return Err(GenericError::NewBoxed(
            format!("Invalid 'effective tool' item for block {} of id {}. Item registry has no item called '{}'", 
            blockRegistry.GetAttributesOfID(effectiveTool.0).Name, effectiveTool.0, effectiveTool.1)));
        }
    }

    for placeBlock in dataItem {
        if blockRegistry.HasBlock(placeBlock.1.as_str()) {
            let id = blockRegistry.NameToID(placeBlock.1.as_str());
            let attrib = itemRegistry.ItemAttributes.get_mut(&placeBlock.0).unwrap();
            attrib.PlaceableBlock = Some(Block { ID: id} );
        }
        else {
            return Err(GenericError::NewBoxed(
            format!("Invalid placeable block for item {} of id {}. Block registry has no item called '{}'", 
            itemRegistry.GetAttributesOfID(placeBlock.0).Name, placeBlock.0, placeBlock.1)));
        }
    }
    Ok(())
}