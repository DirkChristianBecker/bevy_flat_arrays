bevy_flat_arrays
==========
This project implements 2 and 3 dimensional arrays for bevy. Internally these arrays are flattened to avoid indirections and cache misses (while iterating) and thereby improving performance. 