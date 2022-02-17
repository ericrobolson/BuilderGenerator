* Uses blender v2.93.5

Architecture:
* `sprite_maker` - A CLI tool for rendering Blender files in an isometric view
* `sprite_sheets` - A library that includes everything for spritesheets, from data structures to encoding of images.

General project structure for rendering:
* `CHARACTER_NAME/` - top level folder containing character name
* * `character.blend` - the rigged character file to use. All files with the `character.blend` name will be ignored.
* * `ANIMATION_WALK.blend` - For each animation, a separate `.blend` file should be made. This will then be parsed and ran.
* * `ANIMATION_RUN.blend` - For each animation, a separate `.blend` file should be made. This will then be parsed and ran.

Roadmap
- [x] Update `render.py` to update resolution
- [x] Update `render.py` to render animations + all frames
- [x] Update `render.py` to render objects from multiple views
- [x] Update `render.py` to set camera + lights in all perspectives you want
- [x] Make aliasing configurable
- [x] Update `render.py` to set lights in all perspectives you want
- [x] sprite_maker: Make program
- [x] sprite_maker: List all files in a single directory
- [x] sprite_maker: Build out JSON object
- [x] sprite_maker: Add in cropping of images + offset calculations.
- [x] sprite_maker: Add in benchy
- [x] sprite_maker: Git rid of HashMaps and instead make it sorted. Right now the hashing algorithm is non-deterministic, which results in differing results each run.
- [x] sprite_sheet: Expose a library that can be called by other programs.
- [x] Make sprite_maker a CLI. Takes in the root folder for parsing blend files, then renders everything. 
- [x] Embed Python script + execution of blender?
- [x] Add args to render.py, [see this article](https://blender.stackexchange.com/a/8405)
- [x] Add in cfg for for sprite size in CLI args
- [x] Refactor sprite_sheet to be easier to use? Caller shouldn't have to manually do offsets. Add in both pixel and float calculations that are saved. Pixels are self explanatory, floats should use a range of 0.0-1.0
- [x] Fix current bug with renders being output strangely
- [x] Fix bug + add in configurable number of rotations. Start facing left?
- [x] sprite_maker: Test out multiple animations
- [x] Add example. TODO: showcase idle + walking animation + directions. Make a bash script which clones the SpriteMaker repo, then executes on some blend files, puts it in the example `resources/renders/` folder, then runs it.
- [ ] sprite_maker: Post on GitHub with a repo for `sprite_sheet`, `sprite_maker`, and `sprite_maker_example` which is a project that gets the source, builds it, runs it all. Example for how things are structured.
- [ ] Refactor example to clone sprite_maker repo

Future goals:
- [ ] Add in multithreading
- [ ] Add in multiple perspectives