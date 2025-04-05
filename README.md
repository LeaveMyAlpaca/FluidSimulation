# FluidSimulation
## Video https://www.youtube.com/watch?v=QOP5tywBQ6o

 Fluid simulation written in rust using bevy game engine.
 - I based it of https://www.youtube.com/watch?v=rSKMYc1CQHE
 - It can run with 100k particles at 15 fps with 3 physics updates per frame so about 45 updates per second ( in debug mode)
 - it uses at max only about 40-45% of my CPU(Ryzen 7950X3D) without parked cores
# Main bottlenecks
 - Bevy's sprite rendering
 - Poor multi threading of algorithm
 - can't get this to run in release mode
#Potential improvements
 - profiling with tracey 
 - rendering particles as meshes by hand

# Vid
coming soon tm.
