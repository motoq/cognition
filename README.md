Cognition
=========

Framework for study of math and physics related concepts

cogs
----

The cogs directory is the library layer serving other directories
(crates) that are applications.

orbiter
-------

<img src="./sparky_rs.pn">

The orbiter directory is a 6DOF spacecraft attitude dynamics, estimation,
and control simulator.  The current effort focuses on recreating functionality
previously implemented in Java that was abandoned out of frustration due to the
loss of maintenance of the Java3D library.  The Rust based Kiss3d graphics
engine supported by Dimforge has proven functional yet simple to use for the
display of scientific computing data.

Current functionality includes defining an earth centered orbit and choosing
either a 2-body or J2 based gravity model propagated via RK4.  More options
w.r.t. the central body, similar to the Java version, may be added once
the attitude dynamics, control system, and attitude determination have been
reimplemented.  For now, the Rust version is more focused.

After installing the cognition package, the orbiter simulation can be run
out of the orbiter directory with the command:

*cargo run config_file.toml*

The *cfg_dyn.toml* example input flys Sparky the spacecraft in a highly
elliptical orbit.  The *cfg_leo.toml* configuration is a low earth orbit.
All user input is currently disabled for the dynamic simulations (work
currently in progress).  The *cfg_notdyn.toml* disable all dynamics
and allow the user to rotate Sparky via the ASDFGE keys (the mouse must
be over the OpenGL window with Sparky to capture the keyboard inputs).
The text window displays the attitude quaternion.
