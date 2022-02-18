# A Python script for updating the render options
# Inspired by http://clintbellanger.net/articles/isometric_tiles/
# Copyright 2021, Eric Olson
# MIT licensed

import bpy
import math
import os
import sys
from math import cos, sin, radians
from mathutils import Vector, Euler

CWD = os.getcwd()
FILE = bpy.path.basename(bpy.context.blend_data.filepath)  # Also the animation
FILE_NAME = FILE.replace(".blend", "")

# Some defaults
ORTHO_SCALE = 10.0
AA_SAMPLES = 1.0

# Parse args
argv = sys.argv
argv = argv[argv.index("--") + 1:]  # get all args after "--"

OUTPUT_PATH = argv[0]
MODEL_NAME = argv[1]
RENDER_WIDTH = int(argv[2])
RENDER_HEIGHT = int(argv[3])
NUM_ROTATIONS = int(argv[4])


# Set the scene coordinates
for scene in bpy.data.scenes:
    scene.render.resolution_x = RENDER_WIDTH
    scene.render.resolution_y = RENDER_HEIGHT
    scene.render.resolution_percentage = 100
    scene.render.use_border = False
    scene.render.film_transparent = True
    scene.render.filter_size = 0.0
    scene.eevee.taa_render_samples = AA_SAMPLES


def set_camera(location, rotation):
    for obj in bpy.context.scene.objects:
        if obj.type == 'CAMERA':
            # Convert camera to orthographic
            obj.data.type = 'ORTHO'
            obj.data.ortho_scale = ORTHO_SCALE
            obj.data.clip_start = 0.001
            obj.data.clip_end = 100.0

            # Set the position
            obj.location = location
            obj.rotation_euler = rotation


def position(x, y, z):
    return Vector((x, y, z))


def rotation(x, y, z):
    rotation_rads = (math.radians(x), math.radians(y), math.radians(z))
    return Euler(rotation_rads)


def set_lighting(rotation):
    LIGHTING_NAME = "__renderer_scene_lighting__"

    # remove any existing default lighting in the event we're using a previously rendered scene
    for obj in bpy.context.scene.objects:
        if obj.name == LIGHTING_NAME:
            obj.select_set(True)
            bpy.ops.object.delete()

    # create light datablock, set attributes
    light_data = bpy.data.lights.new(name=LIGHTING_NAME, type='SUN')
    light_data.energy = 1

    # create new object with our light datablock
    light_object = bpy.data.objects.new(
        name=LIGHTING_NAME, object_data=light_data)

    # link light object
    bpy.context.collection.objects.link(light_object)

    # make it active
    bpy.context.view_layer.objects.active = light_object

    # change props
    light_object.location = (0, 0, 20)
    light_object.rotation_euler = rotation

    # update scene, if needed
    dg = bpy.context.evaluated_depsgraph_get()
    dg.update()


# Triggers a render
def render(perspective):
    # Trigger render
    # Use '_ESCAPED' to prevent blender file names from mucking with the Rust parsing.
    bpy.context.scene.render.filepath = f'{CWD}/{OUTPUT_PATH}/{FILE_NAME}_ESCAPED{perspective}_ESCAPED'
    bpy.ops.render.render(animation=True, write_still=True)


def render_isometric():
    degs_per_rotation = 360.0 / float(NUM_ROTATIONS)
    initial_rotation = 0

    # default position + rotation
    init_x = 0
    init_y = -15
    z = 10
    pos = position(init_x, init_y, z)
    rot = rotation(60, 0, initial_rotation)

    for rotation_idx in range(0, NUM_ROTATIONS):
        rot_degs = -degs_per_rotation * rotation_idx
        rot_rads = radians(rot_degs)

        # Get new position + rotation for camera and light
        new_x = init_x * cos(rot_rads) + init_y * sin(rot_rads)
        new_y = -init_x * sin(rot_rads) + init_y * cos(rot_rads)

        pos = position(new_x, new_y, z)

        # Leave this
        rot = rotation(60, 0, -rot_degs)
        light_rot = rotation(40, 0, -rot_degs)

        set_lighting(light_rot)
        set_camera(pos, rot)

        render(rotation_idx + 1)


render_isometric()
