# raytracer
A software raytracer written in Rust.

Supported features:
- Rendering of models in [`smf`](https://people.sc.fsu.edu/~jburkardt/txt/smf_format.txt) format.
- Phong and parts of [Hall Greenberg](https://ieeexplore.ieee.org/document/4037684) shading models
- Multi-color point lights
- Shadows, reflections, and refraction
- Bounding volume hierarchy acceleration
- Post-processing supersample anti-aliasing
- In-process adaptive supersample anti-aliasing

Examples of generated images. All images are 1024x1024 resolution and use in-processing adaptive super sampling.

![out1_shadow](https://user-images.githubusercontent.com/19415820/172049880-4e29a4bf-5f7a-47d3-afa8-9b0020fc309e.png)
![out3_shadow](https://user-images.githubusercontent.com/19415820/172049902-9d5c725a-d92d-43ee-a91c-cdbd465ff668.png)
![out1_reflect](https://user-images.githubusercontent.com/19415820/172049956-cadbe88f-b2e9-48e5-ac8e-e3f7d937847f.png)
![out3_reflect](https://user-images.githubusercontent.com/19415820/172049958-44704ee9-1c8c-4b85-a2b6-5af4c3ba0915.png)
![out1_refract](https://user-images.githubusercontent.com/19415820/172050005-cda829ac-f603-4fa5-ba40-b5ec5a80de97.png)
![out2_refract](https://user-images.githubusercontent.com/19415820/172050008-c14d1be9-0e70-4a5d-bd70-87f368a682cf.png)
![out3_refract](https://user-images.githubusercontent.com/19415820/172050011-8ae50933-b612-4329-85de-c435fa33b409.png)
