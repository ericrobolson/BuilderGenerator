use super::{sorted_map::SortedMap, *};
use crate::Render;
use benchy::Benchy;

pub fn execute(animations: SortedMap<String, Vec<ImgToRender>>) -> Vec<Render> {
    Benchy::time("render_animations");

    let mut rendered_animations = vec![];

    for (animation, imgs) in animations.iter() {
        let rendered = render_animation::execute(animation.clone(), imgs.clone());
        rendered_animations.push(rendered);
    }

    rendered_animations
}
