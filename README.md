# FluidSimulation
 Fluid simulation written in rust using bevy game engine.
 - I based it of https://www.youtube.com/watch?v=rSKMYc1CQHE
 - It can run with 10k particles at 15 fps with 3 physics updates per frame so about 45 updates per second ( in debug mode)
 - it uses at max only about 40-45% of my cpu(Ryzen 7950X3D) without parcked cores
# Main bottlenecks
 - Bevy's sprite rendering
 - Poor parallerization
 - can't get this to run in release mode
# Vid
comming soon tm.
