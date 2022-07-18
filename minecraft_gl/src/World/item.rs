


//TODO ENUM for ID? Either u8 or INVALID similiar to an option
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ItemID{
    pub ID: u8,
}

impl ItemID{
    pub fn New(id: u8) -> Self{
        Self { ID: id }
    }
}
#[derive(Clone)]
pub struct Item{
    pub ItemID: ItemID,
    pub Attributes: Option<HashMap<String, super::State>>
}

impl Item{
    pub fn FromID(itemID: u8, itemRegistry: &ItemRegistry) -> Self{
        Self {
            ItemID: ItemID::New(itemID),
            Attributes: Some(itemRegistry.GetAttributesOfID(itemID).CustomAttributes.clone())
        }
    }

    pub fn FromName(itemName: &str, itemRegistry: &ItemRegistry) -> Self{
        let id = itemRegistry.NameToID(itemName);
        Self {
            ItemID: ItemID::New(id),
            Attributes: Some(itemRegistry.GetAttributesOfID(id).CustomAttributes.clone())
        }
    }
}
#[derive(Clone)]
pub struct ItemStack{
    pub Item: Item,
    pub Count: u32,
}

impl ItemStack {
    pub fn New(item: Item) -> Self{
        Self {
            Item: item,
            Count: 1
        }
    }

    pub fn Push(&mut self, item: &Item, itemRegistry: &ItemRegistry) -> bool{
        if item.ItemID == self.Item.ItemID && self.Count <= itemRegistry.GetAttributesOfID(self.Item.ItemID.ID).StackSize{
            self.Count += 1;
            return true;
        }
        false
            
    }

    pub fn Pop(&mut self) -> bool{
        if self.Count <= 0{
            return false;
        }
        self.Count -= 1;
        true
    }
}



////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// /////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// /////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// /////////////////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////////////////////// 
use std::path::Path;
use std::{collections::HashMap, fs::File};
use serde_json::Value;
use std::error::Error;
use std::io::{BufReader, Write};
use std::fs;
use crate::Event::event::Event;
use crate::Util::atlas::TextureAtlas;
use crate::Util::resource;
use super::{GenericError, State};
use super::crafting::{CraftingRecipe, CraftingRegistry};
use super::itemBehavior::ItemBehavior;
use super::block::Block;



#[derive(Clone)]
pub struct ItemAttribute{
    pub Name: String,
    //Custom attributes are for more lossly defined attributes. Not every attribute can be covered by this struct
    pub CustomAttributes: HashMap<String, State>,
    //Amount of items allowed in one stack,
    pub StackSize: u32,
    //A block that can be placed upon the player right clicking with this item
    pub PlaceableBlock: Option<Block>,
    //The item's texture. If NONE, then the null texture will be used
    pub Texture: Option<String>,
}

//Default implementation
impl Default for ItemAttribute{
    fn default() -> Self {
        Self { 
            Name: String::from(""),
            StackSize: 64,
            CustomAttributes: HashMap::new(),
            PlaceableBlock: None,
            Texture: None 
        }
    }
}

pub struct ItemRegistry{
        /*
        Usage of a hashmap as opposed to a vector is due to one simple case.
        Imagine if the user defined three item types, with ID's [0, 1, 15]
        These ID's would be used to index into the vector. But the obvious jump 
        in ID prevents one from doing so efficiently. For another example, consider 
        if a user added a new item with ID 15, later added 16 new items, and then 
        decided to remove the item with ID 15. We don't want them to manually shift
        down all item ID's > 15, so using hashmaps fits well
     */
    //TODO change these back to ves. Have them hold Option<> and fill empty spaces with None
    //TODO otherwise remove stringtoID hashmap and replace the u8 ID's with strings
    //TODO Consider changing the visibility of these too, as there are already a bunch of interface functions to access them
    pub ItemAttributes: HashMap<u8, ItemAttribute>,
    pub ItemBehaviors: HashMap<u8, ItemBehavior>,
    StringToID: HashMap<String, u8>,
    NumRegisteredItems: u32,
    NumRegisteredTextures: u32,

}

impl ItemRegistry{
    pub fn New() -> Self {
        Self {
            ItemAttributes: HashMap::new(),
            ItemBehaviors: HashMap::new(),
            StringToID: HashMap::new(),
            NumRegisteredItems: 0,
            NumRegisteredTextures: 0,
        }
    }

    pub fn ReadItemAttributes(&mut self, craftingRegistry: &mut CraftingRegistry) -> Result<Vec<(u8, String)>, Box<dyn Error>> {
         //Keep track of the number of blocks and textures for those blocks
         let mut itemCount = 0;
         let mut textureCount = 0;

          /*
            In the JSON of items, the user can define a BLOCK that the item can place. I want the user
            to imply write down the name of their desired block instead of hunting down that block's ID.
            Now, in order to check if the user wrote down a valid block, we need to check that block against
            the block registry. This function is intended to be used in the 'ReadAttributes' function in mod.rs, 
            which will validate the block names given by the JSON files
          */
         
         //We will return these two vectors and check them against the block registry. If each string is valid,
         //then we will use the u8 ID and attach the appropiate item to the appropiate item ID
         let mut placeBlocks: Vec<(u8, String)> = Vec::new();

        //keep a list of all json files in the given directory
        let mut jsonFiles: Vec<serde_json::Value> = Vec::new();

        let path = std::path::Path::new("./minecraft_gl/assets/data/item/json/");
        for file in fs::read_dir(path)
           .map_err(|e| format!("Error! Could not find ./minecraft_gl/assets/data/item/json/ directory! The error:\n{}", e.to_string()))? {

           let path = file
           .map_err(|e| format!("Error! Could not retrieve file in ../../assets/data/item/json/ directory! The error:\n{}", e.to_string()))?.path();

           let file = File::open(path)
           .map_err(|e| GenericError::NewBoxed(format!("Error! Could not open file of path in ../../assets/data/item/json/ directory! The error:\n{}", e.to_string())))?;

           let json: Value = serde_json::from_reader(BufReader::new(file))?;
           jsonFiles.push(json);
        }

         /*
            Sort the Json files by their ID. This is due to the running 'textureCount' variable, which is used to determine a blocks
            position in the texture atlas. Block's with lower ID's should be first in the atlas, followed by higher ID's
         */
        jsonFiles.sort_by(|a, b| a["ID"].as_u64().unwrap().partial_cmp(&b["ID"].as_u64().unwrap()).unwrap());

        for json in jsonFiles {
            let id: u8;
            if let Some(val) = json.get("ID") {
                id = val.as_u64().unwrap() as u8;
                if self.ItemAttributes.contains_key(&id) {
                    return Err(GenericError::NewBoxed(format!("Duplicate item ID found! Conflict with block '{}' of id {}. The Json:\n\n{}", self.ItemAttributes[&id].Name, id, json.to_string())));
                }
            }
            else {
                return Err(GenericError::NewBoxed(format!("No 'ID' attribute found for item whilst reading item attributes. The Json:\n\n{}", json.to_string())));
            }
          
            let name: &str;
            if let Some(val) = json.get("Name") {
                name = val.as_str().unwrap();
                if self.StringToID.contains_key(name) {
                    return Err(GenericError::NewBoxed(format!("Duplicate item name found! Conflict with item '{}' of id {}. The Json:\n\n{}", self.ItemAttributes[&id].Name, id, json.to_string())));
                }

            }
            else {
                return Err(GenericError::NewBoxed(format!("No 'Name' attribute found for item of id {} whilst reading block attributes. The Json:\n\n{}", id, json.to_string())));
            }

            if let Some(val) = json.get("Enabled") {
                if ! val.as_bool().unwrap() {
                    continue;
                }
            }
            else {
                return Err(GenericError::NewBoxed(format!("No 'Enabled' attribute found for item of type {} and id {} whilst reading item attributes. The Json:\n{}", name, id, json.to_string())));
            }


           //add the name and id to the String -> BlockID hashmap
           self.StringToID.insert(String::from(name), id);

           //construct the block attribute struct with default values
           let mut itemAttribs = ItemAttribute::default();
           itemAttribs.Name = String::from(name); //add the name
            
            /*
                Gather the custom attributes. These are varying attributes that are two specific to put in
                as individual data members for the block attribute struct. As such, a hashmap is used to grab
                these highly specialized values. The idea is for the user to grab these attributes via their 
                name in the block behavior functions later on defined
            */
           if let Some(attributes) = json.get("Attributes") {
                let obj = attributes.as_object().unwrap();

                for pair in obj.iter() {

                     /*
                        his is for when the user wants to specify a data type, but doesn't want to provide a default value.
                        Imagine a furnace, which uses a floating point number to represent the amount of fuel left inside of it.
                        It won't make much sense for the user to give that a default value, so they can just specify a data type.
                    */
                    if pair.1.is_string() {
                        let val = match State::StateType(pair.1.as_str().unwrap()) {
                            Ok(val) => val,
                            Err(msg) => return Err(GenericError::NewBoxed(format!("{}. Error orignated from Item type {} of Id {}", msg, name, id)))
                        };
                        itemAttribs.CustomAttributes.insert(pair.0.clone(), val);
                    }
                    //For containers of specific row and column dimensions
                    else if pair.1.is_object() {
                        let rows = pair.1["Rows"].as_u64().unwrap() as u32;
                        let cols = pair.1["Cols"].as_u64().unwrap() as u32;
                        let mut vec: Vec<ItemStack> = Vec::new();
                        vec.reserve((rows * cols) as usize);
                        itemAttribs.CustomAttributes.insert(pair.0.clone(), State::Container((vec, rows, cols)));
                    }
                    //integer types with a default value
                    else if pair.1.is_i64() {
                        let val = pair.1.as_i64().unwrap() as i32;
                        itemAttribs.CustomAttributes.insert(pair.0.clone(), State::IntAttribute(val));
                    }
                    //floating point types with a default value//floating point types with a default value
                    else if pair.1.is_f64() {
                        let val = pair.1.as_f64().unwrap() as f32;
                        itemAttribs.CustomAttributes.insert(pair.0.clone(), State::FloatAttribute(val));
                    }
                    //boolean types with a default value
                    else if pair.1.is_boolean() {
                        let val = pair.1.as_bool().unwrap();
                        itemAttribs.CustomAttributes.insert(pair.0.clone(), State::BoolAttribute(val));
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

            //Now start to retrieve the concrete attributes that every block must have...
            if let Some(block) = json.get("Placeable Block") { placeBlocks.push((id, block.as_str().unwrap().to_owned())) }
            if let Some(size) = json.get("Stack Size") { 
                let stackSize = size.as_u64().unwrap() as u32;
                /*
                    Item stacks (the struct) is implemented such that it doesn't actually contain a list of items,
                    but rather a single item with a count attached to it. Because of this, items with unique data
                    attached to them (such as durability, a container of other items, or whatever other attribute)
                    cannot be represented in a stack. So if the stack size is > 1 and the item has custom attributes,
                    that is illegal
                 */
                if stackSize > 1 && itemAttribs.CustomAttributes.len() > 0 {
                    return Err(GenericError::NewBoxed(
                    format!("Cannot have a stack size greater than 1 for an item of custom attributes. 
                    Item {} of id {} with stack size {} is invalid", name, id, stackSize)));
                }
                itemAttribs.StackSize = stackSize; 
            }
            else if itemAttribs.CustomAttributes.len() > 0 {
                //the default stack size is 64, so correct this if the item has attributes and a stack size wasn't defined
                itemAttribs.StackSize = 1;
            }
            
            //Get the crafting recipe for this item if there is one
            if let Some(recipe) = json.get("Crafting Recipe") {
                let array = recipe.as_array().unwrap();

                //check if the crafting grid is a square...
                let sqrt = f32::sqrt(array.len() as f32);
                if sqrt != f32::floor(sqrt) {
                    return Err(GenericError::NewBoxed(format!("Error! Crafting grid for item {} is not a square!", name)));
                }

                //If it is, then add the recipe to the crafting registry
                craftingRegistry.AddRecipe(
                        id, 
                        CraftingRecipe::New(
                                array.into_iter().map(|x| x.as_u64().unwrap() as u8).collect(),
                                sqrt as u32,
                                sqrt as u32
                            )
                            .map_err(|e| format!("Crafting recipe creaion for item {} of id {} failed. The error:\n{}", name, id, e.to_string()))? 
               );
                   
            }

            if let Some(texture) = json.get("Texture") {
                itemAttribs.Texture = Some(String::from(texture.as_str().unwrap()));
            }
            textureCount += 1; //the given texture or null texture

            //add the attribute to the attributes map
            self.ItemAttributes.insert(id, itemAttribs);
            self.ItemBehaviors.insert(id, ItemBehavior::default());
            itemCount += 1;
         }
         
         self.NumRegisteredItems = itemCount;
         self.NumRegisteredTextures = textureCount;

         Ok(placeBlocks)
    }

    fn ValidatePreviousAtlas(&self, dims: u32, textureResolution: u32) -> Result<bool, String> {
        //If a texture atlas already exsits...
        if Path::new("./minecraft_gl/assets/data/item/atlas/atlas.png").exists() {

               //The metaData file contains data about the atlas. We want to check if that data is the same as our atlas
            let file = std::fs::File::open("./minecraft_gl/assets/data/item/atlas/metaData.json")
            .map_err(|_| format!("Could not open metaData file of path {} for the item atlas.", "./minecraft_gl/assets/data/item/atlas/metaData.json"))?;
            
            let json: Value = serde_json::from_reader(BufReader::new(file))
            .map_err(|_| format!("Could not read json meta data file for the item atlas!"))?;

            //check if the dimensions are different
            let rows = json.get("Rows").unwrap().as_u64().unwrap() as u32;
            let cols = json.get("Cols").unwrap().as_u64().unwrap() as u32;
            if rows != dims || cols != dims {
                return Ok(false);
            }

            //check if the resolution is the same
            let res = json.get("Texture Resolution").unwrap().as_u64().unwrap() as u32;
            if textureResolution != res {
                return Ok(false);
            }

            let items = json.get("Items").unwrap().as_array().unwrap();
            for item in items {
                let str = item.as_str().unwrap();
                if ! self.StringToID.contains_key(str) {
                    return Ok(false);
                }

            }
            return Ok(true);
        }
        Ok(false)
    }

    pub fn GenerateAtlas(&self, textureResolution: u32, display: &glium::Display) -> Result<TextureAtlas, String> {
         //attemp to make a square image out of the atlas...
        let dims = f32::ceil(f32::sqrt(self.NumRegisteredTextures as f32)) as u32;

        //First check if the atlas already exists...
        if self.ValidatePreviousAtlas(dims, textureResolution)? {
            let res = resource::GetImageFromPath("./minecraft_gl/assets/data/item/atlas/atlas.png")
            .map_err(|e| format!("Could not open the pre-existing item atlas in ./minecraft_gl/assets/item/atlas/atlas.png! The error:\n{}", e.to_string()))?;
            
            //open the json again, would be sloppy to return the rows and cols from validatePreviousAtlas()
            let file = std::fs::File::open("./minecraft_gl/assets/data/item/atlas/metaData.json")
            .map_err(|_| format!("Could not open metaData file of path {} for the item atlas.", "./minecraft_gl/assets/data/item/atlas/metaData.json"))?;
            
            let json: Value = serde_json::from_reader(BufReader::new(file))
            .map_err(|_| "Could not open block atlas metadata json!")?;
            let rows = json.get("Rows").unwrap().as_u64().unwrap() as u32;
            let cols = json.get("Cols").unwrap().as_u64().unwrap() as u32;

            return Ok(TextureAtlas::FromImage(res, rows, cols, textureResolution, display))
        }

        //the master texture atlas image. We will paste texture - sub images onto this
        let mut img = image::RgbaImage::new(textureResolution * dims, textureResolution * dims);
        
        let dir = String::from("./minecraft_gl/assets/data/item/img/");
        for idx in 0..self.NumRegisteredItems {
             //If a texture doesn't exist, use the null texture     
             let path = if let Some(tex) = &self.ItemAttributes[&(idx as u8)].Texture {
                         let mut d = dir.clone();
                         d.push_str(tex);
                         d
                    }
                    else {
                        String::from("./minecraft_gl/assets/data/item/img/nullTexture.png")
                    };

             //paste the texture onto the atlas
            let mut texture = resource::GetImageFromPath(&path)
            .map_err(|e| format!("Error! Could not read image texture for item '{}' of id '{}'. The error:\n{}", self.ItemAttributes[&(idx as u8)].Name, idx, e.to_string()))?;
            texture = image::DynamicImage::ImageRgba8(image::imageops::resize(&mut texture, textureResolution, textureResolution, image::imageops::FilterType::Nearest));
            let coords = ((idx % dims) * textureResolution, (idx / dims) * textureResolution);
            image::imageops::overlay(&mut img, &mut texture, coords.0, coords.1);
  
         }

        //sSave the image and some metadata about it
        img.save("./minecraft_gl/assets/data/item/atlas/atlas.png")
        .map_err(|e| format!("Could not save item atlas png! The error:\n{}", e.to_string()))?;
        let mut file = File::create("./minecraft_gl/assets/data/item/atlas/metadata.json").map_err(|_| "Could not create item atlas metadata file!")?;

        //generate a list of all the item names
        let mut items: Vec<&str> = Vec::with_capacity(self.NumRegisteredItems as usize);
        for attrib in self.StringToID.keys() {
            items.push(&attrib);
        }
         //generate a serialized json string of these values and write it to the file
        let serialized = serde_json::to_string(&items).expect("Could not serialize block names to json fromat!");
        let finalStr = format!("{{\n\"Items\": {},\n\"Rows\": {},\n\"Cols\": {},\n\"Texture Resolution\": {}\n}}", serialized, dims, dims, textureResolution);
        file.write_all(finalStr.as_bytes()).expect("Could not write to item atlas metadata file!");
         
         Ok(TextureAtlas::FromImage(image::DynamicImage::ImageRgba8(img), dims, dims, textureResolution, display))
    }

    pub fn OnLeftClickWithName(&self, itemName: &str) {
        let itemID = self.StringToID[itemName];
        let behavior =  &self.ItemBehaviors[&itemID];
        (behavior.OnLeftClick)(self.GetAttributesOfID(itemID))
    }

    pub fn OnLeftClickWithID(&self, itemID: u8) {
        let behavior =  &self.ItemBehaviors[&itemID];
        (behavior.OnLeftClick)(self.GetAttributesOfID(itemID))
    }

    pub fn OnRightClickWithName(&self, itemName: &str) -> Option<fn(attributes: &ItemAttribute, Event)> {
        let itemID = self.StringToID[itemName];
        let behavior = &self.ItemBehaviors[&itemID];
        (behavior.OnRightClick)(self.GetAttributesOfID(itemID))
    }

    pub fn OnRightClickWithID(&self, itemID: u8) -> Option<fn(attributes: &ItemAttribute, Event)> {
        let behavior = &self.ItemBehaviors[&itemID];
        (behavior.OnRightClick)(self.GetAttributesOfID(itemID))
    }

    pub fn GetAttributesOf(&self, itemID: u8) -> &ItemAttribute{
        &self.ItemAttributes[&itemID]
    }

    pub fn GetAttributesOfID(&self, id: u8) -> &ItemAttribute{
        &self.ItemAttributes[&id]
    }

    pub fn NameToID(&self, itemName: &str) -> u8{
        //TODO Return Result<u8, &str> saying in the error that ID for {itemName} doesn't exist
        self.StringToID[itemName]
    }

    pub fn HasItem(&self, itemName: &str) -> bool{
        self.StringToID.contains_key(itemName)
    }

    pub fn InitBehaviors(&mut self){
        
    }
}