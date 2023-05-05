pub mod crafting;
pub mod block;
pub mod blockBehavior;
pub mod item;
pub mod itemBehavior;
pub mod chunk;
pub mod world;
mod biomeGenerator;
use std::{io::BufReader, collections::HashMap, marker::PhantomData};
use self::{item::{ItemRegistry, ItemStack, ItemID}, 
           block::{BlockRegistry, Block}, crafting::CraftingRegistry, 
           biomeGenerator::{BiomeGenerator, Biome, GenerationData, 
           HeightModifier, ForestGenerator}
          };

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
            let id = blockRegistry.NameToID(placeBlock.1.as_str()).expect(
                format!("could not find {} in block registry", placeBlock.1).as_ref());
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

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//// //////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//// //////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn ReadBiomeGenerators(blockRegistry: &BlockRegistry) -> Result<HashMap<Biome, Box<dyn BiomeGenerator + Send>>, Box<dyn std::error::Error>> {
    //TODO implement the capacity for item and block registries
    let path = std::path::Path::new("./minecraft_gl/assets/data/biome/");
    let dir = std::fs::read_dir(path)
    .map_err(|e| format!("Error! Could not find ./minecraft_gl/assets/data/biome/ directory! The error:\n{}", e.to_string()))?;
    
    let mut generators: HashMap<Biome, Box<dyn BiomeGenerator + Send>> = HashMap::with_capacity(dir.count());

    for file in std::fs::read_dir(path).unwrap() {
        let path = file
        .map_err(|e| format!("Error! Could not retrieve file in ../../assets/data/block/json/ directory! The error:\n{}", e.to_string()))?.path();

        let file = std::fs::File::open(path)
        .map_err(|e| GenericError::NewBoxed(format!("Error! Could not open file of path in ../../assets/data/block/json/ directory! The error:\n{}", e.to_string())))?;

        let json: serde_json::Value = serde_json::from_reader(BufReader::new(file))?;
        
        let mut genData = GenerationData {
            Crust: Vec::new(),
            Mantle: None,
            Core: Block::Air(),
            Ores: Vec::new(),
            MantleRange: (0u32, 0u32),
            HeightLevel: 0u32,
            SurfaceAmplitude: 0u32,
            SeaLevel: 0u32,
            CaveModifier: HeightModifier::default(),
            CaveCutoff: 0f32,
            OreCutoff: 0f32,
        };

        let name = json["Name"].as_str().unwrap();

        if let Some(val) = json.get("Cave") {
            if let Some(c) = val.get("Noise Cutoff") {
                genData.CaveCutoff = c.as_f64().unwrap() as f32;
            } else {
                return Err(GenericError::NewBoxed(
                    format!("The {} biome json has no property 'Noise Cuftoff' inside of 'Cave'. Fix the json file!", name)));
            }
            genData.CaveModifier = ReadHeightModifier(val, name, "Cave")?;
        }  else {
            return Err(GenericError::NewBoxed(
                format!("The {} biome json has no property 'cave'. Fix the json file!", name)));
        }

        if let Some(val) = json.get("Height Level") {
            genData.HeightLevel = val.as_u64().unwrap() as u32;
        }  else {
            return Err(GenericError::NewBoxed(
                format!("The {} biome json has no property 'Height Level'. Fix the json file!", name)));
        }

        if let Some(val) = json.get("Surface Amplitude") {
            genData.SurfaceAmplitude = val.as_u64().unwrap() as u32;
        }  else {
            return Err(GenericError::NewBoxed(
                format!("The {} biome json has no property 'Surface Amplitude'. Fix the json file!", name)));
        }

        if let Some(val) = json.get("Sea Level") {
            genData.SeaLevel = val.as_u64().unwrap() as u32;
        }  else {
            return Err(GenericError::NewBoxed(
                format!("The {} biome json has no property 'Sea Level'. Fix the json file!", name)));
        }

        //Read the block data
        if let Some(val) = json.get("Crust") {
            genData.Crust = ReadBlockList(val, name, "Blocks", blockRegistry)?;
        }  else {
            return Err(GenericError::NewBoxed(
                format!("The {} biome json has no property 'Blocks'. Fix the json file!", name)));
        }

        if let Some(val) = json.get("Mantle") {
            genData.Mantle = Some(Block { ID: blockRegistry.NameToID(val.as_str().unwrap()).expect(
                format!("could not find {} in block registry", val.as_str().unwrap()).as_ref()
            ) });
        }

        if let Some(val) = json.get("Mantle Min Length") {
            genData.MantleRange.0 = val.as_u64().unwrap() as u32;
        }  else {
            return Err(GenericError::NewBoxed(
                format!("The {} biome json has no property 'Mantle Min Length'. Fix the json file!", name)));
        }

        if let Some(val) = json.get("Mantle Max Length") {
            genData.MantleRange.1 = val.as_u64().unwrap() as u32;
        }  else {
            return Err(GenericError::NewBoxed(
                format!("The {} biome json has no property 'Mantle Max Length'. Fix the json file!", name)));
        }

        if let Some(val) = json.get("Core") {
            genData.Core = Block { ID: blockRegistry.NameToID(val.as_str().unwrap()).expect(
                format!("could not find {} in block registry", val.as_str().unwrap()).as_ref()
            ) };
        }  else {
            return Err(GenericError::NewBoxed(
                format!("The {} biome json has no property 'Core'. Fix the json file!", name)));
        }
        //Read the ore data
        if let Some(val) = json.get("Ores") {
            if let Some(v) = val.get("Noise Cutoff") {
                genData.OreCutoff = v.as_f64().unwrap() as f32;
            } else {
                return Err(GenericError::NewBoxed(
                    format!("The {} biome json has no property 'Noise Cuftoff' inside of 'Ores'. Fix the json file!", name)));
            }

            if let Some(v) = val.get("Blocks") {
                 genData.Ores = ReadBlockList(v, name, "Ore Blocks", blockRegistry)?;
            } else {
                return Err(GenericError::NewBoxed(
                    format!("The {} biome json has no property 'Blocks' inside of 'Ores'. Fix the json file!", name)));
            }
        }  else {
            return Err(GenericError::NewBoxed(
                format!("The {} biome json has no property 'Blocks'. Fix the json file!", name)));
        }


        match name {
            "Forest" => {
                generators.insert(Biome::Forest, Box::new(ForestGenerator::New(genData)));
            },
            _ => {
                return Err(GenericError::NewBoxed(
                    format!("The {} biome is not yet supported!", name)));
            }
        }

    }
    /*
        Returning a hashmap instead of a vector of biomes. Why?
        Could have used the biome enum to index into the vector, but that wouldn't
        work if I wanted to disable certain biomes.
    */
    Ok(generators)
}

fn ReadHeightModifier(json: &serde_json::Value, biomeName: &str, propertyName: &str) -> Result<HeightModifier, GenericError> {
    let mut modif = HeightModifier::default();

    if let Some(val) = json.get("Min Height") {
        modif.MinHeight = val.as_f64().unwrap() as f32;
    } else {
        return Err(GenericError::New(
            format!("The {} object has no 'Min Height' property. Error occursed in {} biome json file", propertyName, biomeName)));
    }

    if let Some(val) = json.get("Max Height") {
        modif.MaxHeight = val.as_f64().unwrap() as f32;
    } else {
        return Err(GenericError::New(
            format!("The {} object has no 'Max Height' property. Error occursed in {} biome json file", propertyName, biomeName)));
    }

    if let Some(val) = json.get("Decay") {
        modif.Decay = val.as_bool().unwrap();
    } 

    if let Some(val) = json.get("Constant") {
        modif.Constant = val.as_bool().unwrap();
        if modif.Decay && modif.Constant {
            return Err(GenericError::New(
                format!("The {} object cannot be both constant and decaying. Error occursed in {} biome json file", propertyName, biomeName)));
        }
    } 

    if let Some(val) = json.get("Speed") {
        modif.Speed = val.as_f64().unwrap() as f32;
        if modif.Constant {
            return Err(GenericError::New(
                format!("The {} object cannot be constant and have a defined speed. Error occursed in {} biome json file", propertyName, biomeName)));
        }
    } 

    Ok(modif)
}

fn ReadBlockList(json: &serde_json::Value, biomeName: &str, propertyName: &str, blockRegistry: &BlockRegistry) -> Result<Vec<(Block, HeightModifier)>, GenericError>{
    let arr = json.as_array().unwrap();
    let mut vec: Vec<(Block, HeightModifier)> = Vec::with_capacity(arr.len());

    for val in arr {
        println!("VALUES\n\n{}", val.to_string());
        if let Some(v) = val.get("Name") {
            let name = v.as_str().unwrap();
            let id = blockRegistry.NameToID(name).expect(
                format!("could not find {} in block registry", name).as_ref()
            );
            println!("ABout to write push value my little kitten :3");
            vec.push((Block {ID: id}, ReadHeightModifier(val, biomeName, propertyName)?));
        }
        else {
            return Err(GenericError::New(format!("
            Block object inside of the '{}' object must have the property 
            'Name'. Error occured in {} biome json file", propertyName, biomeName)));
        }
    }
    println!("Heres the data my little kitten :3 {:?}", vec);
    Ok(vec)
}