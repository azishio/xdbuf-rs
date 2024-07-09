pub mod step2d {
    pub const NEXT: [isize; 2] = [1, 0];
    pub const PREV: [isize; 2] = [-1, 0];

    pub const RIGHT: [isize; 2] = [1, 0];
    pub const LEFT: [isize; 2] = [-1, 0];
    pub const UP: [isize; 2] = [0, 1];
    pub const DOWN: [isize; 2] = [0, -1];

    pub const RIGHT_UP: [isize; 2] = [1, 1];
    pub const RIGHT_DOWN: [isize; 2] = [1, -1];
    pub const LEFT_UP: [isize; 2] = [-1, 1];
    pub const LEFT_DOWN: [isize; 2] = [-1, -1];
}

pub mod step3d {
    pub const NEXT: [isize; 3] = [1, 0, 0];
    pub const PREV: [isize; 3] = [-1, 0, 0];

    pub const RIGHT: [isize; 3] = [1, 0, 0];
    pub const LEFT: [isize; 3] = [-1, 0, 0];
    pub const FRONT: [isize; 3] = [0, 1, 0];
    pub const BACK: [isize; 3] = [0, -1, 0];
    pub const TOP: [isize; 3] = [0, 0, 1];
    pub const BOTTOM: [isize; 3] = [0, 0, -1];

    pub const RIGHT_FRONT: [isize; 3] = [1, 1, 0];
    pub const RIGHT_BACK: [isize; 3] = [1, -1, 0];
    pub const RIGHT_TOP: [isize; 3] = [1, 0, 1];
    pub const RIGHT_BOTTOM: [isize; 3] = [1, 0, -1];
    pub const LEFT_FRONT: [isize; 3] = [-1, 1, 0];
    pub const LEFT_BACK: [isize; 3] = [-1, -1, 0];
    pub const LEFT_TOP: [isize; 3] = [-1, 0, 1];
    pub const LEFT_BOTTOM: [isize; 3] = [-1, 0, -1];

    pub const FRONT_TOP: [isize; 3] = [0, 1, 1];
    pub const FRONT_BOTTOM: [isize; 3] = [0, 1, -1];
    pub const BACK_TOP: [isize; 3] = [0, -1, 1];
    pub const BACK_BOTTOM: [isize; 3] = [0, -1, -1];

    pub const RIGHT_FRONT_TOP: [isize; 3] = [1, 1, 1];
    pub const RIGHT_FRONT_BOTTOM: [isize; 3] = [1, 1, -1];
    pub const RIGHT_BACK_TOP: [isize; 3] = [1, -1, 1];
    pub const RIGHT_BACK_BOTTOM: [isize; 3] = [1, -1, -1];

    pub const LEFT_FRONT_TOP: [isize; 3] = [-1, 1, 1];
    pub const LEFT_FRONT_BOTTOM: [isize; 3] = [-1, 1, -1];
    pub const LEFT_BACK_TOP: [isize; 3] = [-1, -1, 1];
    pub const LEFT_BACK_BOTTOM: [isize; 3] = [-1, -1, -1];
}



