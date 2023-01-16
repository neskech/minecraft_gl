use std::sync::Arc;
use crate::{World::{chunk::{Chunk, CHUNK_BOUNDS_X, CHUNK_BOUNDS_Z}, block::{Block, TextureData, BlockRegistry}}, Renderer::worldRenderer::Vertex};

const ADJACENT_LEFT: i32 = 0;
const ADJACENT_RIGHT: i32 = 1;
const ADJACENT_UP: i32 = 2;
const ADJACENT_DOWN: i32 = 3;

const X_AXIS: usize = 0;
const Y_AXIS: usize = 1;
const Z_AXIS: usize = 2;
#[derive(PartialEq, Eq, Clone, Debug)]
enum BlockStateType {
    Solid,
    Air,
    Empty
}

impl Into<bool> for BlockStateType {
    fn into(self) -> bool {
        match self {
            Self::Solid => true,
            Self::Air => false,
            Self::Empty => false
        }
    }
}


pub fn SweepVolume(chunk: &mut Chunk, dimensions: &[usize; 3], 
               currentDimension: usize, adjacencyChunks: &[Option<Arc<Chunk>>; 4],
               blockRegistry: &BlockRegistry) {
     

     /*
           We get the other dimension axis. These will always
           be the other two integers in the 0..2 range that
           are NOT currentDimension
     */
     let axis1 = (currentDimension + 1) % 3;
     let axis2 = (currentDimension + 2) % 3;
     
     /*
            We sweep over a plane a multitude of times

            Each time we fill a 2D boolean mask which determines
            whether a block is present in that plane slice
     */
     let mut mask = vec![false; dimensions[axis1] * 
                                           dimensions[axis2]];

     let mut indexHolder: [i32; 3] = [0; 3];
     let mut normalVector: [i32; 3] = [0; 3];
     normalVector[currentDimension] = 1;
      
     /*
            Sweep over a volume by taking plane slices, where each plane 
            is defined by the two axis vectors. We move the planes in the
            direction of the normal vector dimensions[currentDimensions] + 1
            times

            Why + 1? Because we want the number of faces, not the number of blocks
     */
     indexHolder[currentDimension] = -1;
     while indexHolder[currentDimension] < dimensions[currentDimension] as i32 {
        //Fill our mask for this plane slice

        SweepPlane(chunk, &mut mask, axis1, axis2, dimensions, currentDimension, 
                   &mut indexHolder, &normalVector, adjacencyChunks);

        // if currentDimension == Y_AXIS && indexHolder[currentDimension] == 20 {
        //     println!("MASK {:?}", mask);
        // }

        indexHolder[currentDimension] += 1;

        ConstructMeshFromMask(chunk, &mut mask, axis1, axis2, dimensions, 
                              currentDimension, &mut indexHolder, blockRegistry);
     }
     
}

fn SweepPlane(chunk: &mut Chunk, mask: &mut Vec<bool>, axis1: usize, axis2: usize, 
              dimensions: &[usize; 3], currentDimension: usize, indexHolder: &mut [i32; 3], 
              normalVector: &[i32; 3], adjacencyChunks: &[Option<Arc<Chunk>>; 4]) {
    /*
        Index holder holds our indices for each axis x, y, z

        Dimensions array holds the length of each dimension of 
        the chunk

        Current dimension is the dimension perpendicular to the plane
        defined by axis1 and axis2

        Imagine we sweep a plane (again defined by axis1 and axis2) across
        our chunk in the direction of the 'current dimension' (aka either 
        the x y or z direction)

        Each intersection of this plane with our chunk represent a block face.
        So if there are n block faces on the x axis for example (defined by dimensions[X_AXIS])
        then there should be n + 1 total faces

        We rectify this by starting our iteration at -1. -1 represent the leftmost face at the
        of he x axis, bottomost face of the y axis, and forward most face of the z axis
        
    */
    //TODO fact check that last part

    //Our method of indexing into the mask
    let mut n: usize = 0;

    //Our axis values ranges from 0..2
    indexHolder[axis2] = 0;
    while indexHolder[axis2] < dimensions[axis2] as i32 {
        indexHolder[axis1] = 0;
        while indexHolder[axis1] < dimensions[axis1] as i32 {
            
            let bottomY = indexHolder[currentDimension] == -1 && 
            currentDimension == Y_AXIS;
            let topY = indexHolder[currentDimension] == dimensions[currentDimension] as i32 -1 && 
            currentDimension == Y_AXIS;

            //If we are on the bottom plane of the y axis...
            //Or the top plane of the y axis...
            if bottomY || topY {
                /*
                    simply skip this iteration while making
                    sure to still iterate our counters
                */
                n += 1;
                indexHolder[axis1] += 1;
                continue;
            }

            /*
                Index holder also acts as our current position in the chunk

                Our normal vector points perpendicular to the plane and towards
                the dimension defined by current dimension

                What we want to do is see if we need to draw a face between the current 
                block and the next block up in the direction of our normal vector

                We draw a face if...

                One block is AIR and the other isn't
                One block is EMPTY and the other isn't

                What's EMPTY mean? Empty means we reached the final block of the chunk 
                (in a particular direction) and thus looking forward with our normal 
                vector yields nothing back
            */
            let currentBlockSolid = IsSolid(indexHolder.clone(), dimensions, 
                                           currentDimension, adjacencyChunks, &chunk.Blocks);
            let nextBlockSolid = IsSolid(AddArrayVector(indexHolder, normalVector).clone(), 
                                                     dimensions, currentDimension, adjacencyChunks, &chunk.Blocks);

            let a: bool = currentBlockSolid.clone().into();
            let b: bool = nextBlockSolid.clone().into();
            if currentBlockSolid == BlockStateType::Empty || nextBlockSolid == BlockStateType::Empty {
                 mask[n] = false;
            }
            else {
                mask[n] = a != b;
            }
            
            //TODO change this shit
            if currentBlockSolid != BlockStateType::Empty &&
               nextBlockSolid != BlockStateType::Empty 
            {
                if (a && !b && indexHolder[currentDimension] == -1) || 
                   (!a && b && indexHolder[currentDimension] + normalVector[currentDimension] >= dimensions[currentDimension] as i32)
                {
                    mask[n] = false;
                }
            }

            n += 1;
            indexHolder[axis1] += 1;
        }
        indexHolder[axis2] += 1;
    }
}

fn ConstructMeshFromMask(chunk: &mut Chunk, mask: &mut Vec<bool>, axis1: usize, axis2: usize, 
    dimensions: &[usize; 3], currentDimension: usize, indexHolder: &mut [i32; 3], 
    blockRegistry: &BlockRegistry) 
{

        /*
            Loop over the plane defined by the mask to once again construct the mesh

            n is our mask index
        */
        let mut n = 0;

        for i in 0..dimensions[axis2] {
            let mut j = 0;
            while j < dimensions[axis1] {

                if !mask[n] {
                    j += 1;
                    n += 1;
                    continue;
                }

                let (width, height) = GetFaceDimensions(mask, axis1, axis2, dimensions, 
                                                    currentDimension, indexHolder, i, j, n, &chunk.Blocks);

                //
                ConstructFace(chunk, indexHolder.clone(), currentDimension, 
                              dimensions, blockRegistry, width, height, axis1,
                              axis2);

                //clear the mask to prevent the creation of duplicate faces
                for l in 0..height {
                    for k in 0..width {
                        mask[n + k + l * dimensions[axis1]] = false;
                    }
                }

                //We moved width points on both variables, so increment by that
                j += width;
                n += width;
            
            }
        }
}

fn ConstructFace(chunk: &mut Chunk, indexHolder: [i32; 3], currentDimension: usize, 
                 dimensions: &[usize; 3], blockRegistry: &BlockRegistry,
                 mut width: usize, mut height: usize, axis1: usize, axis2: usize) 
{
    let currBlock = GetBlock(indexHolder.clone(), currentDimension, 
                                    &chunk.Blocks, dimensions);
    
    let mut dimensionsQuadWidth = [0; 3];
    let mut dimensionsQuadHeight = [0; 3];
    dimensionsQuadWidth[axis1] = width as i32;
    dimensionsQuadHeight[axis2] = height as i32;

    let mut tmp = indexHolder.clone();
    tmp[currentDimension] += 1;

    let block1 = GetBlock(tmp.clone(), currentDimension,
                                 &chunk.Blocks, dimensions);
    let block2 = GetBlock(indexHolder.clone(), currentDimension,
                                 &chunk.Blocks, dimensions);
    let fID: i32 = ( indexHolder[currentDimension] != dimensions[currentDimension] as i32
                   && block1 == block2 ) as i32 + currentDimension as i32 * 2;


    let mut texID = 7_i32;
    if let Some(TextureData::SixSided(data)) = &blockRegistry.GetAttributesOf(&currBlock).TextureData {
        texID = data.TextureID as i32 + data.Offsets[fID as usize] as i32;//+ data.Offsets[fID as usize] as i32;
    } else {
         //TODO textureData should not be an optional. Either its real or the null texture
    }

    
    let offsets = [3, 2, 1, 0];
    //TODO why tf is this here
    if currentDimension == 0 {
        let temp = dimensionsQuadWidth;
        dimensionsQuadWidth = dimensionsQuadHeight;
        dimensionsQuadHeight = temp;

        let tmp = width;
        width = height;
        height = tmp;
    }
    
    //index holder is the top left of our quad
    //Top-left vertice position
    AddVertex(&indexHolder, texID, offsets[0], 
                    fID, width, height, &mut chunk.Mesh); 


                   // println!("One {:?}", indexHolder);
    //Top right vertice position
    let arr = [
                         indexHolder[0] + dimensionsQuadWidth[0], 
                         indexHolder[1] + dimensionsQuadWidth[1], 
                         indexHolder[2] + dimensionsQuadWidth[2]
                        ];
    AddVertex(&arr, texID, offsets[1], 
                     fID, width, height, &mut chunk.Mesh); 
                     
                    // println!("Two {:?}", arr);               
    //Bottom left vertice position
    let arr = [
                         indexHolder[0] + dimensionsQuadHeight[0], 
                         indexHolder[1] + dimensionsQuadHeight[1], 
                         indexHolder[2] + dimensionsQuadHeight[2]
                        ];  
    AddVertex(&arr, texID, offsets[2], 
                        fID, width, height, &mut chunk.Mesh);   

                      // println!("Three {:?}", arr);
    //Bottom right vertice position
    let arr = [
                         indexHolder[0] + dimensionsQuadWidth[0] + dimensionsQuadHeight[0], 
                         indexHolder[1] + dimensionsQuadWidth[1] + dimensionsQuadHeight[1], 
                         indexHolder[2] + dimensionsQuadWidth[2] + dimensionsQuadHeight[2]
                        ];
   // println!("Four {:?}\n", arr);
    AddVertex(&arr, texID, offsets[3], 
                    fID, width, height, &mut chunk.Mesh);    
    
}

fn GetFaceDimensions(mask: &Vec<bool>, axis1: usize, axis2: usize, dimensions: &[usize; 3], 
    currentDimension: usize, indexHolder: &mut [i32; 3], i: usize, j: usize, n: usize,
    blocks: &Vec<Block>) -> (usize, usize)
{

    indexHolder[axis2] = i as i32;
    indexHolder[axis1] = j as i32;

    let currBlock = GetBlock(indexHolder.clone(), currentDimension, 
                                    blocks, dimensions);

    let mut holderCopy = indexHolder.clone();
    holderCopy[axis1] += 1;

    /*  
        Sweep over the width axis (axis 1). Extend out more
        and more until we reach an empty block (mask is false)
    */
    let mut width = 1;
    while j + width < dimensions[axis1] && mask[n + width] &&
          currBlock == GetBlock(holderCopy.clone(), currentDimension, 
                                blocks, dimensions)
    {
        width += 1;
        holderCopy[axis1] += 1;
    }

    /*
        Now extend the quad's height along axis2
    */

    let mut height = 1;
    while i + height < dimensions[axis2] {

        //add one to height and reset width
        holderCopy[axis2] += 1;
        holderCopy[axis1] = indexHolder[axis1];

        for k in 0..width {
            //if there isn't a solid face present, the quad has a hole in it
            if !mask[k + n + height * dimensions[axis1]] || 
                currBlock != GetBlock(holderCopy.clone(), currentDimension, 
                                      blocks, dimensions) 
            {  
                return (width, height);
            }
            holderCopy[axis1] += 1;
        }

        height += 1;
    }

    (width, height)
}


fn IsSolid(mut point: [i32; 3], dimensions: &[usize; 3], currentDimension: usize, 
          adjacencyChunks: &[Option<Arc<Chunk>>; 4], blocks: &Vec<Block>) -> BlockStateType 
{
    /*
        Recall that dimensionIndex ranges from 0..2 where 
        0 = x axis
        1 = y axis
        2 = z axis

        Our adjacency array contains our adjacent chunks at particular indices
        These our defined by the ADJACENT constants. We index into the array below
        to see the adjacent constants for a particular axis
    */
    let adjacencyIndices = [(ADJACENT_LEFT, ADJACENT_RIGHT), 
                                             (0, 0), 
                                             (ADJACENT_UP, ADJACENT_DOWN)];
    
    /*
        first we check if we need to sample an adjacent chunk

        A LHS violation means we must sample the LHS chunk, defined
        by the first element of one of our tuples in the array

        A RHS violation means we must sample the RHS chunk, defined
        by the second element of one of our tuples in the array

        We modify our adjacent chunk index so that it will SAMPLE
        from the adjacent chunk instead of the current one

        A LHS violation means we sample from the RHS of the LHS chunk
        So we do dimensions[currentDimension] - 1

        A RHS violation means we sample from the LHS of the RHS chunk
        So we do 0 
    */

    let mut adjacentChunkIdx = -1;
    if point[currentDimension] == -1 {
        point[currentDimension] = dimensions[currentDimension] as i32 - 1;
        adjacentChunkIdx = adjacencyIndices[currentDimension].0;
    }
    else if point[currentDimension] == dimensions[currentDimension] as i32 {
        point[currentDimension] = 0;
        adjacentChunkIdx = adjacencyIndices[currentDimension].1;
    }

    //If we don't need to sample an adjacent chunk then...
    if adjacentChunkIdx == -1 {
        let air =  blocks[To1D(&point) as usize] == Block::Air();
        if air {return BlockStateType::Air} else {return BlockStateType::Solid};
    }

    //Else we must sample an adjacent chunk, but only if it's Some()...
    assert!(adjacentChunkIdx >= 0 && adjacentChunkIdx < 4);
    if let Some(chunk) = adjacencyChunks[adjacentChunkIdx as usize].clone() {
        let air = (*chunk).Blocks[To1D(&point) as usize] == Block::Air();
        if air {return BlockStateType::Air} else {return BlockStateType::Solid};
    }

    BlockStateType::Empty
}

fn AddVertex(point: &[i32; 3], textureId: i32, vertexId: i32, faceId: i32,
             width: usize, height: usize, mesh: &mut Vec<Vertex>)
{
    let core = point[0] | point[2] << 4 | point[1] << 8 | textureId << 16 | 
               vertexId << 24 | faceId << 26;
    let dims = width | height << 16;

    mesh.push(Vertex {Core: core as u32, Dims: dims as u32} );
}

fn GetBlock(mut point: [i32; 3], currentDimension: usize, blocks: &Vec<Block>,
            dimensions: &[usize; 3]) -> Block 
{
    point[currentDimension] = (point[currentDimension] - 1).max(0);
    let mut block = blocks[To1D(&point) as usize].clone();

    if block == Block::Air() && 
       point[currentDimension] < dimensions[currentDimension] as i32 -1
    {
        point[currentDimension] += 1;
        block = blocks[To1D(&point) as usize].clone();
    }

    block
}

fn AddArrayVector(v1: &[i32; 3], v2: &[i32; 3]) -> [i32; 3] {
    [v1[0] + v2[0], v1[1] + v2[1], v1[2] + v2[2]]
}

pub fn To1D(cord: &[i32; 3]) -> i32 {
    cord[0] + CHUNK_BOUNDS_X as i32 * (cord[2] + cord[1] * CHUNK_BOUNDS_Z as i32)
}