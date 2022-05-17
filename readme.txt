Scene is created using a left-handed coordinate system.
[1.0, 0.0, 0.0] is to the right (x axis)
[0.0, 1.0, 0.0] is up (y axis)
[0.0, 0.0, 1.0] is toward the viewer out of the screen (z axis)

Camera is at the origin [0.0, 0.0, 0.0]
Camera view direction is [0.0, 0.0, -1.0]
Camera up direction is [0.0, 1.0, 0.0]

Images at https://www.cs.drexel.edu/~wtb35/cs636/hw4.html

Source code of note...

Entrypoint for rendering with adaptive AA - objects/scene.rs, render_supersample_frame_threaded, line 176

Recursive adaptive AA - objects/scene.rs, supersample_recurse, line 244
