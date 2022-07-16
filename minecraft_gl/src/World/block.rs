
#[derive(Clone)]
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
use std::io::{BufReader, BufRead, Write};
use std::collections::{HashMap, HashSet};
use std::fs::{self, File};
use std::path::{PathBuf, Path};
use crate::Util::atlas::TextureAtlas;
use super::{State, GenericError, item};
use super::item::{Item, ItemRegistry};
use super::blockBehavior::BlockBehavior;
use glfw::WindowEvent;
use super::super::Util::resource;
use image::{self, GenericImageView};
use serde_json::Value;
use serde_json;

#[derive(Clone)]
pub struct TextureData{
    pub Textures: [String; 6],
    pub TextureID: u32,
    pub Offsets: [u32; 6],
}

#[derive(Clone)]
pub struct BlockAttribute{
    pub Toughness: f32,
    pub Friction: f32,
    pub DropItem: Option<Item>,
    pub EffectiveTool: Option<Item>,
    pub TextureData: Option<TextureData>,
    //TODO rename the Item Attribute's name for this to 'custom' too
    pub CustomAttributes: HashMap<String, State>,
}

impl Default for BlockAttribute{
    fn default() -> Self {
        Self { 
            Toughness: 1f32,
            Friction: 1f32, 
            DropItem: None, 
            EffectiveTool: None,
            TextureData: None,
            CustomAttributes: HashMap::new()
        }
    }
}

pub struct BlockRegistry{
    pub BlocksAttributes: Vec<BlockAttribute>,
    pub BlockBehaviors: Vec<Option<BlockBehavior>>,
    StringToID: HashMap<String, u8>,
    NumRegisteredBlocks: u32,
    NumRegisteredTextures: u32,
}

impl BlockRegistry{
    pub fn New(itemRegistry: &ItemRegistry) -> Result<Self, Box<dyn Error>> {
        let mut s = Self {
            BlocksAttributes: Vec::new(),
            BlockBehaviors: Vec::new(),
            StringToID: HashMap::new(),
            NumRegisteredBlocks: 0, //placeholder
            NumRegisteredTextures: 0,
        };

        if let Err(msg) = s.ReadBlockAttributes(itemRegistry) {
            return Err(msg);
        }

        Ok(s)
    }

    fn ReadBlockAttributes(&mut self, itemRegistry: &ItemRegistry) -> Result<(), Box<dyn Error>>{

         let mut blockCount = 0;
         let mut textureCount = 0;
         //TODO GET RID OF GENERIC ERRORS AND JUST USE MAP_ERR
         let path = std::path::Path::new("./minecraft_gl/assets/data/block/json/");
         for file in fs::read_dir(path).map_err(|e| format!("Error! Could not find ./minecraft_gl/assets/data/block/json/! The error:\n{}", e.to_string()))? {
            let path = file.map_err(|e| GenericError::NewBoxed(format!("Error! Could not retrieve file in ../../assets/data/block/json/ directory! The error:\n{}", e.to_string())))?.path();
            let file = File::open(path).map_err(|e| GenericError::NewBoxed(format!("Error! Could not open file of path in ../../assets/data/block/json/ directory! The error:\n{}", e.to_string())))?;
            let buff = BufReader::new(file);
            let json: Value = serde_json::from_reader(buff)?;

            let id = json["ID"].as_u64().unwrap() as u8;
            let name = json["Name"].as_str().unwrap();
            self.StringToID.insert(String::from(name), id);


            let mut attrib = BlockAttribute::default();
            //get the attributes...
            //TODO Make this function miror the other function more
            //TODO move all the base attributes out of the attribute tag in the json
            //TODO also rename it to custom attributes maybe idk
            if let Some(attributes) = json.get("Attributes") {
                let obj = attributes.as_object().unwrap();

                for pair in obj.iter() {
                    if pair.1.is_string() {
                        let val = match State::StateType(pair.1.as_str().unwrap()) {
                            Ok(val) => val,
                            Err(msg) => return Err(GenericError::NewBoxed(format!("{}. Error orignated from Item type {} of Id {}", msg, name, id)))
                        };
                        attrib.CustomAttributes.insert(pair.0.clone(), val);
                    }
                    else if pair.1.is_object() {
                        let rows = pair.1["Rows"].as_u64().unwrap() as u32;
                        let cols = pair.1["Cols"].as_u64().unwrap() as u32;
                        let mut vec: Vec<Item> = Vec::new();
                        vec.reserve((rows * cols) as usize);
                        attrib.CustomAttributes.insert(pair.0.clone(), State::Container((vec, rows, cols)));
                    }
                    else if pair.1.is_i64() {
                        let val = pair.1.as_i64().unwrap() as i32;
                        attrib.CustomAttributes.insert(pair.0.clone(), State::IntAttribute(val));
                    }
                    else if pair.1.is_f64() {
                        let val = pair.1.as_f64().unwrap() as f32;
                        attrib.CustomAttributes.insert(pair.0.clone(), State::FloatAttribute(val));
                    }
                    else if pair.1.is_boolean() {
                        let val = pair.1.as_bool().unwrap();
                        attrib.CustomAttributes.insert(pair.0.clone(), State::BoolAttribute(val));
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


            if let Some(val) = json.get("Toughness") { attrib.Toughness = val.as_f64().unwrap() as f32; }
            if let Some(val) = json.get("Friction") { attrib.Friction = val.as_f64().unwrap() as f32; }
            if let Some(val) = json.get("DropItem") { 
                if val.is_string() {
                    attrib.DropItem = Some(Item { ID: itemRegistry.IDofItem(val.as_str().unwrap()) }); 
                }
                else {
                     attrib.DropItem = Some(Item { ID: val.as_u64().unwrap() as u8 }); 
                }
            }
            if let Some(val) = json.get("EffectiveTool") { attrib.EffectiveTool = Some(Item { ID: val.as_u64().unwrap() as u8 }); }

            if let Some(textures) = json.get("Textures") {
                //TODO handle error if the texturees aren't an array
                 if ! textures.is_array() {
                    return Err(GenericError::NewBoxed(format!("Error in block registry creation! Texture attribute for {} of id {} must be an array of 6 elements", name, id)));
                 }
                 let paths = textures.as_array().unwrap();
                 
                 //intialize an empty texture array
                 let texs = [String::new(), String::new(), String::new(), String::new(), String::new(), String::new()];
                 let mut texData = TextureData { Textures: texs, TextureID: textureCount, Offsets: [0; 6] };

                 let mut cumul = 0;
                 let mut set: HashMap<String, u32> = HashMap::new();
                 for i in 0..6 {
                    texData.Textures[i] = String::from(paths[i].as_str().unwrap());

                    if ! set.contains_key(&texData.Textures[i]) {
                        cumul += 1 * (i != 0) as u32;
                        set.insert(texData.Textures[i].clone(), cumul);
                    }
                    
                    texData.Offsets[i] = set[&texData.Textures[i]];
                 }
                 textureCount += cumul + 1;
                 attrib.TextureData = Some(texData);
            }
            else {
                textureCount += 1; //for the null texture
            }

            //add the attribute to the attributes array
            if id as usize >= self.BlocksAttributes.len() {
                self.BlocksAttributes.resize((id + 1) as usize, BlockAttribute::default());
            }
            self.BlocksAttributes[id as usize] = attrib;

            blockCount += 1;
         }
         
         self.NumRegisteredBlocks = blockCount;
         self.NumRegisteredTextures = textureCount;

         Ok(())
    }

    fn ValidatePreviousAtlas(&self) -> Result<bool, String> {

        if  Path::new("./minecraft_gl/assets/data/block/atlas/atlas.png").exists() {
            //now check if it has the same textures through the data file...
            let file = std::fs::File::open("./minecraft_gl/assets/data/block/atlas/metaData.json")
            .map_err(|e| format!("Could not open metaData file of path {} for the block atlas.", "./minecraft_gl/assets/data/block/atlas/metaData.json"))?;
            
            let buff = BufReader::new(file);
            let json: Value = serde_json::from_reader(buff)
            .map_err(|e| format!("Could not read json meta data file for the block atlas!"))?;
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

        //TODO HAVE atlases of different texture resolutions and put the tex resolution in the name of file and in the json
        //TODO or have one master jason with all the atlas names and their resolutions
        //First check if the atlas already exists...
        if self.ValidatePreviousAtlas()? {
            let res = resource::GetImageFromPath("./minecraft_gl/assets/data/block/atlas/atlas.png")
            .expect("Could not open the pre-existing block atlas in ./minecraft_gl/assets/block/atlas/atlas.png!");
            
            //open the json again, would be sloppy to return the rows and cols from validatePreviousAtlas()
            let file = std::fs::File::open("./minecraft_gl/assets/data/block/atlas/metaData.json")
            .map_err(|e| format!("Could not open metaData file of path {} for the block atlas.", "./minecraft_gl/assets/data/block/atlas/metaData.json"))?;
            
            let buff = BufReader::new(file);
            let json: Value = serde_json::from_reader(buff).expect("Could not open block atlas metadata json!");
            let rows = json.get("Rows").unwrap().as_u64().unwrap() as u32;
            let cols = json.get("Cols").unwrap().as_u64().unwrap() as u32;

            return Ok(TextureAtlas::FromImage(res, rows, cols, textureResolution, display))
        }
        //attemp to make a square image out of the atlas...
        let dims = f32::ceil(f32::sqrt(self.NumRegisteredTextures as f32)) as u32;
        let mut img = image::RgbaImage::new(textureResolution * dims, textureResolution * dims);

        let mut runningTextureCount = 0;
        for idx in 0..self.NumRegisteredBlocks {

            if let Some(texData) = &self.BlocksAttributes[idx as usize].TextureData {
   
                for i in 0..6 {
                    if i == 0 || (i > 0 && texData.Offsets[i] != texData.Offsets[i - 1]) {
                        println!("IM IN HERE AGAIN BABY AT INDEX {} WITH {} AND {}", i, texData.Offsets[i], if i > 0 {texData.Offsets[i - 1]} else {0});
                        let mut pathBuf = PathBuf::new();
                        pathBuf.push("./minecraft_gl/assets/data/block/img");
                        pathBuf.push(texData.Textures[i].as_str());
                        let mut texture = resource::GetImageFromPath(pathBuf.as_path().as_os_str().to_str().unwrap())?;
                        texture = image::DynamicImage::ImageRgba8(image::imageops::resize(&texture, textureResolution, textureResolution, image::imageops::FilterType::Nearest));
                        let coords = ((runningTextureCount % dims) * textureResolution, (runningTextureCount / dims) * textureResolution);
                        image::imageops::overlay(&mut img, &mut texture, coords.0, coords.1);
                        runningTextureCount += 1;
                    }
                }
            }
            else{

                let mut texture = resource::GetImageFromPath("./minecraft_gl/assets/data/block/img/nullTexture.png")?;
                let texture = image::DynamicImage::ImageRgba8(image::imageops::resize(&mut texture, textureResolution, textureResolution, image::imageops::FilterType::Nearest));
                let coords = ((runningTextureCount % dims) * textureResolution, (runningTextureCount / dims) * textureResolution);
                image::imageops::overlay(&mut img, &texture, coords.0, coords.1);
                runningTextureCount += 1;
            }
 
        }
        
        let image = image::DynamicImage::ImageRgba8(img);
        
        //Save the image and some metadata about it
        image.save("./minecraft_gl/assets/data/block/atlas/atlas.png").expect("Could not save block atlas png!");
        let mut file = File::create("./minecraft_gl/assets/data/block/atlas/metadata.json").expect("Could not create block atlas metadata file!");

        let mut blocks: Vec<&str> = Vec::with_capacity(self.NumRegisteredBlocks as usize);
        for attrib in self.StringToID.keys() {
            blocks.push(&attrib);
        }
        let serialized = serde_json::to_string(&blocks).expect("Could not serialize block names to json fromat!");
        let finalStr = format!("{{\n\"Items\": {},\n\"Rows\": {},\n\"Cols\": {}\n}}", serialized, dims, dims);
        file.write_all(finalStr.as_bytes()).expect("Could not write to block atlas metadata file!");

        Ok(TextureAtlas::FromImage(image, dims, dims, textureResolution, display))
    }

    pub fn InitBehaviors(&self){
         
    }

    pub fn OnLeftClick(&self, blockName: &str, hit: Item) {
        let blockID = self.StringToID[blockName];
        if let Some(behavior) = &self.BlockBehaviors[blockID as usize] {
            (behavior.OnLeftClick)(self.GetAttributesOfID(blockID), hit);
        }
        let default = BlockBehavior::default();
        (default.OnLeftClick)(self.GetAttributesOfID(blockID), hit);
    }

    pub fn OnRightClick(&self, blockName: &str) -> Option<fn(&BlockAttribute, WindowEvent) -> bool> {
        let blockID = self.StringToID[blockName];
        if let Some(behavior) = &self.BlockBehaviors[blockID as usize] {
            return (behavior.OnRightClick)(self.GetAttributesOfID(blockID));
        }
        let default = BlockBehavior::default();
        (default.OnRightClick)(self.GetAttributesOfID(blockID))
    }

    pub fn GetAttributesOf(&self, block: &Block) -> &BlockAttribute{
        &self.BlocksAttributes[block.ID as usize]
    }

    pub fn GetAttributesOfID(&self, id: u8) -> &BlockAttribute{
        &self.BlocksAttributes[id as usize]
    }

    pub fn IDofBlock(&self, blockName: &str) -> u8{
        self.StringToID[blockName]
    }
}