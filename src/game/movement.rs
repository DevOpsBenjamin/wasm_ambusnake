//use bevy::log::tracing_subscriber::fmt::time::SystemTime;

use std::time::Duration;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Dir 
{
    Left,
    Right,
    Up,
    Down
}

pub struct Move
{
    //DIRECTION
    pub dir: Dir,
    next_dir: Dir,
    //last_move_start: SystemTime,
}
impl Move {
    pub(crate) fn new() -> Move 
    {
        Move
        {
            dir: Dir::Right,        
            next_dir: Dir::Right
        }
    }

    pub(crate) fn reset(&mut self) {
        self.dir = Dir::Right;
        self.next_dir = Dir::Right;
    }    

    /*
    pub fn start(&mut self) {
        self. last_move_start = SystemTime::now();
    }

    pub fn check_reach(&mut self, move_duration: Duration)  -> bool
    {        
        let mut reach = false;
        let now = SystemTime::now();
        let curr_move_duration = now.duration_since(self.last_move_start).unwrap();

        if curr_move_duration > move_duration
        {
            //we reach BIG EVENT
            reach = true;
            self.compute_target();
            self.last_move_start = SystemTime::now();    
        }

        reach
    }

    pub(crate) fn try_add(&mut self, dir: Dir) {
        match self.dir {
            Dir::Left => 
            {   
                if dir == Dir::Up || dir == Dir::Down
                {
                    self.next_dir = dir;
                }
            },
            Dir::Right => 
            {
                if dir == Dir::Up || dir == Dir::Down
                {
                    self.next_dir = dir;
                }                
            },
            Dir::Up => 
            {
                if dir == Dir::Left || dir == Dir::Right
                {
                    self.next_dir = dir;
                }                
            },
            Dir::Down => 
            {
                if dir == Dir::Left || dir == Dir::Right
                {
                    self.next_dir = dir;
                }                
            }
        }
        
    }

    
    pub fn grow(&mut self) 
    {
        if self.body_part.is_empty()
        {
            self.body.start_shader_time();
        }
        self.body_part.push(Pos{ x:-1, y: -1});
    }
    
    pub fn draw(&mut self, ctx: &mut GraphicsContext) {
        self.head.rotate(ctx, get_rotation(self.dir));
        self.head.draw(ctx, self.pos);
        //SnakeDraw
        for b in &self.body_part
        {
            self.body.draw(ctx, *b);
        }
    }

    fn compute_target(&mut self) {
        self.dir = self.next_dir;

        //BODY TARGET
        let mut last_x = self.pos.x;
        let mut last_y = self.pos.y;
        match self.dir {
            Dir::Left => 
            {   
                self.pos.x -=  1;
            },
            Dir::Right => 
            {
                self.pos.x += 1;      
            },
            Dir::Up => 
            {
                self.pos.y += 1;           
            },
            Dir::Down => 
            {
                self.pos.y -= 1;        
            }
        }
        for b in self.body_part.iter_mut()
        {
            let curr_x = b.x;
            let curr_y = b.y;
            b.x = last_x;
            b.y = last_y;
            last_x = curr_x;
            last_y = curr_y;
        }
    }

    pub(crate) fn eat_himself(&self) -> bool {        
        for b in &self.body_part
        {
            if b.x == self.pos.x && b.y == self.pos.y
            {
                eprintln!("EAT TAIL");
                return true;
            }
        }
        false
    }    
    */
}
/*
fn get_rotation(dir: Dir) -> ROTATION {
    match dir {
        Dir::Left => ROTATION::Clockwise270,
        Dir::Right => ROTATION::Clockwise90,
        Dir::Up => ROTATION::None,
        Dir::Down => ROTATION::Clockwise180,
    }
} 
*/