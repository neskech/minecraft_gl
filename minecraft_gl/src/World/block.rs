
#[derive(Clone, Copy, Debug)]
pub struct Block{
    pub ID: u8
}

impl Block{
    pub fn Air() -> Self {
        Block { ID: 0 }
    }
}
////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// /////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// /////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// /////////////////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////////////////////// 
use std::error::Error;
use std::io::{BufReader, Write};
use std::collections::{HashMap, HashSet};
use std::fs::{self, File};
use std::path::{PathBuf, Path, self};
use crate::Util::atlas::TextureAtlas;
use super::{State, GenericError};
use super::item::{Item, ItemID, ItemStack};
use super::blockBehavior::{BlockBehavior, BlockBindingFunction};
use crate::Event::event::Event;
use super::super::Util::resource;
use image;
use serde_json::Value;
use serde_json;

#[derive(Clone)]
pub enum TextureData{
    SixSided(TextureSix),
    Decoration(TextureSingle)
}

#[derive(Clone)]
pub struct TextureSix{
    //might possibly need the texture paths to debug later on
    pub Textures: [String; 6],
    /*
        Block ID's will not always line up perfectly with the position of the block's 
        texture on the atlas. This is for that
    */
    pub TextureID: u32,
    /*  
        The texture ID is the 1D coordinate of a block's texture on the texture atlas. However,
        a block may have multiple textures AFTER that position. The offsets array is to be indexed
        by the face ID (all faces numbered 0-5) of a block. This tells us how far a face's desired
        texture is away from the start of a block's texture on the atlas
    */
    pub Offsets: [u32; 6],
}

#[derive(Clone)]
pub struct TextureSingle{
    pub Texture: String,
    pub TextureID: u32
}

#[derive(Clone)]
pub struct BlockAttribute{
    pub Name: String,
    //Think of this as the 'hitpoints' of a block. This informs how hard the block is to mine
    pub Toughness: f32,
    //Think soul sand and how it makes the player go slower
    pub Friction: f32,
    //Not every block drops an item, so this is an option
    pub DropItem: Option<ItemID>,
    //Not every block has an effective tool (think a shovel for dirt) so this is an option
    pub EffectiveTool: Option<ItemID>,
    //If the user did not define a proper texture, the null texture is used
    pub TextureData: Option<TextureData>,
    //Custom attributes are for more lossly defined attributes. Not every attribute can be covered by this struct
    pub CustomAttributes: HashMap<String, State>,
    //Whether or not the block is decoration (tall grass, flowers, etc)
    pub Decor: bool,
}

impl Default for BlockAttribute{
    fn default() -> Self {
        Self { 
            Name: String::from(""),
            Toughness: 1f32,
            Friction: 1f32, 
            DropItem: None, 
            EffectiveTool: None,
            TextureData: None,
            CustomAttributes: HashMap::new(),
            Decor: false,
        }
    }
}

pub struct BlockRegistry{
    /*
        Usage of a hashmap as opposed to a vector is due to one simple case.
        Imagine if the user defined three block types, with ID's [0, 1, 15]
        These ID's would be used to index into the vector. But the obvious jump 
        in ID prevents one from doing so efficiently. For another example, consider 
        if a user added a new block with ID 15, later added 16 new blocks, and then 
        decided to remove the block with ID 15. We don't want them to manually shift
        down all block ID's > 15, so using hashmaps fits well
     */
    //TODO change these back to ves. Have them hold Option<> and fill empty spaces with None
    //TODO otherwise remove stringtoID hashmap and replace the u8 ID's with strings
    //TODO Consider changing the visibility of these too, as there are already a bunch of interface functions to access them
    pub BlocksAttributes: HashMap<u8, BlockAttribute>,
    pub BlockBehaviors: HashMap<u8, BlockBehavior>,
    StringToID: HashMap<String, u8>,
    NumRegisteredBlocks: u32,
    NumRegisteredTextures: u32,
}

impl BlockRegistry{
    pub fn New() -> Self{
        Self {
            BlocksAttributes: HashMap::new(),
            BlockBehaviors: HashMap::new(),
            StringToID: HashMap::new(),
            NumRegisteredBlocks: 0, 
            NumRegisteredTextures: 0,
        }
    }

    pub fn ReadBlockAttributes(&mut self) -> Result<(Vec<(u8, String)>, Vec<(u8, String)>), Box<dyn Error>>{
         //Keep track of the number of blocks and textures for those blocks
         let mut blockCount = 0;
         let mut textureCount = 0;

         /*
            In the JSON of blocks, the user can define an ITEM that the block can drop and an ITEM
            that is best at mining this block. I want the user to imply write down the name of their
            desired item instead of hunting down that item's ID. Now, in order to check if the user 
            wrote down a valid item, we need to check that item against the item registry. This function
            is intended to be used in the 'ReadAttributes' function in mod.rs, which will validate
            the item names given by the JSON files
          */
         
         //We will return these two vectors and check them against the item registry. If each string is valid,
         //then we will use the u8 ID and attach the appropiate item to the appropiate block ID
         let mut dropItems: Vec<(u8, String)> = Vec::new();
         let mut effectiveMiningItems: Vec<(u8, String)> = Vec::new();

         //keep a list of all json files in the given directory
         let mut jsonFiles: Vec<serde_json::Value> = Vec::new();

         let path = std::path::Path::new("./minecraft_gl/assets/data/block/json/");
         for file in fs::read_dir(path)
            .map_err(|e| format!("Error! Could not find ./minecraft_gl/assets/data/block/json/ directory! The error:\n{}", e.to_string()))? {

            let path = file
            .map_err(|e| format!("Error! Could not retrieve file in ../../assets/data/block/json/ directory! The error:\n{}", e.to_string()))?.path();

            let file = File::open(path)
            .map_err(|e| GenericError::NewBoxed(format!("Error! Could not open file of path in ../../assets/data/block/json/ directory! The error:\n{}", e.to_string())))?;

            let json: Value = serde_json::from_reader(BufReader::new(file))?;
            jsonFiles.push(json);
         }

         /*
            Sort the Json files by their ID. This is due to the running 'textureCount' variable, which is used to determine a blocks
            position in the texture atlas. Block's with lower ID's should be first in the atlas, followed by higher ID's
         */
         jsonFiles.sort_by(|a, b| a["ID"].as_u64().unwrap().partial_cmp(&b["ID"].as_u64().unwrap()).unwrap());

         //reserve block ID #0 to air
         self.StringToID.insert("Air".to_owned(), 0);
         self.BlocksAttributes.insert(0, BlockAttribute::default());

         //Iterate through each Json, extracting its attributes
         for json in jsonFiles {
            let id: u8;
            if let Some(val) = json.get("ID") {
                id = val.as_u64().unwrap() as u8;
                if id == 0 {
                    return Err(GenericError::NewBoxed(format!("ID #0 is reserved for the air block. Start your block ID's at #1. The Json:\n\n{}", json.to_string())));
                }
                else if self.BlocksAttributes.contains_key(&id) {
                    return Err(GenericError::NewBoxed(format!("Duplicate block ID found! Conflict with block '{}' of id {}. The Json:\n\n{}", self.BlocksAttributes[&id].Name, id, json.to_string())));
                }
            }
            else {
                return Err(GenericError::NewBoxed(format!("No 'ID' attribute found for block whilst reading block attributes. The Json:\n\n{}", json.to_string())));
            }
          
            let name: &str;
            if let Some(val) = json.get("Name") {
                name = val.as_str().unwrap();
                if name == "Air" {
                    return Err(GenericError::NewBoxed(format!("The block 'Air' is reserved. Rename block ID #{} to something else. The Json:\n\n{}", id, json.to_string())));
                }
                else if self.StringToID.contains_key(name) {
                    return Err(GenericError::NewBoxed(format!("Duplicate block name found! Conflict with block '{}' of id {}. The Json:\n\n{}", self.BlocksAttributes[&id].Name, id, json.to_string())));
                }

            }
            else {
                return Err(GenericError::NewBoxed(format!("No 'Name' attribute found for block of id {} whilst reading block attributes. The Json:\n\n{}", id, json.to_string())));
            }

            if let Some(val) = json.get("Enabled") {
                if ! val.as_bool().unwrap() {
                    continue;
                }
            }
            else {
                return Err(GenericError::NewBoxed(format!("No 'Enabled' attribute found for block of type {} and id {} whilst reading block attributes. The Json:\n{}", name, id, json.to_string())));
            }

            //add the name and id to the String -> BlockID hashmap
            self.StringToID.insert(String::from(name), id);

            //construct the block attribute struct with default values
            let mut blockAttribs = BlockAttribute::default();
            blockAttribs.Name = String::from(name); //add the name

            /*
                A 'Decor' block is a decoration block such as tall grass or a flower
                These blocks are limited and won't have the same attributes as other more extensive blocks
            */
            if let Some(val) = json.get("Decor") { //TODO just pass this through the rest, don't continue
                if val.as_bool().unwrap() {
                    blockAttribs.Decor = true;
                    //get the texture
                    if let Some(tex) = json.get("Texture") {
                        blockAttribs.TextureData = Some(TextureData::Decoration(TextureSingle {
                            Texture: tex.as_str().unwrap().to_owned(),
                            TextureID: textureCount
                        }));
                    } else {
                        return Err(GenericError::NewBoxed(format!("Error in block registry creation! Texture attribute for decoration block {} of id {} does not exist.", name, id)));
                    }

                    self.BlocksAttributes.insert(id, blockAttribs);
                    self.BlockBehaviors.insert(id, BlockBehavior::default());
                    textureCount += 1;
                    blockCount += 1;

                    continue;
                }
            }

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
                        let val = State::StateType(pair.1.as_str().unwrap()) 
                        .map_err(|e| format!("Error reading custom attribute '{}' for block of type {} and ID {}. The error:\n{}", pair.1.to_string(), name, id, e.to_string()))?;
                        blockAttribs.CustomAttributes.insert(pair.0.clone(), val);
                    }
                    //For containers of specific row and column dimensions
                    else if pair.1.is_object() {
                        let rows = pair.1.get("Rows").unwrap().as_u64().unwrap() as u32;
                        let cols = pair.1.get("Cols").unwrap().as_u64().unwrap() as u32;
                        let vec: Vec<ItemStack> = Vec::with_capacity((rows * cols) as usize);
                        blockAttribs.CustomAttributes.insert(pair.0.clone(), State::Container((vec, rows, cols)));
                    }
                    //integer types with a default value
                    else if pair.1.is_i64() {
                        let val = pair.1.as_i64().unwrap() as i32;
                        blockAttribs.CustomAttributes.insert(pair.0.clone(), State::IntAttribute(val));
                    }
                    //floating point types with a default value
                    else if pair.1.is_f64() {
                        let val = pair.1.as_f64().unwrap() as f32;
                        blockAttribs.CustomAttributes.insert(pair.0.clone(), State::FloatAttribute(val));
                    }
                    //boolean types with a default value
                    else if pair.1.is_boolean() {
                        let val = pair.1.as_bool().unwrap();
                        blockAttribs.CustomAttributes.insert(pair.0.clone(), State::BoolAttribute(val));
                    }
                    else {
                        return Err(GenericError::NewBoxed(format!("Error! Item attribute {} for Item {} is not a valid type!\n
                        The only valid types are...\n
                        StateType:\n
                            Dynamic Container\n
                            Int\n
                            Float\n
                            Bool\n
                        List: {{Rows, Cols}}\n
                        Integer: int\n
                        Float: float\n
                        Bool: boolean\n", pair.0, name)));
                    }  
                }
            }

            //Now start to retrieve the concrete attributes that every block must have...
            if let Some(val) = json.get("Toughness") { blockAttribs.Toughness = val.as_f64().unwrap() as f32; }
            if let Some(val) = json.get("Friction") { blockAttribs.Friction = val.as_f64().unwrap() as f32; }
            //TODO figure out a way to fix this
            //TODO implement a function in mod.rs that checks if the dropItems of the block registry are valid, as well as the place items of the item registry
            if let Some(val) = json.get("DropItem") {  dropItems.push((id, val.as_str().unwrap().to_owned())); }
            if let Some(val) = json.get("EffectiveTool") { effectiveMiningItems.push((id, val.as_str().unwrap().to_owned())) }

            if let Some(textures) = json.get("Textures") {
                 //Must have an array of 6 textures, one for each block face
                 if ! textures.is_array() {
                    return Err(GenericError::NewBoxed(format!("Error in block registry creation! Texture attribute for {} of id {} must be an array of 6 elements", name, id)));
                 }

                 //Construct an empty texData object to be written into
                 let paths = textures.as_array().unwrap();  
                 let texs = [String::new(), String::new(), String::new(), String::new(), String::new(), String::new()];
                 let mut texData = TextureSix { Textures: texs, TextureID: textureCount, Offsets: [0; 6] };

                 /*
                    This pioece of code determines the offset for each texture
                    What we do is we query if a texture has already been seen before,
                    which with the use of a hashmap is O(1). The we get the offset of that
                    particular texture, which is the value of the hashmap.
                 */
                 let mut map: HashMap<&str, u32> = HashMap::new(); //&str to avoid heap allocation
                 let mut cumul = 0;
                 for i in 0..6 {
                    texData.Textures[i] = String::from(paths[i].as_str().unwrap());

                    if ! map.contains_key(&texData.Textures[i].as_str()) {
                        /*
                            on the first iteration i will not be incremented, as the first
                            first texture location should be = textureID which means offsets[0] 
                            should = 0
                        */
                        cumul += 1 * (i != 0) as u32;
                        map.insert(paths[i].as_str().unwrap(), cumul);
                    }
                    //What is the offset of this texture?
                    texData.Offsets[i] = map[&texData.Textures[i].as_str()];
                 }
                 textureCount += cumul + 1;
                 blockAttribs.TextureData = Some(TextureData::SixSided(texData));
            }
            else {
                //for the null texture, leave texture data field as None()
                textureCount += 1; 
            }

            //add the attribute to the attributes hashmap, keyed by the block ID
            self.BlocksAttributes.insert(id, blockAttribs);
            self.BlockBehaviors.insert(id, BlockBehavior::default());
            blockCount += 1;
         }
         
         //finish up the loop
         self.NumRegisteredBlocks = blockCount;
         self.NumRegisteredTextures = textureCount;

         Ok((dropItems, effectiveMiningItems))
    }

    fn ValidatePreviousAtlas(&self, dims: u32, textureResolution: u32) -> Result<bool, String> {
        //If a texture atlas already exsits...
        if Path::new("./minecraft_gl/assets/data/block/atlas/atlas.png").exists() {
            
            //The metaData file contains data about the atlas. We want to check if that data is the same as our atlas
            let file = std::fs::File::open("./minecraft_gl/assets/data/block/atlas/metaData.json")
            .map_err(|_| format!("Could not open metaData file of path ./minecraft_gl/assets/data/block/atlas/metaData.json for the block atlas."))?;
            
            let json: Value = serde_json::from_reader(BufReader::new(file))
            .map_err(|_| format!("Could not read json meta data file for the block atlas!"))?;

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

            //check if the items in the atlas are different than our current collection of items
            let items = json.get("Items").unwrap().as_array().unwrap();
            let strItems: Vec<&str> = items.into_iter().map(|x| x.as_str().unwrap()).collect();
            for string in self.StringToID.keys() {
                
                let mut found = false;
                for string2 in &strItems {
                    if string2 == string {
                        found = true;
                        break;
                    }
                }
                
                if ! found {
                    return Ok(false);
                }

            }
            return Ok(true);
        }
        Ok(false)
    }

    pub fn GenerateAtlas(&self, textureResolution: u32, display: &glium::Display) -> Result<TextureAtlas, String> {
        //TODO Create a loading bar when creating a new texture atlas
        //TODO Make a loading bar thing in resource which takes a percentage and prints a bar for you and some metadata
        //attemp to make a square image out of the atlas...
        let dims = f32::ceil(f32::sqrt(self.NumRegisteredTextures as f32)) as u32;

        //First check if the atlas already exists...
        if self.ValidatePreviousAtlas(dims, textureResolution)? {
            let res = resource::GetImageFromPath("./minecraft_gl/assets/data/block/atlas/atlas.png")
            .map_err(|e| format!("Could not open the pre-existing block atlas in ./minecraft_gl/assets/block/atlas/atlas.png! The error:\n{}", e.to_string()))?;
            
            //open the json again, would be sloppy to return the rows and cols from validatePreviousAtlas()
            let file = std::fs::File::open("./minecraft_gl/assets/data/block/atlas/metaData.json")
            .map_err(|_| format!("Could not open metaData file of path {} for the block atlas.", "./minecraft_gl/assets/data/block/atlas/metaData.json"))?;
            
            let json: Value = serde_json::from_reader(BufReader::new(file))
            .map_err(|_| "Could not open block atlas metadata json!")?;
            let rows = json.get("Rows").unwrap().as_u64().unwrap() as u32;
            let cols = json.get("Cols").unwrap().as_u64().unwrap() as u32;

            return Ok(TextureAtlas::FromImage(res, rows, cols, textureResolution, display))
        }

        //the master texture atlas image. We will paste texture - sub images onto this
        let mut img = image::RgbaImage::new(textureResolution * dims, textureResolution * dims);

        let mut runningTextureCount = 0;
        let mut keys: Vec<&u8> = self.BlocksAttributes.keys().collect();
        keys.sort();
        for id in keys {
            if *id == 0 {
                //skip air, it will have no texture
                continue;
            }

            //If there is texture data...
            if let Some(TextureData::SixSided(texData)) = &self.BlocksAttributes[id].TextureData {

                let mut set: HashSet<&str> = HashSet::new(); //&str to prevent heap allocation
                for i in 0..6 {
                    //Only add a new texture if we haven't encountered it before
                    if ! set.contains(&texData.Textures[i].as_str()) {
                        set.insert(texData.Textures[i].as_str());
                 
                        let mut pathBuf = PathBuf::new();
                        pathBuf.push("./minecraft_gl/assets/data/block/img");
                        pathBuf.push(texData.Textures[i].as_str());
                   
                        //If the path is invalid, use the null texture and print an error response
                        let mut texture = match resource::GetImageOrNull(pathBuf.as_path().as_os_str().to_str().unwrap()) {
                            Ok(tex) => tex,
                            Err(tex) => {
                                eprintln!("Image of path {} is invalid! Using null texture for block {} of id {}", pathBuf.as_os_str().to_str().unwrap(), self.BlocksAttributes[id].Name, id);
                                tex
                            }
                        };

                        //resize the image
                        texture = image::DynamicImage::ImageRgba8(image::imageops::resize(&texture, textureResolution, textureResolution, image::imageops::FilterType::Nearest));
                        let coords = ((runningTextureCount % dims) * textureResolution, (runningTextureCount / dims) * textureResolution);
                        image::imageops::overlay(&mut img, &mut texture, coords.0, coords.1);
                        runningTextureCount += 1;
                    }
                }
            }
            else if let Some(TextureData::Decoration(texData)) = &self.BlocksAttributes[id].TextureData {
                println!("HERE!!!!");
                let mut pathBuf = PathBuf::new();
                pathBuf.push("./minecraft_gl/assets/data/block/img/");
                pathBuf.push(texData.Texture.as_str());

                let mut texture = resource::GetImageFromPath(pathBuf.as_os_str().to_str().unwrap())?;
                texture = image::DynamicImage::ImageRgba8(image::imageops::resize(&mut texture, textureResolution, textureResolution, image::imageops::FilterType::Nearest));
                let coords = ((runningTextureCount % dims) * textureResolution, (runningTextureCount / dims) * textureResolution);
                image::imageops::overlay(&mut img, &texture, coords.0, coords.1);
                runningTextureCount += 1;
            }
            else {
                //Else add the null texture...
                let mut texture = resource::GetImageFromPath("./minecraft_gl/assets/data/block/img/nullTexture.png")?;
                texture = image::DynamicImage::ImageRgba8(image::imageops::resize(&mut texture, textureResolution, textureResolution, image::imageops::FilterType::Nearest));
                let coords = ((runningTextureCount % dims) * textureResolution, (runningTextureCount / dims) * textureResolution);
                image::imageops::overlay(&mut img, &texture, coords.0, coords.1);
                runningTextureCount += 1;
            }
 
        }
        
        let image = image::DynamicImage::ImageRgba8(img);
        
        //Save the image and some metadata about it
        image.save("./minecraft_gl/assets/data/block/atlas/atlas.png").expect("Could not save block atlas png!");
        let mut file = File::create("./minecraft_gl/assets/data/block/atlas/metadata.json").expect("Could not create block atlas metadata file!");

        //generate a list of all the block names
        let mut blocks: Vec<&str> = Vec::with_capacity(self.NumRegisteredBlocks as usize);
        for attrib in self.StringToID.keys() {
            blocks.push(&attrib);
        }
        //generate a serialized json string of these values and write it to the file
        let serialized = serde_json::to_string(&blocks).expect("Could not serialize block names to json fromat!");
        let finalStr = format!("{{\n\"Items\": {},\n\"Rows\": {},\n\"Cols\": {},\n\"Texture Resolution\": {}\n}}", serialized, dims, dims, textureResolution);
        file.write_all(finalStr.as_bytes()).expect("Could not write to block atlas metadata file!");

        Ok(TextureAtlas::FromImage(image, dims, dims, textureResolution, display))
    }

    pub fn OnLeftClickWithName(&self, blockName: &str, hit: Item) {
        let blockID = self.StringToID[blockName];
        let behavior =  &self.BlockBehaviors[&blockID];
        (behavior.OnLeftClick)(self.GetAttributesOfID(blockID), hit)
    }

    pub fn OnLeftClickWithID(&self, blockID: u8, hit: Item) {
        let behavior =  &self.BlockBehaviors[&blockID];
        (behavior.OnLeftClick)(self.GetAttributesOfID(blockID), hit)

    }

    pub fn OnRightClickWithName(&self, blockName: &str) -> Option<fn(&BlockAttribute, Event) -> bool> {
        let blockID = self.StringToID[blockName];
        let behavior =  &self.BlockBehaviors[&blockID];
        (behavior.OnRightClick)(self.GetAttributesOfID(blockID))
        

    }

    pub fn OnRightClickWithID(&self, blockID: u8) -> Option<fn(&BlockAttribute, Event) -> bool> {
        let behavior =  &self.BlockBehaviors[&blockID];
        (behavior.OnRightClick)(self.GetAttributesOfID(blockID))
    }

    pub fn GetAttributesOf(&self, block: &Block) -> &BlockAttribute{
        &self.BlocksAttributes[&block.ID]
    }

    pub fn GetAttributesOfID(&self, id: u8) -> &BlockAttribute{
        &self.BlocksAttributes[&id]
    }

    pub fn GetNameOfID<'a>(&'a self, id: u8) -> Result<&'a String, String>{
        if ! self.BlocksAttributes.contains_key(&id) {
            return Err(format!("Block id of {} not found in block registry. Be sure that it exists or is actually enabled", id));
        }
        Ok(&self.BlocksAttributes[&id].Name)
    }

    pub fn NameToID(&self, blockName: &str) -> u8{
        //TODO Convert the whole string to lowercase, then the first charactet to uppercase
        self.StringToID[blockName]
    }

    pub fn HasBlock(&self, blockName: &str) -> bool{
        self.StringToID.contains_key(blockName)
    }

    pub fn InitBehaviors(&mut self){
        BlockBindingFunction(self);
    }
}