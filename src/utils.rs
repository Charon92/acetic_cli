extern crate rand;

const N: u8 = 8;

fn limits(x: u8, y: u8) -> bool {
    ((x >= 0 && y >= 0) && (x < N && y < N));
}

fn is_empty(x: u8, y: u8) -> bool {
    (limits(x, y)) && (a[y*N+x] < 0);
}

fn get_degree(a: [u8;16], x: u8, y: u8) {
    let count: u8 = 0;
    for i in 0..N {
        if isempty(a, (x + cx[i]), (y + cy[i])) {
            count += 1;
        }
    }

    return count;
}

fn next_move(a: [u8;16], &x: u8, &y: u8) {
    let min_deg_idx: u8 = -1;
    let min_deg = (N+1);
    let c: u8; let nx: u8; let ny: u8;
}

fn neighbour(x: u8, y: u8, xx: u8, yy: u8) -> bool {
    for i in 0..N {
        if ((x+cx[i]) == xx)&&((y + cy[i]) == yy) {
            return true;
        }
    }

    return false;
}

pub fn knights_tour() {
    let cx: [u8;N] = [1,1,2,2,-1,-1,-2,-2];
    let cy: [u8;N] = [2,-2,1,-1,2,-2,1,-1];
    let start: i32 = rand::random();

    for i in 0..N {
        let i = (start + count)%N;
        nx = *x + cx[i];
        ny = *y + cy[i];

        if ((isempty(a, nx, ny)) &&
            (c = getDegree(a, nx, ny)) < min_deg) {
                min_deg_idx = i;
                min_deg = c;
        }
    };

    if (min_deg_idx == -1) {
        return false;
    }

    nx = *x + cx[min_deg_idx];
    ny = *y + cy[min_deg_idx];

    // Mark next move
    a[ny*N + nx] = a[(*y)*N + (*x)]+1;

    // Update next point
    *x = nx;
    *y = ny;

    return true;
}
