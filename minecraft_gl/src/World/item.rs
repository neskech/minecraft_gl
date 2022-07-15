

const INVALID_ITEM_ID: u8 = u8::MAX;

//TODO ENUM for ID? Either u8 or INVALID similiar to an option
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Item{
    pub ID: u8,
}

impl Item{
    pub fn IsValid(&self) -> bool {
        self.ID != INVALID_ITEM_ID
    }

    pub fn Null() -> Item {
        Item { ID: INVALID_ITEM_ID }
    }
}

const MAX_ITEM_STACK: u8 = 64;
pub struct ItemStack{
    pub Item: Item,
    pub count: u8,
}


use std::{collections::HashMap, fs::File};

use glfw::WindowEvent;
use serde_json::Value;

////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// /////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// /////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// /////////////////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////////////////////// 
use std::error::Error;
use std::io::BufReader;
use std::fs::{self};
use crate::OpenGL::texture::Texture;
use crate::Util::atlas::TextureAtlas;
use crate::Util::resource;
use super::{GenericError, State};
use super::crafting::{CraftingRecipe, CraftingRegistry};
use super::itemBehavior::ItemBehavior;
use super::block::Block;



#[derive(Clone)]
pub struct ItemAttribute{
    //To be cloned upon creation of a new item of this type. Serves as a template
    pub Attributes: HashMap<String, State>,
    pub PlaceableBlock: Option<Block>,
    pub Texture: Option<String>,
}

impl Default for ItemAttribute{
    fn default() -> Self {
        Self { 
            Attributes: HashMap::new(),
            PlaceableBlock: None,
            Texture: None 
        }
    }
}

pub struct ItemRegistry{
    ItemAttributes: Vec<ItemAttribute>,
    ItemBehaviors: Vec<Option<ItemBehavior>>,
    StringToID: HashMap<String, u8>,
    NumRegisteredItems: u32,
    NumRegisteredTextures: u32,

}

impl ItemRegistry{
    pub fn New(craftingRegistry: &mut CraftingRegistry) -> Result<Self, Box<dyn Error>> {
        let mut s = Self {
            ItemAttributes: Vec::new(),
            ItemBehaviors: Vec::new(),
            StringToID: HashMap::new(),
            NumRegisteredItems: 0,
            NumRegisteredTextures: 0,
        };

        if let Err(err) = s.ReadItemAttributes(craftingRegistry) {
            return Err(err);
        }

        Ok(s)
    }

    pub fn ReadItemAttributes(&mut self, craftingRegistry: &mut CraftingRegistry) -> Result<(), Box<dyn Error>> {
         let mut itemCount = 0;
         let mut textureCount = 0;

         let path = std::path::Path::new("./minecraft_gl/assets/data/block/json/");
         for file in fs::read_dir(path).map_err(|e| format!("Error! Could not find ./minecraft_gl/assets/data/block/json/! The error:\n{}", e.to_string()))? {
            let path = file.map_err(|e| GenericError::NewBoxed(format!("Error! Could not retrieve file in ../../assets/data/block/json/ directory! The error:\n{}", e.to_string())))?.path();
            let file = File::open(path).map_err(|e| GenericError::NewBoxed(format!("Error! Could not open file of path in ../../assets/data/block/json/ directory! The error:\n{}", e.to_string())))?;
            let buff = BufReader::new(file);
            let json: Value = serde_json::from_reader(buff)?;

            let id = json["ID"].as_u64().unwrap() as u8;
            let name = json["Name"].as_str().unwrap();
            self.StringToID.insert(String::from(name), id);

            let mut attrib = ItemAttribute::default();
            
            //get the attributes...
            if let Some(attributes) = json.get("Attributes") {
                let obj = attributes.as_object().unwrap();

                for pair in obj.iter() {
                    if pair.1.is_string() {
                        let val = match State::StateType(pair.1.as_str().unwrap()) {
                            Ok(val) => val,
                            Err(msg) => return Err(GenericError::NewBoxed(format!("{}. Error orignated from Item type {} of Id {}", msg, name, id)))
                        };
                        attrib.Attributes.insert(pair.0.clone(), val);
                    }
                    else if pair.1.is_object() {
                        let rows = pair.1["Rows"].as_u64().unwrap() as u32;
                        let cols = pair.1["Cols"].as_u64().unwrap() as u32;
                        let mut vec: Vec<Item> = Vec::new();
                        vec.reserve((rows * cols) as usize);
                        attrib.Attributes.insert(pair.0.clone(), State::Container((vec, rows, cols)));
                    }
                    else if pair.1.is_i64() {
                        let val = pair.1.as_i64().unwrap() as i32;
                        attrib.Attributes.insert(pair.0.clone(), State::IntAttribute(val));
                    }
                    else if pair.1.is_f64() {
                        let val = pair.1.as_f64().unwrap() as f32;
                        attrib.Attributes.insert(pair.0.clone(), State::FloatAttribute(val));
                    }
                    else if pair.1.is_boolean() {
                        let val = pair.1.as_bool().unwrap();
                        attrib.Attributes.insert(pair.0.clone(), State::BoolAttribute(val));
                    }
                    else {
                        return Err(GenericError::NewBoxed(format!("Error! Item attribute {} for Item {} is not a valid type!\n
                        The only valid types are...\n
                        List: {{Rows, Cols}}\n
                        Integer: int\n
                        Float: float\n
                        Bool: boolean\n", pair.0, name)));
                    }  
                }
            }

            //TODO ITEM registry needs block reg and block reg needs item, fix this!!!!!!!
            if let Some(block) = json.get("PlaceableBlock") {
                let block = Block { ID: block.as_u64().unwrap() as u8 };
                attrib.PlaceableBlock = Some(block);
            }
   
            if let Some(recipe) = json.get("CraftingRecipe") {
                let array = recipe.as_array().unwrap();

                //check if the crafting grid is a square...
                let sqrt = f32::sqrt(array.len() as f32);
                if sqrt != f32::floor(sqrt) {
                    return Err(GenericError::NewBoxed(format!("Error! Crafting grid for item {} is not a square!", name)));
                }

                craftingRegistry.AddRecipe(
                        id, 
                        match CraftingRecipe::New(
                                array.into_iter().map(|x| x.as_u64().unwrap() as u8).collect(),
                                sqrt as u32,
                                sqrt as u32
                            ) 
                            {
                                Ok(val) => val,
                                Err(msg) => return Err(GenericError::NewBoxed(msg))
                            }
               );
                   
            }

            if let Some(texture) = json.get("Texture") {
                attrib.Texture = Some(String::from(texture.as_str().unwrap()));
            }
            textureCount += 1;

            //add the attribute to the attributes array
            if id as usize >= self.ItemAttributes.len() {
                self.ItemAttributes.resize((id + 1) as usize, ItemAttribute::default());
            }
            self.ItemAttributes[id as usize] = attrib;

            itemCount += 1;
         }
         
         self.NumRegisteredItems = itemCount;
         self.NumRegisteredTextures = textureCount;

         Ok(())
    }

    pub fn GenerateAtlas(&self, texture: Texture, textureResolution: u32) -> Result<TextureAtlas, String> {
         //attemp to make a square image out of the atlas...
         let dims = f32::ceil(f32::sqrt(self.NumRegisteredItems as f32)) as u32;
         let mut img = image::RgbaImage::new(textureResolution * dims, textureResolution * dims);
 
         for idx in 0..self.NumRegisteredItems {
             
             let path = if let Some(tex) = &self.ItemAttributes[idx as usize].Texture {
                        tex.clone()
                    }
                    else {
                        String::from("../../assets/data/item/img/nullTexture.png")
                    };

             
            let mut texture = resource::GetImageFromPath(&path)?;
            image::imageops::resize(&mut texture, textureResolution, textureResolution, image::imageops::FilterType::Nearest);
            let coords = ((idx % dims) * textureResolution, (idx / dims) * textureResolution);
            image::imageops::overlay(&mut img, &mut texture, coords.0, coords.1);
  
         }
         
         Ok(TextureAtlas::FromImage(image::DynamicImage::ImageRgba8(img), texture, dims, dims, textureResolution))
    }

    pub fn OnLeftClick(&self, blockName: &str) {
        let itemID = self.StringToID[blockName];
        if let Some(behavior) = &self.ItemBehaviors[itemID as usize] {
            (behavior.OnLeftClick)(self.GetAttributesOfID(itemID));
        }
        let default = ItemBehavior::default();
        (default.OnLeftClick)(self.GetAttributesOfID(itemID));
    }

    pub fn OnRightClick(&self, blockName: &str) -> Option<fn(attributes: &ItemAttribute, WindowEvent)> {
        let itemID = self.StringToID[blockName];
        if let Some(behavior) = &self.ItemBehaviors[itemID as usize] {
            return (behavior.OnRightClick)(self.GetAttributesOfID(itemID));
        }
        let default = ItemBehavior::default();
        (default.OnRightClick)(self.GetAttributesOfID(itemID))
    }

    pub fn GetAttributesOf(&self, item: &Item) -> &ItemAttribute{
        &self.ItemAttributes[item.ID as usize]
    }

    pub fn GetAttributesOfID(&self, id: u8) -> &ItemAttribute{
        &self.ItemAttributes[id as usize]
    }

    pub fn IDofItem(&self, itemName: &str) -> u8{
        //TODO Return Result<u8, &str> saying in the error that ID for {itemName} doesn't exist
        self.StringToID[itemName]
    }
}