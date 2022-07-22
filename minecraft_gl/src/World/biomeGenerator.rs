
use bracket_noise::prelude::{FastNoise, NoiseType};
use rand::Rng;
use super::{chunk::{CHUNK_BOUNDS_X, CHUNK_BOUNDS_Z, CHUNK_BOUNDS_Y, To1D}, block::Block};


pub trait BiomeGenerator {
    fn Generate(&mut self, blocks: &mut Vec<Block>, chunkX: i32, chunkZ: i32);
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
        sign * 1f32 * f32::min(1f32, (height - self.MinHeight) / ((self.MaxHeight - self.MinHeight) * self.Speed)) + (1 * self.Decay as u8) as f32
    }
}

pub struct NoiseParameters {
    pub Octaves: i32,
    pub Seed: u64,
    pub Frequency: f32,
    pub Persistance: f32,
    pub Lacunarity: f32,
}

impl NoiseParameters {
    pub fn None() -> Self {
        Self {
            Octaves: 0,
            Seed: 0u64,
            Frequency: 0f32,
            Persistance: 0f32,
            Lacunarity: 0f32,
        }
    }

    pub fn Apply(&self, noise: &mut FastNoise){
        noise.set_frequency(self.Frequency);
        noise.set_fractal_octaves(self.Octaves);
        noise.set_fractal_lacunarity(self.Lacunarity);
        noise.set_seed(self.Seed);
    }
}

#[derive(Debug)]
pub struct GenerationData{
    pub Crust: Vec<(Block, HeightModifier)>,
    pub Mantle: Option<Block>,
    pub Core: Block,
    pub Ores: Vec<(Block, HeightModifier)>,
    pub MantleRange: (u32, u32),

    pub HeightLevel: u32,
    pub SurfaceAmplitude: u32,
    pub SeaLevel: u32,

    pub CaveModifier: HeightModifier,
    pub CaveCutoff: f32,

    pub OreCutoff: f32,
}

pub struct ForestGenerator {
    Noise: FastNoise,
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
            Frequency: 0.008f32,
            Lacunarity: (std::f64::consts::PI * 2.0 / 3.0) as f32,
            Persistance: 0.5f32,
            
        };

        let selection = NoiseParameters{
            Octaves: 6,
            Seed: rng.gen_range(0..10000),
            Frequency: 0.08f32,
            Lacunarity: (std::f64::consts::PI * 2.0 / 3.0) as f32,
            Persistance: 0.5f32,
        };

        let cave = NoiseParameters{
            Octaves: 6,
            Seed: rng.gen_range(0..10000),
            Frequency: 0.08f32,
            Lacunarity: (std::f64::consts::PI * 2.0 / 3.0) as f32,
            Persistance: 0.5f32,
        };

        let ore = NoiseParameters{
            Octaves: 6,
            Seed: rng.gen_range(0..10000),
            Frequency: 0.08f32,
            Lacunarity: (std::f64::consts::PI * 2.0 / 3.0) as f32,
            Persistance: 0.5f32,
        };

        let mut noise = FastNoise::new();
        noise.set_noise_type(NoiseType::Simplex);

        Self {
            Noise: noise,
            HeightMapNoise: height,
            SelectionNoise: selection,
            CaveNoise: cave,
            OreNoise: ore,
            GenData: genData,
        }
    }
}

impl BiomeGenerator for ForestGenerator {
    fn Generate(&mut self, blocks: &mut Vec<Block>, chunkX: i32, chunkZ: i32) {

        let mut heightMap =  [0; (CHUNK_BOUNDS_X * CHUNK_BOUNDS_Z) as usize];
        let mut crust = [Block::Air(); (CHUNK_BOUNDS_X * CHUNK_BOUNDS_Z) as usize];

        for x in 0..CHUNK_BOUNDS_X {
            for z in 0..CHUNK_BOUNDS_Z {
                self.HeightMapNoise.Apply(&mut self.Noise);
                let heightNoisenoise = self.Noise.get_noise((x as i32 + chunkX * CHUNK_BOUNDS_X as i32) as f32, (z as i32 + chunkZ * CHUNK_BOUNDS_Z as i32) as f32);
                let heightNoiseNormalized = (heightNoisenoise + 1f32) / 2f32;
                let height = (self.GenData.HeightLevel as f32 + self.GenData.SurfaceAmplitude as f32 * heightNoiseNormalized) as u32;
                heightMap[(z * CHUNK_BOUNDS_X + x) as usize] = height;

                self.SelectionNoise.Apply(&mut self.Noise);
                let crustNoise = self.Noise.get_noise3d(
                    (x as i32 + chunkX * CHUNK_BOUNDS_X as i32) as f32, 
                    (z as i32 + chunkZ * CHUNK_BOUNDS_Z as i32) as f32,
                    height as f32
                );
                let crustNoiseNormalized = (crustNoise + 1f32) / 2f32;
                crust[(z * CHUNK_BOUNDS_X + x) as usize] = GetBlockType(&self.GenData.Crust, crustNoiseNormalized, height as f32).unwrap();

            }

        }

        let mut rng = rand::thread_rng();
        for x in 0..CHUNK_BOUNDS_X {
            for z in 0..CHUNK_BOUNDS_Z {
                let mapIdx  = (x + z * CHUNK_BOUNDS_X) as usize;
                let height = heightMap[mapIdx];
                let crustBlock = crust[mapIdx];
                let mantleLength = rng.gen_range(self.GenData.MantleRange.0..self.GenData.MantleRange.1);

                for y in 0..CHUNK_BOUNDS_Y {
                    let idx = To1D((x, y, z)) as usize;

                    match y {
                        // _ if y > height && y <= SeaLevel => block = water,
                        _ if y > height => continue,
                        _ if y == height => blocks[idx] = crustBlock,
                        _ if y >= height - mantleLength => {
                            if let Some(block_) = self.GenData.Mantle {
                                blocks[idx] = block_;
                            } else {
                                blocks[idx] = crustBlock;
                            }
                        },
                        _ => blocks[idx] = self.GenData.Core
                    }; 
                    
                }
            }
        }
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

    if baseRanges.len() == 0 {panic!("No height ranges match the height of {}!", height)}
    //println!("How could it be 0 daddy hehehe :3 {:?} \n\n{:?} {}", baseRanges, blockData, height);

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