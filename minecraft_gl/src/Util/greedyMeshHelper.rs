use std::sync::Arc;
use crate::{World::{chunk::{Chunk, CHUNK_BOUNDS_X, CHUNK_BOUNDS_Z}, 
            block::{Block, TextureData, BlockRegistry}}, 
            Renderer::worldRenderer::Vertex
           };

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
    Flora,
    Empty
}

impl Into<bool> for BlockStateType {
    fn into(self) -> bool {
        match self {
            Self::Solid => true,
            _ => false
        }
    }
}


pub fn SweepVolume(chunk: &mut Chunk, dimensions: &[usize; 3], 
               currentDimension: usize, adjacencyChunks: &[Option<Arc<Chunk>>; 4],
               blockRegistry: &BlockRegistry) 
{

    /*
        Index holder holds our indices for each axis x, y, z

        Dimensions array holds the length of each dimension of 
        the chunk

        Current dimension is the dimension perpendicular to the plane
        defined by axis1 and axis2

        Imagine we sweep a plane (again defined by axis1 and axis2) across
        our chunk in the direction of the 'current dimension' (aka either 
        the x y or z direction, defined by the normal vector)

        Each intersection of this plane with our chunk represent a block face.
        So if there are n block faces on the x axis for example (defined by n = dimensions[X_AXIS])
        then there should be n + 1 total faces

        We rectify this by starting our iteration at -1. -1 represent the leftmost face at the
        of he x axis, bottomost face of the y axis, and forward most face of the z axis
    */
     

     /*
           We get the other dimension axis. These will always
           be the other two integers in the 0..2 range that
           are NOT = currentDimension
     */
     let axis1 = (currentDimension + 1) % 3;
     let axis2 = (currentDimension + 2) % 3;
     
     /*
            We sweep over a plane multiple times to fill a volume

            Each time we fill a 2D boolean mask which determines
            whether a block face is present in that plane slice
     */
     let mut mask = vec![false; dimensions[axis1] * 
                                           dimensions[axis2]];

     let mut indexHolder: [i32; 3] = [0; 3];
     let mut normalVector: [i32; 3] = [0; 3];
     normalVector[currentDimension] = 1;
      
     /*
            Our chunk is 3D, meaning we have faces that are not constricted
            by a 2D plane

            This means if we want to draw all the faces on a particular axis,
            we have to sweep over a volume instead of a plane. The volume
            is defined as the number of plane slices needed to fill that volume

            As mentioned before, we start at -1 because there are n + 1 faces
            on any given axis where n = dimensions[currentDimension]
     */
     indexHolder[currentDimension] = -1;
     while indexHolder[currentDimension] < dimensions[currentDimension] as i32 {

        //Fill our mask for this plane slice
        SweepPlane(chunk, &mut mask, axis1, axis2, dimensions, currentDimension, 
                   &mut indexHolder, &normalVector, adjacencyChunks, blockRegistry);

        //advance 
        indexHolder[currentDimension] += 1;

        //construct mesh
        ConstructMeshFromMask(chunk, &mut mask, axis1, axis2, dimensions, 
                              currentDimension, &mut indexHolder, blockRegistry);
     }
     
}

fn SweepPlane(chunk: &mut Chunk, mask: &mut Vec<bool>, axis1: usize, axis2: usize, 
              dimensions: &[usize; 3], currentDimension: usize, indexHolder: &mut [i32; 3], 
              normalVector: &[i32; 3], adjacencyChunks: &[Option<Arc<Chunk>>; 4],
              blockRegistry: &BlockRegistry) {
    /*
       
    */

    //Our method of indexing into the mask
    let mut n: usize = 0;

    //Our axis values ranges from 0..2
    indexHolder[axis2] = 0;
    while indexHolder[axis2] < dimensions[axis2] as i32 {
        indexHolder[axis1] = 0;
        while indexHolder[axis1] < dimensions[axis1] as i32 {
            
            let bottomY = indexHolder[currentDimension] == -1;
            let topY = indexHolder[currentDimension] == dimensions[currentDimension] as i32 -1;

            //If we are on the bottom plane of the y axis...
            //Or the top plane of the y axis...
            if (bottomY || topY) && currentDimension == Y_AXIS {
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
            let currentBlock = IsSolid(indexHolder.clone(), dimensions, 
                                                    currentDimension, adjacencyChunks, &chunk.Blocks, blockRegistry);

            let nextBlock = IsSolid(AddArrayVector(indexHolder, normalVector).clone(), 
                                                     dimensions, currentDimension, adjacencyChunks, &chunk.Blocks,
                                                     blockRegistry);
            
               
            //If either block is out of bounds // empty don't fill the mask
            if currentBlock != BlockStateType::Empty && nextBlock != BlockStateType::Empty {
                let a: bool = currentBlock.into();
                let b: bool = nextBlock.into();

                /*
                    Case 1) If the current block is out of bounds -1, solid, and the next block is solid and in bounds
                    don't draw anything. Why? Because the other chunk should handle drawing this block, not us

                    Case 2) If the current block is in bounds and air and the next block is out of bounds and empty,
                            don't draw anything. Why? Because the other chunk should handle drawing this block, not us

                    If we don't consider these cases, then we risk drawing the INCORRECT block. The faces described
                    belong to the other chunk, and thus should be drawn by that chunk instead
                */
                let cond = (a && !b && indexHolder[currentDimension] == -1) || 
                                 (!a && b && indexHolder[currentDimension] == dimensions[currentDimension] as i32 - 1);

                if !cond {
                    mask[n] = a != b;
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

                /*
                    If the mask is empty simply increment our
                    counters and continue on
                */
                if !mask[n] {
                    j += 1;
                    n += 1;
                    continue;
                }

                //set to the top left of the quad
                indexHolder[axis2] = i as i32;
                indexHolder[axis1] = j as i32;

                //Get the current block and pass it to the following functions
                let currBlock =  GetBlock(indexHolder.clone(), currentDimension, 
                                                    &chunk.Blocks, dimensions, blockRegistry);

                //Get the width and height of this quad        
                let (width, height) = GetFaceDimensions(mask, axis1, axis2, dimensions, 
                                                        currentDimension, indexHolder, i, j, n, currBlock,
                                                        &chunk.Blocks, blockRegistry);
                    
                //Construct the face mesh and put it into our chunk
                ConstructFace(chunk, indexHolder.clone(), currentDimension, dimensions, 
                              blockRegistry, currBlock, width, height, axis1,
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

fn GetFaceDimensions(mask: &Vec<bool>, axis1: usize, axis2: usize, dimensions: &[usize; 3], 
    currentDimension: usize, indexHolder: &[i32; 3], i: usize, j: usize, n: usize, 
    currBlock: Block, blocks: &Vec<Block>, blockRegistry: &BlockRegistry) -> (usize, usize)
{
    //Make a copy of indexholder
    let mut holderCopy = indexHolder.clone();

    /*  
        Sweep over the width axis (axis 1). Extend out more
        and more until we reach an empty block (mask is false),
        reach out of bounds, or the next block isn't equal to the
        current block

        Note that all quads are automatically width and height = 1,
        so we start by defining that before the loop
    */
    let mut width = 1;
    holderCopy[axis1] += 1;
    while j + width < dimensions[axis1] && mask[n + width] &&
          currBlock == GetBlock(holderCopy.clone(), currentDimension, 
                                blocks, dimensions, blockRegistry)
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

        /*
            When we extend our quad on axis2, we want each row to be the same 
            exact width. Else our resulting shape wouldn't be a quad, but rather
            a weird polygon
        */
        for k in 0..width {

            //if there isn't a solid face present, the quad has a hole in it. Exit
            if !mask[k + n + height * dimensions[axis1]] || 
                currBlock != GetBlock(holderCopy.clone(), currentDimension, 
                                      blocks, dimensions, blockRegistry) 
            {  
                return (width, height);
            }

            holderCopy[axis1] += 1;
        }

        height += 1;
    }

    (width, height)
}

fn ConstructFace(chunk: &mut Chunk, indexHolder: [i32; 3], currentDimension: usize,  
    dimensions: &[usize; 3], blockRegistry: &BlockRegistry, currBlock: Block,
    mut width: usize, mut height: usize, axis1: usize, axis2: usize) 
{
    //Array vectors to store the width and height
    let mut dimensionsQuadWidth = [0; 3];
    let mut dimensionsQuadHeight = [0; 3];
    dimensionsQuadWidth[axis1] = width as i32;
    dimensionsQuadHeight[axis2] = height as i32;

    /*
        The face ID tells us which face we are currently looking at
        There are 6 faces in total distributed as so...

        [0, 1] --> X AXIS
        [2, 3] --> Y AXIS
        [4, 5] --> Z AXIS

        We do current dimension * 2 and then offset that by 1 
        depending on whether the block in the next plane slice 
        (plus the normal vector) is the same as the current block
    */
    let mut tmp = indexHolder.clone();
    tmp[currentDimension] += 1;

    let block1 = GetBlock(tmp.clone(), currentDimension,
                        &chunk.Blocks, dimensions, blockRegistry);
    let cond = block1 == currBlock;

    let fid: i32 = cond as i32 + currentDimension as i32 * 2;


    let mut texid = -1;
    if let TextureData::SixSided(data) = 
           &blockRegistry.GetAttributesOf(&currBlock).TextureData 
    {
            texid = data.TextureID as i32 + data.Offsets[fid as usize] as i32;
    } 
    else if let TextureData::Single(data) = 
                  &blockRegistry.GetAttributesOf(&currBlock).TextureData 
    {
        texid = data.TextureID as i32;
    }
        


    /*
        For some reason the x axis is oreinted sideways so we
        have to do this to fix it (swap width height)
    */
    if currentDimension == X_AXIS {
        let temp = dimensionsQuadWidth;
        dimensionsQuadWidth = dimensionsQuadHeight;
        dimensionsQuadHeight = temp;

        let tmp = width;
        width = height;
        height = tmp;
    }

    ConstructStandardFace(chunk, &indexHolder, texid, fid,
                             width, height, &dimensionsQuadWidth,
                             &dimensionsQuadHeight);


}

fn ConstructStandardFace(chunk: &mut Chunk, indexHolder: &[i32; 3], texId: i32, fId: i32,
                         width: usize, height: usize, dimensionsQuadWidth: &[i32; 3],
                         dimensionsQuadHeight: &[i32; 3]) 
{
    //vertex ID offsets
    let offsets = [3, 2, 1, 0];

    //index holder is the top left of our quad
    //Top-left vertice position
    AddVertex(&indexHolder, texId, offsets[0], 
        fId, width, height, &mut chunk.Mesh); 


    //Top right vertice position
    let arr = [
                indexHolder[0] + dimensionsQuadWidth[0], 
                indexHolder[1] + dimensionsQuadWidth[1], 
                indexHolder[2] + dimensionsQuadWidth[2]
            ];
    AddVertex(&arr, texId, offsets[1], 
            fId, width, height, &mut chunk.Mesh); 
                         
    //Bottom left vertice position
    let arr = [
                indexHolder[0] + dimensionsQuadHeight[0], 
                indexHolder[1] + dimensionsQuadHeight[1], 
                indexHolder[2] + dimensionsQuadHeight[2]
            ];  
    AddVertex(&arr, texId, offsets[2], 
            fId, width, height, &mut chunk.Mesh);   

    //Bottom right vertice position
    let arr = [
                indexHolder[0] + dimensionsQuadWidth[0] + dimensionsQuadHeight[0], 
                indexHolder[1] + dimensionsQuadWidth[1] + dimensionsQuadHeight[1], 
                indexHolder[2] + dimensionsQuadWidth[2] + dimensionsQuadHeight[2]
            ];

    AddVertex(&arr, texId, offsets[3], 
        fId, width, height, &mut chunk.Mesh);    
}

pub fn ConstructFloraFaces(chunk: &mut Chunk, indexHolder: &[i32; 3], texId: i32) 
{   
    let offsets = [3, 2, 1, 0];
    let fId = 0;

    let start1 = *indexHolder;
    let start2 = AddArrayVector(indexHolder, &[1, 0, 0]);
    let starts = [(start1, 1), (start2, -1)];

    for (start, sign) in starts {
        let arr = AddArrayVector(&start, &[0, -1, 0]);
        AddVertex(&arr, texId, offsets[1], 
            fId, 1, 1, &mut chunk.Mesh); 
    
        //+1 on height
        let arr= AddArrayVector(&start, &[0, 0, 0]);
        AddVertex(&arr, texId, offsets[3], 
                fId, 1, 1, &mut chunk.Mesh); 
                             
        //+-1 across
        let arr = AddArrayVector(&start, &[sign, -1, 1]);
        AddVertex(&arr, texId, offsets[0], 
                fId, 1, 1, &mut chunk.Mesh);   
    
        //+-1 across and +1 height
        let arr = AddArrayVector(&start, &[sign, 0, 1]);
        AddVertex(&arr, texId, offsets[2], 
            fId, 1, 1, &mut chunk.Mesh); 
    }

}

fn IsSolid(mut point: [i32; 3], dimensions: &[usize; 3], currentDimension: usize, 
          adjacencyChunks: &[Option<Arc<Chunk>>; 4], blocks: &Vec<Block>,
          blockRegistry: &BlockRegistry) -> BlockStateType 
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
        let block = blocks[To1D(&point)] ;
        let air =  block == Block::Air();

        if air {return BlockStateType::Air}

        let isFlora = IsFlora(block, blockRegistry);
        if isFlora {return BlockStateType::Flora} else {return BlockStateType::Solid};
    }

    //Else we must sample an adjacent chunk, but only if it's Some()...
    assert!(adjacentChunkIdx >= 0 && adjacentChunkIdx < 4);
    if let Some(chunk) = adjacencyChunks[adjacentChunkIdx as usize].clone() {
        let block = (*chunk).Blocks[To1D(&point)];
        let air = block == Block::Air();

        if air {return BlockStateType::Air}

        let isFlora = IsFlora(block, blockRegistry);
        if isFlora {return BlockStateType::Flora} else {return BlockStateType::Solid};
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
            dimensions: &[usize; 3], blockRegistry: &BlockRegistry) -> Block 
{
    point[currentDimension] = (point[currentDimension] - 1).max(0);
    let mut block = blocks[To1D(&point)].clone();

    //treat flora as air
    if IsFlora(block, blockRegistry) {
        block = Block::Air()
    }

    /*
        If I'm air just get the next block in the direction
        of the normal vector
    */
    if block == Block::Air() && 
       point[currentDimension] < dimensions[currentDimension] as i32 -1
    {
        point[currentDimension] += 1;
        block = blocks[To1D(&point)].clone();
    }

    block
}

pub fn AddArrayVector(v1: &[i32; 3], v2: &[i32; 3]) -> [i32; 3] {
    [v1[0] + v2[0], v1[1] + v2[1], v1[2] + v2[2]]
}

pub fn To1D(cord: &[i32; 3]) -> usize {
    let t =  cord[0] + CHUNK_BOUNDS_X as i32 * (cord[2] + cord[1] * CHUNK_BOUNDS_Z as i32);
    t as usize
}

pub fn IsFlora(block: Block, blockRegistry: &BlockRegistry) -> bool {
    blockRegistry.NameToID("Flower").filter(|e| block.ID == *e).is_some() ||
    blockRegistry.NameToID("tallGrass").filter(|e| block.ID == *e).is_some()
}