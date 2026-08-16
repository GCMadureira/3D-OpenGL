# 3D OpenGL

To learn OpenGL, my goal was to create a simple 2D snake game using only OpenGL directly so I could learn both Rust and OpenGL. However, I got curious about the matrix calculations that made it possible to display a 3D space on the screen. Therefore, this simple example contains all the math necessary do enable this. All of it was written by me, using only OpenGL as a dependency (and GLFW to create a screen).

## Controls

* *Arrows* - move the snake
* *W and S* - move the camera up and down, respectively
* *A and D* - move the camera left and right, respectively
* *Q and E* - move the camera forward and backward, respectively
* *I and K* - change the pitch up and down, respectively
* *J and L* - change the yaw left and right, respectively
* *U and O* - change the roll counter clockwise and clockwise respectively