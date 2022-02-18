
# BuilderGenerator
- A CLI tool used for rendering Blender files to a cropped spritesheet

## Usage
General project structure for rendering:
* `CHARACTER_NAME/` - top level folder containing character name
* * `character.blend` - the rigged character file to use. All files with the `character.blend` name will be ignored.
* * `ANIMATION_WALK.blend` - For each animation, a separate `.blend` file should be made. This will then be parsed and ran.
* * `ANIMATION_RUN.blend` - For each animation, a separate `.blend` file should be made. This will then be parsed and ran.

## Future Roadmap/Nice to haves
- [ ] Add in multithreading
- [ ] Add in multiple perspectives
