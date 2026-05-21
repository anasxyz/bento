#[derive(Clone, Copy)]
pub struct Accumulated {
    pub offset_x: f32,
    pub offset_y: f32,
    pub opacity: f32,
    pub clip: Option<[f32; 4]>,
    pub rotate: f32,
    pub scale_x: f32,
    pub scale_y: f32,
}

impl Accumulated {
    pub fn identity() -> Self {
        Self {
            offset_x: 0.0,
            offset_y: 0.0,
            opacity: 1.0,
            clip: None,
            rotate: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
        }
    }

    pub fn push(&self, offset_x: f32, offset_y: f32, clip: Option<[f32; 4]>) -> Self {
        Self {
            offset_x: self.offset_x + offset_x,
            offset_y: self.offset_y + offset_y,
            opacity: self.opacity,
            clip: merge_clip(self.clip, clip),
            rotate: self.rotate,
            scale_x: self.scale_x,
            scale_y: self.scale_y,
        }
    }
}

fn merge_clip(a: Option<[f32; 4]>, b: Option<[f32; 4]>) -> Option<[f32; 4]> {
    match (a, b) {
        (Some(a), Some(b)) => {
            let x = a[0].max(b[0]);
            let y = a[1].max(b[1]);
            let x2 = (a[0] + a[2]).min(b[0] + b[2]);
            let y2 = (a[1] + a[3]).min(b[1] + b[3]);
            Some([x, y, (x2 - x).max(0.0), (y2 - y).max(0.0)])
        }
        (Some(a), None) => Some(a),
        (None, Some(b)) => Some(b),
        (None, None) => None,
    }
}
