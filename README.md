# CS_523_Solar_System_Model
A Rust project in which a Sun and a number of planets/planetoids are spawned and animated.  The amount of physics implemented will depend strongly on the difficulty of implementing said physics

Sun texture taken from wikimedia:
https://upload.wikimedia.org/wikipedia/commons/c/cb/Solarsystemscope_texture_2k_sun.jpg

additional textures from Solar System Scope:
https://www.solarsystemscope.com/textures/

ktx2 texture files generated using https://jaxry.github.io/panorama-to-cubemap/ to generate the cubefaces and then https://github.khronos.org/KTX-Software/ktxtools/ktx.html to package them into a texture file.

A small amount of work went into https://github.com/Plonq/bevy_panorbit_camera/issues/133 in order to try and resolve some camera limitations.  I will likely issue a pull request in order to install the clamps on the translation values despite those clamps not being needed on this project anymore.