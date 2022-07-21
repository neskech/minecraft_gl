
use noise::{Seedable, NoiseFn};
use rand::Rng;
use super::{chunk::{CHUNK_BOUNDS_X, CHUNK_BOUNDS_Z}, block::Block};


pub trait BiomeGenerator {
    fn Sample(&self, x: f64, y: f64, z: f64) -> Block;
    fn HeightMap(&self, chunkX: i32, chunkY: i32) -> [u32; (CHUNK_BOUNDS_X * CHUNK_BOUNDS_Z) as usize];
}

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub enum Biome{
    Forest,
    Mountain,
    Desert,
    Trundraw, //TODO fix
    Swamp,
    None,
}

impl Biome {
    pub fn Random() -> Biome {
        let mut rng = rand::thread_rng();
        let val = rng.gen_range(0..=0);

        return match val {
            0 => Biome::Forest,
            // 1 => Biome::Mountain,
            // 2 => Biome::Desert,
            // 3 => Biome::Trundraw,
            // 4 => Biome::Swamp,
            _ => {
                panic!("Cannot generate Biome::None")
            }
        }
    }
}
#[derive(Debug)]
pub struct HeightModifier {
    pub MinHeight: f32,
    pub MaxHeight: f32,
    pub Speed: f32, //in units of how many blocks until the asymptope is reached
    pub Decay: bool,
    pub Constant: bool,
}

impl Default for HeightModifier{
    fn default() -> Self {
        Self {
            MinHeight: 0f32,
            MaxHeight: 0f32,
            Speed: 1f32,
            Decay: false,
            Constant: false,
        }
    }
}

impl HeightModifier {
    pub fn Sample(&self, height: f32) -> f32{
        if self.Constant { return 1f32; }
        let sign =  if self.Decay {-1f32} else {1f32};
        sign * (1.0f32 - f32::exp(-height / ( (self.MaxHeight - self.MinHeight) * self.Speed))) + (1 * self.Decay as u8) as f32
    }

    pub fn SampleLinear(&self, height: f32) -> f32{
        if self.Constant { return 1f32; }
        let sign = if self.Decay {-1f32} else {1f32};
        sign * 1f32 + f32::min(1f32, height / ((self.MaxHeight - self.MinHeight) * self.Speed)) + (1 * self.Decay as u8) as f32
    }
}

pub struct NoiseParameters {
    pub Octaves: usize,
    pub Seed: u32,
    pub Frequency: f64,
    pub Persistance: f64,
    pub Lacunarity: f64,
}

impl NoiseParameters {
    pub fn None() -> Self {
        Self {
            Octaves: 0,
            Seed: 0u32,
            Frequency: 0f64,
            Persistance: 0f64,
            Lacunarity: 0f64,
        }
    }

    pub fn ApplyToFBM(&self, fbm: &mut noise::Fbm){
        fbm.octaves = self.Octaves;
        fbm.frequency = self.Frequency;
        fbm.lacunarity = self.Lacunarity;
        fbm.persistence = self.Persistance;
    }

    pub fn ApplyToRidged(&self, fbm: &mut noise::RidgedMulti){
        fbm.octaves = self.Octaves;
        fbm.frequency = self.Frequency;
        fbm.lacunarity = self.Lacunarity;
        fbm.persistence = self.Persistance;
    }

    pub fn ApplyToBasic(&self, fbm: &mut noise::BasicMulti){
        fbm.octaves = self.Octaves;
        fbm.frequency = self.Frequency;
        fbm.lacunarity = self.Lacunarity;
        fbm.persistence = self.Persistance;
    }
}

#[derive(Debug)]
pub struct GenerationData{
    pub Blocks: Vec<(Block, HeightModifier)>,
    pub Ores: Vec<(Block, HeightModifier)>,

    pub HeightLevel: u32,
    pub SeaLevel: u32,

    pub CaveModifier: HeightModifier,
    pub CaveCutoff: f32,

    pub OreCutoff: f32,
}

pub struct ForestGenerator {
    HeightMapNoise: NoiseParameters,
    SelectionNoise: NoiseParameters,
    CaveNoise: NoiseParameters,
    OreNoise: NoiseParameters,
    GenData: GenerationData,

}

impl ForestGenerator {
    pub fn New(genData: GenerationData) -> Self {
        let mut rng = rand::thread_rng();
        let height = NoiseParameters{
            Octaves: 6,
            Seed: rng.gen_range(0..10000),
            Frequency: 0.08f64,
            Lacunarity: std::f64::consts::PI * 2.0 / 3.0,
            Persistance: 0.5f64,
            
        };

        let selection = NoiseParameters{
            Octaves: 6,
            Seed: rng.gen_range(0..10000),
            Frequency: 1f64,
            Lacunarity: std::f64::consts::PI * 2.0 / 3.0,
            Persistance: 0.5f64,
        };

        let cave = NoiseParameters{
            Octaves: 6,
            Seed: rng.gen_range(0..10000),
            Frequency: 1f64,
            Lacunarity: std::f64::consts::PI * 2.0 / 3.0,
            Persistance: 0.5f64,
        };

        let ore = NoiseParameters{
            Octaves: 6,
            Seed: rng.gen_range(0..10000),
            Frequency: 1f64,
            Lacunarity: std::f64::consts::PI * 2.0 / 3.0,
            Persistance: 0.5f64,
        };

        Self {
            HeightMapNoise: height,
            SelectionNoise: selection,
            CaveNoise: cave,
            OreNoise: ore,
            GenData: genData,
        }
    }
}

impl BiomeGenerator for ForestGenerator {
    fn Sample(&self, x: f64, y: f64, z: f64) -> Block{
        //TODO benchmark creating a new noise. If bad, make static noise objects whom's parameters can be changed

        // use std::time::Instant;
        // let now = Instant::now();

        let mut w = noise::Fbm::new();

        // let elapsed = now.elapsed();
        // println!("Elapsed: {:.2?}", elapsed);

        //cave...
        w = w.set_seed(self.CaveNoise.Seed);
        self.CaveNoise.ApplyToFBM(&mut w);

        use std::time::Instant;
        let now = Instant::now();
        let caveValue = self.GenData.CaveModifier.SampleLinear(y as f32) * (w.get([x, z, y]) as f32 + 1f32) / 2f32;

        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed);

        if caveValue >= self.GenData.CaveCutoff {
            return Block::Air();
        }

        //ores...
        w = w.set_seed(self.OreNoise.Seed);
        self.OreNoise.ApplyToFBM(&mut w);
        let oreValue =  (w.get([x, z, y]) as f32 + 1f32) / 2f32;

        if oreValue >= self.GenData.OreCutoff {
            let normalized = (oreValue - self.GenData.OreCutoff) / (1f32 - self.GenData.OreCutoff);
            if let Some(block) = GetBlockType(&self.GenData.Ores, normalized, y as f32) {
                return block;
            }
        }
        
        //Normal blocks...
        w = w.set_seed(self.SelectionNoise.Seed);
        self.OreNoise.ApplyToFBM(&mut w);
        let blockValue =  (w.get([x, z, y]) as f32 + 1f32) / 2f32;

        GetBlockType(&self.GenData.Blocks, blockValue, y as f32).unwrap()

    }   
    
    fn HeightMap(&self, chunkX: i32, chunkZ: i32) -> [u32; (CHUNK_BOUNDS_X * CHUNK_BOUNDS_Z) as usize] {
        let mut w = noise::Fbm::new();
        w = w.set_seed(self.HeightMapNoise.Seed);
        self.HeightMapNoise.ApplyToFBM(&mut w);

       // println!("Height level {}", self.GenData.HeightLevel);
        let mut arr = [0; (CHUNK_BOUNDS_X * CHUNK_BOUNDS_Z) as usize];
        for x in 0..CHUNK_BOUNDS_X {
            for z in 0..CHUNK_BOUNDS_Z {
                let noise = w.get([(x as i32 + chunkX * CHUNK_BOUNDS_X as i32) as f64, (z as i32 + chunkZ * CHUNK_BOUNDS_Z as i32) as f64]) as f32;
                let normalized = (noise + 1f32) / 2f32;
                arr[(z * CHUNK_BOUNDS_X + x) as usize] = (self.GenData.HeightLevel as f32 * normalized) as u32;
            }

        }
         arr
    }
}

// struct MountainGenerator {
//     Blocks: Vec<(Block, HeightModifier)>,
//     HeightMapNoise: NoiseParameters,
//     RidgeNoise: NoiseParameters,
//     SelectionNoise: NoiseParameters,
//     CaveNoise: NoiseParameters
// }

// impl BiomeGenerator for MountainGenerator {
//     fn Sample(x: f32, y: f32, z: f32) -> Block{
        
//     }
    
//     fn HeightMap(chunkX: f32, chunkY: f32) -> [u16; (CHUNK_BOUNDS_X * CHUNK_BOUNDS_Z) as usize] {
        
//     }
// }

// struct DesertGenerator {
//     Blocks: Vec<(Block, HeightModifier)>,
//     HeightMapNoise: NoiseParameters,
//     SelectionNoise: NoiseParameters,
//     CaveNoise: NoiseParameters
// }

// impl BiomeGenerator for DesertGenerator {
//     fn Sample(x: f32, y: f32, z: f32) -> Block{
        
//     }
    
//     fn HeightMap(chunkX: f32, chunkY: f32) -> [u16; (CHUNK_BOUNDS_X * CHUNK_BOUNDS_Z) as usize] {
        
//     }
// }

// struct TundraGenerator {
//     Blocks: Vec<(Block, HeightModifier)>,
//     HeightMapNoise: NoiseParameters,
//     SelectionNoise: NoiseParameters,
//     CaveNoise: NoiseParameters
// }

// impl BiomeGenerator for TundrawGenerator {
//     fn Sample(x: f32, y: f32, z: f32) -> Block{
        
//     }
    
//     fn HeightMap(chunkX: f32, chunkY: f32) -> [u16; (CHUNK_BOUNDS_X * CHUNK_BOUNDS_Z) as usize] {
        
//     }
// }

// struct SwampGenerator {
//     Blocks: Vec<(Block, HeightModifier)>,
//     HeightMapNoise: NoiseParameters,
//     SelectionNoise: NoiseParameters,
//     CaveNoise: NoiseParameters
// }

// impl BiomeGenerator for SwampGenerator {
//     fn Sample(x: f32, y: f32, z: f32) -> Block{
        
//     }
    
//     fn HeightMap(chunkX: f32, chunkY: f32) -> [u16; (CHUNK_BOUNDS_X * CHUNK_BOUNDS_Z) as usize] {
        
//     }
// }

fn GetBlockType(blockData: &Vec<(Block, HeightModifier)>, noiseValue: f32,  height: f32) -> Option<Block>{
    if blockData.len() == 0 {
        return None
    }

    //Store the index of the corresponding block, along with a proportion
    let mut baseRanges: Vec<(usize, f32)> = Vec::with_capacity(blockData.len());
    let mut idx = 0;
    for pair in blockData {
        if height <= pair.1.MaxHeight && height >= pair.1.MinHeight {
            baseRanges.push((idx, 1f32));
        }
        idx += 1;
    }

    if baseRanges.len() == 0 {return Some(Block::Air())}
   // println!("How could it be 0 daddy hehehe :3 {:?} {}", blockData, height);

    let len = baseRanges.len() as f32;
    for val in &mut baseRanges {
        val.1 /= len;
    }

    let mut augRanges: Vec<f32> = Vec::with_capacity(baseRanges.len());
    augRanges.resize(baseRanges.len(), 0f32);

    let mut range: f32;
    let mut extents: f32;
    for i in 0..baseRanges.len(){
        range = baseRanges[i].1;
        //Extend the range by a scalar value equally on both sides 
  
        let sample = blockData[baseRanges[i].0].1.SampleLinear(height);
        extents =  sample / 2f32 * range;
        augRanges[i] = range + extents;

        let mut negative = false;
        if augRanges[i] < 0f32{
            negative = true;
            augRanges[i] = 0f32;
        }

        //Push the range before us backwards to make room for this current range after its expanded
        let mut a: i32 = i as i32 - 1;
        while a >= 0 && ( (negative && extents < 0f32) || (!negative && extents > 0f32) ) {
            let tmp = augRanges[a as usize];
            augRanges[a as usize] = f32::max(0f32, augRanges[a as usize] - extents);
            extents += if extents < 0f32 {tmp} else {-tmp};
            a -= 1;
        }

    }

    //normalize values between 0 and 1
    let mut sum = 0f32;
    for val in &augRanges {
        sum += val;
    }

    //TODO find a better way to handle a 0 sum array
    sum += if sum == 0f32  {1f32} else {0f32}; //prevent a /0
    for val in &mut augRanges {
        *val /= sum;
    }

    let noise = (noiseValue + 1f32) / 2f32; //normalize noise value to [0,1]
    let mut runningSum = 0f32;
    //see what range the noise value falls in
    for i in 0..augRanges.len(){
        //if within this current range
        if noise >= runningSum && noise <= augRanges[i] + runningSum {
            return Some(blockData[baseRanges[i].0].0.clone());
        }
        runningSum += augRanges[i];
    }


    eprintln!("Error! GetBlockType function could not produce a block type. 
   //The array: {:?}, height value: {}, and base ranges: {:?}", augRanges, height, baseRanges);
    None
}