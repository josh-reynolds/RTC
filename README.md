# RTC
The project from Jamis Buck's "The Ray Tracer Challenge."

This started in Rust - that's the project under /raytrace. Everything is working there up through Chapter 
13: all features including rendering, primitive shapes, patterns and so on. However, I ran into difficulties 
implementing Groups from Chapter 14, likely due to my inexperience with Rust. I spent some time trying to 
solve, part of which is the side effort under /poly. The low velocity was aggravating, though, so I 
restarted from scratch in Python (under /python obviously), and was able to complete the challenge. Project 
comparisons and some thoughts for future improvements follow.

**Rust version**
Start date: 3/12/2024
Completed Chapter 13: 9/16/2024 (188 days)

Pencil render (600 x 300) - 5m 4s

**Python version**
Start date: 11/10/2024
Completed Chapter 13: 12/22/2024 (42 days)
Completed Chapter 16: 1/4/2025 (55 days)

Pencil render (600 x 300) - 18m 54s

As expected, Rust performance is considerably better, and Python development velocity is much higher (though
the latter measurement is slightly skewed by familiarity with the project, and distractions over the summer).

**TO_DO**
* review Python against PEP 8
* write user documentation
* general refactoring and deduplication
* performance optimizations
* * investigate NumPy
* * profiling
* * multi-threaded rendering
* stretch goal features from the text:
* * perturbed patterns (p. 140)
* * optional shadows (p. 165)
* * optimized cube intersection algorithm (p. 176)
* * shapes inheriting parent materials (p. 205)
* * area lighting / soft shadows (p. 239)
* * spotlights (p. 240)
* * focal blur (p. 241)
* * motion blur (p. 242)
* * anti-aliasing (p. 243)
* * texture maps (p. 244)
* * normal perturbation (p. 245)
* * torus primitive (p. 246)
* * scene file interpreter (p. 249)
