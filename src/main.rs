use macroquad::prelude::*;
use quad_rand::*;


mod collision;
use collision::*;


const PI: f32 = 3.14;


#[macroquad::main("Asteroids")]
async fn main() {
    loop{
        menu().await;
        let score = game().await;
        game_over(score).await;
    }
}
async fn menu() {
    loop {
        draw_text("Asteroids"               ,90. ,150.,100.,WHITE);
        draw_text("W to accellerate"        ,100.,250.,30.,WHITE);
        draw_text("S to stabelize velocity" ,100.,220.,30.,WHITE);
        draw_text("A and D to turn"         ,100.,290.,30.,WHITE);
        draw_text("Space to shoot"          ,100.,310.,30.,WHITE);
        draw_text("Press Space to begin"    ,90. ,350.,40.,WHITE);
        if is_key_pressed(KeyCode::Space) {
            next_frame().await;
            break;
        }
        next_frame().await
    }

}
async fn game_over(score: i32) {
    loop {
        draw_text("Game Over",100.,200.,100.,WHITE);
        draw_text(format!("Score: {}",score).as_str(),100.,300.,50.,WHITE);
        if is_key_pressed(KeyCode::Space) {
            next_frame().await;
            break;
        }
        next_frame().await
    }
}

async fn game() -> i32{
    let mut score = 0;
    let mut asteroid_count = 2;
    let mut asteroids = vec![];
    let mut player = Player::new();
    let mut jet_particles = Vec::new();
    let mut missiles = Vec::new();
    let mut game_over = false;
    loop {

        let dt = get_frame_time();
        
        player.update(dt,&mut jet_particles, &mut missiles);

        jet_particles.iter_mut().for_each(|p|p.update(dt));
        jet_particles = jet_particles.into_iter().filter(|p|p.fade > 0.).collect();

        missiles.iter_mut().for_each(|m|m.update(dt));
        missiles = missiles.into_iter().filter(|m|{
            !m.hit &&
            m.pos.x > -10. && m.pos.y > -10. &&
            m.pos.x < screen_width() + 10. &&
            m.pos.y < screen_height() + 10.
        }).collect();

        if asteroids.is_empty() {
            spawn_asteroids(asteroid_count,&mut asteroids);
            asteroid_count*=2;
        }

        asteroids.iter_mut().for_each(|a|{
            a.update(dt);
            if a.point_collision(player.pos) {
                game_over = true;
            }
        });
        let mut new_asteroids = Vec::new();
        asteroids = asteroids.into_iter().filter(|a|{
            for m in missiles.iter_mut() {
                if a.point_collision(m.pos) {
                    score += 1;
                    m.hit = true;
                    match a.size {
                        AsteroidSize::Large => {
                            new_asteroids.push(Asteroid::new(AsteroidSize::Medium,a.pos));
                            new_asteroids.push(Asteroid::new(AsteroidSize::Medium,a.pos));
                        }
                        AsteroidSize::Medium => {
                            new_asteroids.push(Asteroid::new(AsteroidSize::Small,a.pos));
                            new_asteroids.push(Asteroid::new(AsteroidSize::Small,a.pos));
                        }
                        AsteroidSize::Small => {}
                    }
                    return false;
                }
            };
            true
        }).collect();
        asteroids.append(&mut new_asteroids);
        
        
        clear_background(BLACK);
        draw_text(format!("Score: {}",score).as_str(),50.,50.,30.,WHITE);
        player.draw();
        asteroids.iter().for_each(|a|a.draw());
        jet_particles.iter().for_each(|p|p.draw());
        missiles.iter().for_each(|m|m.draw());

        next_frame().await;
        if game_over {
            break;
        }
    }
    score
}

fn spawn_asteroids(asteroid_count: i32, asteroids: &mut Vec<Asteroid>) {
    for _ in 0..asteroid_count {
        let mut x = gen_range(0.,screen_width());
        let mut y = gen_range(0.,screen_height());
        if gen_range(0.,1.) > 0.5 {
            if y > screen_height()/2. {
                y = screen_height() + AsteroidSize::Large.as_f32();
            } else {
                y = -AsteroidSize::Large.as_f32();
            }
        } else {
            if x > screen_width()/2. {
                x = screen_width() + AsteroidSize::Large.as_f32();
            } else {
                x = -AsteroidSize::Large.as_f32();
            }

        }
        asteroids.push(Asteroid::new(AsteroidSize::Large,vec2(x,y)));
    }
}

fn draw_shape(thickness: f32,color: Color,points: Vec<Vec2>) {
    if let Some(last) = points.last() {
        let mut previous = last.clone();
        for point in points.iter() {
            draw_line(point.x,point.y,previous.x,previous.y,thickness,color);
            previous = point.clone();
        }
    }
}

fn gen_asteroid() -> Vec<Vec2> {
    let mut points = Vec::new();

    let sides = 10;
    let radius = 10.;
    let rotation = PI*2./sides as f32;

    for _ in 0..sides {
        points.push(vec2(radius+gen_range(-3.,3.),0.));
        points = points.iter().map(|point|{
            Mat3::from_angle(rotation)
                .transform_point2(*point)
        }).collect();
    }
    points
}

enum AsteroidSize {
    Large,
    Medium,
    Small,
}

impl AsteroidSize {
    fn as_f32(&self) -> f32 {
        match self {
            AsteroidSize::Large => 10.,
            AsteroidSize::Medium => 5.,
            AsteroidSize::Small => 2.,
        }
    }
}

struct Asteroid {
    size: AsteroidSize,
    pos: Vec2,
    points: Vec<Vec2>,
    vel: Vec2,
    rotation_speed: f32,
    rotation: f32,

}

impl Asteroid {
    fn new(size: AsteroidSize, pos: Vec2) -> Self {
        let vel = Mat3::from_angle(gen_range(0.,PI*2.)).transform_point2(Vec2::ONE)*gen_range(50.,100.);
        let rotation_speed = gen_range(-1.,1.);
        Self {
            points: gen_asteroid(),
            size,
            pos,
            vel,
            rotation_speed,
            rotation: 0.,
        }
    }

    fn update(&mut self,dt: f32) {
        self.pos += self.vel*dt;
        self.rotation += self.rotation_speed*dt;

        if self.pos.x + (self.size.as_f32()*15.) < 0. {
            self.pos.x = screen_width() + (self.size.as_f32()*13.);
        }
        if self.pos.x - (self.size.as_f32()*15.) > screen_width() {
            self.pos.x = -(self.size.as_f32()*13.);
        }
        if self.pos.y + (self.size.as_f32()*15.) < 0. {
            self.pos.y = screen_height() + (self.size.as_f32()*13.);
        }
        if self.pos.y - (self.size.as_f32()*15.) > screen_height() {
            self.pos.y = -(self.size.as_f32()*13.);
        }       
    }

    fn draw(&self) {
        let shifted = self.points.iter().map(|point|{
            (
                Mat3::from_translation(self.pos)*
                Mat3::from_angle(self.rotation)*
                Mat3::from_scale(Vec2::ONE*self.size.as_f32())
            ).transform_point2(*point)
        }).collect();

        draw_shape(1.,WHITE,shifted);

        /*
        let triangles = self.inner_triangles();

        if self.point_collision(mouse_position().into()) {
            BLUE
        } else {
            WHITE
        };
        for t in triangles {
            let mut color = WHITE;
            if point_triangle_collision(mouse_position().into(),t.0,t.1,t.2) {
                color = BLUE;
            }
            draw_triangle_lines(t.0,t.1,t.2,1.,color);
        }
        */

    }

    fn inner_triangles(&self) -> Vec<(Vec2,Vec2,Vec2)> {
        let mut triangles = Vec::new();
        if let Some(last) = self.points.last() {
            let mut previous: Vec2 = last.clone();
            for tp in self.points.iter() {
                let triangle_points: Vec<Vec2> = vec![vec2(0.,0.),previous,*tp].iter().map(|p|{
                    (
                        Mat3::from_translation(self.pos)*
                        Mat3::from_angle(self.rotation)*
                        Mat3::from_scale(Vec2::ONE*self.size.as_f32())
                    ).transform_point2(*p)
                }).collect();

                previous = tp.clone();
                triangles.push((triangle_points[0],triangle_points[1],triangle_points[2]));
            }
        }
        

        triangles
    }

    fn point_collision(&self,point: Vec2) -> bool {
        for t in self.inner_triangles() {
            if point_triangle_collision(point,t.0,t.1,t.2) {
                return true;
            }
        }
        false
    }

}

struct Player {
    pos: Vec2,
    points: Vec<Vec2>,
    vel: Vec2,
    rotation: f32,
    jet_timer: f32,
    missile_timer: f32,
}


impl Player {
    fn new() -> Self {
        Self {
            pos: vec2(screen_width()/2.,screen_height()/2.),
            points: vec![
                vec2(0.,-10.),
                vec2(6.,10.),
                vec2(0.,2.),
                vec2(-6.,10.)
            ],
            vel: Vec2::ZERO,
            rotation: 0.,
            jet_timer: 0.1,
            missile_timer: 0.0,
        }
    }
    fn inner_triangles(&self) -> Vec<(Vec2,Vec2,Vec2)> {
        let mut triangles = Vec::new();
        if let Some(last) = self.points.last() {
            let mut previous: Vec2 = last.clone();
            for tp in self.points.iter() {
                let triangle_points: Vec<Vec2> = vec![vec2(0.,0.),previous,*tp].iter().map(|p|{
                    (
                        Mat3::from_translation(self.pos)*
                        Mat3::from_angle(self.rotation)
                    ).transform_point2(*p)
                }).collect();

                previous = tp.clone();
                triangles.push((triangle_points[0],triangle_points[1],triangle_points[2]));
            }
        }
        

        triangles
    }

    fn draw(&self) {
        let shifted = self.points.iter().map(|point|{
            (
                Mat3::from_translation(self.pos)*
                Mat3::from_angle(self.rotation)
            ).transform_point2(*point)
        }).collect();

        draw_shape(1.,WHITE,shifted);

        /*
        let triangles = self.inner_triangles();

        if self.point_collision(mouse_position().into()) {
            BLUE
        } else {
            WHITE
        };
        for t in triangles {
            let mut color = WHITE;
            if point_triangle_collision(mouse_position().into(),t.0,t.1,t.2) {
                color = BLUE;
            }
            draw_triangle(t.0,t.1,t.2,color);
        }
        */

    }

    fn facing(&self) -> Vec2 {
        Mat3::from_angle(self.rotation)
            .transform_point2(-Vec2::Y)
    }

    fn update(&mut self, dt: f32, jet_particles: &mut Vec<JetParticle>,missiles: &mut Vec<Missile>) {
        if is_key_down(KeyCode::A) {
            self.rotation -= dt*5.;
        }
        if is_key_down(KeyCode::D) {
            self.rotation += dt*5.;
        }
        if is_key_down(KeyCode::W) {
            self.vel += self.facing()*100.*dt;
            self.jet_timer -= dt;
            if self.jet_timer < 0. {
                jet_particles.push(JetParticle::new(
                    self.pos,
                    (Mat3::from_angle(PI+gen_range(-0.1,0.1))
                     .transform_point2(self.facing())*50.)+self.vel
                ));
                self.jet_timer = 0.1;
            }
        }

        if is_key_down(KeyCode::S) {
            self.vel = self.vel * 0.99;
        }

        self.missile_timer-=dt;
        if is_key_down(KeyCode::Space)&&self.missile_timer<0. {
            self.missile_timer = 0.3;
            missiles.push(Missile{
                pos: self.pos,
                vel: self.vel+(self.facing()*240.),
                hit: false
            });
        }

        self.pos += self.vel*dt;

        if self.pos.x + (20.) < 0. {
            self.pos.x = screen_width() + (18.);
        }
        if self.pos.x - (20.) > screen_width() {
            self.pos.x = -(18.);
        }
        if self.pos.y + (20.) < 0. {
            self.pos.y = screen_height() + (18.);
        }
        if self.pos.y - (20.) > screen_height() {
            self.pos.y = -(18.);
        }
    }
}

#[derive(Debug)]
struct JetParticle {
    size: f32,
    fade: f32,
    vel: Vec2,
    pos: Vec2,
}

impl JetParticle {
    fn new(pos: Vec2, vel: Vec2) -> Self {
        Self {
            size: 0.,
            fade: 1.,
            vel,
            pos,
        }
    }
    fn draw(&self) {
        let color = color_u8!(256,256,256,self.fade * 256.);
        draw_circle_lines(self.pos.x,self.pos.y,self.size,1.,color);
    }

    fn update(&mut self, dt: f32) {
        self.pos += self.vel * dt;
        self.size += dt*10.;
        self.fade -= dt;
    }
}

struct Missile {
    vel: Vec2,
    pos: Vec2,
    hit: bool,
}

impl Missile {
    fn draw(&self) {
        let rotation = -self.vel.angle_between(vec2(0.,-1.));
        let points = vec![
            vec2(0.,-1.),
            vec2(1.,0.),
            vec2(0.,5.),
            vec2(-1.,0.)
        ].iter().map(|p|{
            (
                Mat3::from_translation(self.pos)*
                Mat3::from_angle(rotation)*
                Mat3::from_scale(Vec2::ONE)
            ).transform_point2(*p)
        }).collect();
        draw_shape(1.,WHITE,points);
    }

    fn update(&mut self,dt: f32) {
        self.pos += self.vel*dt;
    }
}
