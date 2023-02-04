open_gl experiments
# Setup
`sudo apt install libsdl2-gfx-dev`
- for testing and debugging:
`sudo apt install mesa-utils`
# problem with opengl on Ubuntu with dual graphics cards
`$ glxinfo | grep OpenGL`
You should see either
`OpenGL vendor string: NVIDIA Corporation`
or 
`OpenGL vendor string: Intel`
Check which one is active:
`$ sudo prime-select query`
Switch to NVidia
`$ sudo prime-select nvidia`
# problem with opengl: version not supported
`glxinfo | grep 'version'`
You should get:
`OpenGL ES profile version string: OpenGL ES 3.2 Mesa 22.0.5`
Minimum required is ES 3.2!