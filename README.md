# OSM2MC Studio 🛣️🧊

An end-to-end web application, API, and high-performance Rust geospatial engine designed to convert 1:1 real-world **OpenStreetMap (OSM)** vector data into playable **Minecraft World Saves** (`.mca`) and **Schematic files** (`.schem` / `.litematic` / `.schematic`).

## Key Features
- **1:1 Meter-to-Block Accuracy:** Real-world road widths, building floorplans, and heights mapped with exact scaling.
- **Accurate Surface Materials:** Uses Cyan Terracotta for asphalt, Stone for grey pavements, and White/Yellow Wool for lane stripes and tactile crossings.
- **Wikimedia MUTCD Integration:** Automatically fetches high-resolution route shields and renders them directly onto Item Frame overhead gantry signs using Minecraft map art (`map_X.dat`).
- **Complete Cross-Version Support:** Targets Java 1.7.10 through 1.21.1+ and Bedrock Edition up to 1.26.50.

## Project Structure
- `/index.html`: Complete single-file Web UI with Highway Gothic styling.
- `/server`: Python FastAPI server orchestrating Overpass queries, Wikimedia processing, and zip bundling.
- `/core_engine`: High-performance Rust binary and PyO3 module executing 1:1 geospatial buffering, Bézier curve interpolation, and NBT region generation.
- `/data_assets`: Templates and legacy translation matrices (e.g., mapping modern block states to 1.7.10 numeric IDs).
- `/output/worlds`: Local storage location for generated world downloads.
## License
Licensed under the Apache License, Version 2.0 (January 2004). See `LICENSE` for details.
