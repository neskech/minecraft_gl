# MinecraftGL

A *work in progress* Minecraft clone made using Rust and opengGL

I plan to rewrite this entire project in the future. This was my first ever Rust application and as such a lot of it isn't idiomatic

Project should be running again! Note that I developed this on MacOS so I can't
make any guarantees as to whether or not this will run on windows

# Features
  -
      - Work in progress greedy meshing ![](./img/white.png) ![](./img/flaura.png)
      - Dynamic creation of texture atlases at runtime
      - ![](./minecraft_gl/assets/data/block/atlas/atlas.png)
      - Multithreaded chunk generation

 # Plans
   -
      - General cleanup of the chunk generation system code
      - Event based communication between the scene and renderer
      - Switch back to raw OpenGL
      - Backface culling
      - Batching of chunks into a single draw call
      - CubeMap for skybox
      - Transparent blocks via sorting of mesh faces in particular chunks
      - Better JSON parsing with helper functions for better organization
      - Null texture implementation
      - Noisemap generation on a seperate thread to reduce chunk generation time
      - Dynamic biomes
      - Physics and a player model
      - GUI system utilizing the EGUI library
      - Interactable blocks such as chests, crafting tables, and furnaces
      - Defferred rendering pipeline for ambient occlusion, shadows, and dynamic light sources
      - Hostile and friendly mobs utilizing the entity component system
      
