
#[derive(Clone)]
pub struct Block{
    pub ID: u8
}
////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// /////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// /////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// /////////////////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////////////////////// 
use std::error::Error;
use std::io::BufReader;
use std::collections::HashMap;
use std::fs::{self, File};
use crate::Util::atlas::TextureAtlas;
use super::item::{Item};
use super::blockBehavior::BlockBehavior;
use glfw::WindowEvent;
use super::super::Util::resource;
use image;
use serde_json::Value;
use serde_json;

#[derive(Clone)]
pub struct TextureData{
    Textures: [String; 6],
    Offsets: [u32; 6],
}

#[derive(Clone)]
pub struct BlockAttribute{
    pub Toughness: f32,
    pub Friction: f32,
    pub DropItem: Option<Item>,
    pub EffectiveTool: Option<Item>,
    pub TextureData: Option<TextureData>,
}

impl Default for BlockAttribute{
    fn default() -> Self {
        Self { 
            Toughness: 1f32,
            Friction: 1f32, 
            DropItem: None, 
            EffectiveTool: None,
            TextureData: None
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
    pub fn New() -> Result<Self, Box<dyn Error>> {
        let mut s = Self {
            BlocksAttributes: Vec::new(),
            BlockBehaviors: Vec::new(),
            StringToID: HashMap::new(),
            NumRegisteredBlocks: 0, //placeholder
            NumRegisteredTextures: 0,
        };

        if let Err(msg) = s.ReadBlockAttributes() {
            return Err(msg);
        }

        Ok(s)
    }

    fn ReadBlockAttributes(&mut self) -> Result<(), Box<dyn Error>>{

         let mut blockCount = 0;
         let mut textureCount = 0;

         for file in fs::read_dir("../../assets/data/block/json")? {
            let path = file?.path();
            let file = File::open(path)?;
            let buff = BufReader::new(file);
            let json: Value = serde_json::from_reader(buff)?;

            let id = json["ID"].as_u64().unwrap() as u8;
            let name = json["Name"].as_str().unwrap();
            self.StringToID.insert(String::from(name), id);

            //get the attributes...
            let attributes = &json["Attributes"];

            let mut attrib = BlockAttribute::default();

            if let Some(val) = attributes.get("Toughness") { attrib.Toughness = val.as_f64().unwrap() as f32; }
            if let Some(val) = attributes.get("Friction") { attrib.Friction = val.as_f64().unwrap() as f32; }
            if let Some(val) = attributes.get("DropItem") { attrib.DropItem = Some(Item { ID: val.as_u64().unwrap() as u8 }); }
            if let Some(val) = attributes.get("EffectiveTool") { attrib.EffectiveTool = Some(Item { ID: val.as_u64().unwrap() as u8 }); }

            if let Some(textures) = attributes.get("Textures") {
                 let paths = textures.as_array().unwrap();
                 
                 //intialize an empty texture array
                 let texs = [String::new(), String::new(), String::new(), String::new(), String::new(), String::new()];
                 let mut texData = TextureData { Textures: texs, Offsets: [0; 6] };

                 let mut cumul = 0;
                 for i in 0..6 {
                    texData.Textures[i] = String::from(paths[i].as_str().unwrap());

                    if i > 0 && texData.Textures[i] != texData.Textures[i - 1] {
                        cumul += 1;
                    }
                    texData.Offsets[i] = cumul;
                 }
                 textureCount += cumul + 1;
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

    pub fn GenerateAtlas(&self, textureResolution: u32) -> Result<TextureAtlas, String> {
        //attemp to make a square image out of the atlas...
        let dims = f32::ceil(f32::sqrt(self.NumRegisteredTextures as f32)) as u32;
        let mut img = image::RgbaImage::new(textureResolution * dims, textureResolution * dims);

        let mut runningTextureCount = 0;
        for idx in 0..self.NumRegisteredBlocks {

            if let Some(texData) = &self.BlocksAttributes[idx as usize].TextureData {
                for i in 0..6 {
                    if i == 0 || (i > 0 && texData.Offsets[i] != texData.Offsets[i - 1]) {
            
                        let mut texture = resource::GetImageFromPath(texData.Textures[i].as_str())?;
                        image::imageops::resize(&mut texture, textureResolution, textureResolution, image::imageops::FilterType::Nearest);
                        let coords = ((runningTextureCount % dims) * textureResolution, (runningTextureCount / dims) * textureResolution);
                        image::imageops::overlay(&mut img, &mut texture, coords.0, coords.1);
                        runningTextureCount += 1;
                    }
                }
            }
            else{

                let mut texture = resource::GetImageFromPath("../../assets/data/block/img/nullTexture.png")?;
                image::imageops::resize(&mut texture, textureResolution, textureResolution, image::imageops::FilterType::Nearest);
                let coords = ((runningTextureCount % dims) * textureResolution, (runningTextureCount / dims) * textureResolution);
                image::imageops::overlay(&mut img, &texture, coords.0, coords.1);
                runningTextureCount += 1;
            }
 
        }
        
        Ok(TextureAtlas::FromImage(image::DynamicImage::ImageRgba8(img), dims, dims, textureResolution))
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