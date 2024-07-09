pub mod step2d {
    const NEXT: [isize; 2] = [1, 0];
    const PREV: [isize; 2] = [-1, 0];

    const RIGHT: [isize; 2] = [1, 0];
    const LEFT: [isize; 2] = [-1, 0];
    const UP: [isize; 2] = [0, 1];
    const DOWN: [isize; 2] = [0, -1];

    const RIGHT_UP: [isize; 2] = [1, 1];
    const RIGHT_DOWN: [isize; 2] = [1, -1];
    const LEFT_UP: [isize; 2] = [-1, 1];
    const LEFT_DOWN: [isize; 2] = [-1, -1];
}

pub mod step3d {
    const NEXT: [isize; 3] = [1, 0, 0];
    const PREV: [isize; 3] = [-1, 0, 0];

    const RIGHT: [isize; 3] = [1, 0, 0];
    const LEFT: [isize; 3] = [-1, 0, 0];
    const FRONT: [isize; 3] = [0, 1, 0];
    const BACK: [isize; 3] = [0, -1, 0];
    const TOP: [isize; 3] = [0, 0, 1];
    const BOTTOM: [isize; 3] = [0, 0, -1];

    const RIGHT_FRONT: [isize; 3] = [1, 1, 0];
    const RIGHT_BACK: [isize; 3] = [1, -1, 0];
    const RIGHT_TOP: [isize; 3] = [1, 0, 1];
    const RIGHT_BOTTOM: [isize; 3] = [1, 0, -1];
    const LEFT_FRONT: [isize; 3] = [-1, 1, 0];
    const LEFT_BACK: [isize; 3] = [-1, -1, 0];
    const LEFT_TOP: [isize; 3] = [-1, 0, 1];
    const LEFT_BOTTOM: [isize; 3] = [-1, 0, -1];

    const FRONT_TOP: [isize; 3] = [0, 1, 1];
    const FRONT_BOTTOM: [isize; 3] = [0, 1, -1];
    const BACK_TOP: [isize; 3] = [0, -1, 1];
    const BACK_BOTTOM: [isize; 3] = [0, -1, -1];

    const RIGHT_FRONT_TOP: [isize; 3] = [1, 1, 1];
    const RIGHT_FRONT_BOTTOM: [isize; 3] = [1, 1, -1];
    const RIGHT_BACK_TOP: [isize; 3] = [1, -1, 1];
    const RIGHT_BACK_BOTTOM: [isize; 3] = [1, -1, -1];

    const LEFT_FRONT_TOP: [isize; 3] = [-1, 1, 1];
    const LEFT_FRONT_BOTTOM: [isize; 3] = [-1, 1, -1];
    const LEFT_BACK_TOP: [isize; 3] = [-1, -1, 1];
    const LEFT_BACK_BOTTOM: [isize; 3] = [-1, -1, -1];
}



