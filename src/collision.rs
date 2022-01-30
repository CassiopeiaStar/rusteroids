use macroquad::prelude::*;

pub fn triangle_area(a: Vec2,b: Vec2,c: Vec2) -> f32 {
    (((a.x*(b.y-c.y))+(b.x*(c.y-a.y))+(c.x*(a.y-b.y)))/2.).abs()
}

pub fn point_triangle_collision(p: Vec2, a: Vec2, b: Vec2, c: Vec2) -> bool {
    let ta = triangle_area(a,b,c);
    let ca = triangle_area(p,a,b)+triangle_area(p,b,c)+triangle_area(p,a,c);
    //ta == ca
    (ta-ca).abs() < 0.1
}
