//! A simple animation library using the `crow` rendering crate.
use crow::Texture;

#[derive(Debug, Clone)]
pub struct Sprite {
    pub texture: Texture,
    pub offset: (i32, i32),
}

#[derive(Debug, Clone)]
pub struct AnimationState {
    pub current: AnimationHandle,
    pub frame: usize,
}

#[derive(Debug, Clone, Default)]
pub struct Animation {
    pub frames: Vec<Sprite>,
    pub next: AnimationHandle,
}

impl Animation {
    pub fn empty() -> Self {
        Animation {
            frames: Vec::new(),
            next: AnimationHandle(0),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct AnimationHandle(usize);

#[derive(Debug, Clone)]
pub struct AnimationStorage {
    unique: Vec<Animation>,
}

impl AnimationStorage {
    pub fn new() -> Self {
        AnimationStorage {
            unique: Vec::new(),
        }
    }

    pub fn get(&self, handle: AnimationHandle) -> &Animation {
        &self.unique[handle.0]
    }

    pub fn get_mut(&mut self, handle: AnimationHandle) -> &mut Animation {
        &mut self.unique[handle.0]
    }

    pub fn insert(&mut self, animation: Animation) -> AnimationHandle {
        assert!(animation.frames.len() > 0);
        let idx = AnimationHandle(self.unique.len());
        self.unique.push(animation);
        idx
    }

    pub fn next(&self, state: &mut AnimationState) -> Sprite {
        let anim = self.get(state.current);
        let frame = anim.frames.get(state.frame).unwrap();
        state.frame += 1;
        if anim.frames.len() > state.frame {
            frame.clone()
        } else {
            state.current = anim.next;
            state.frame = 0;
            frame.clone()
        }
    }
}
