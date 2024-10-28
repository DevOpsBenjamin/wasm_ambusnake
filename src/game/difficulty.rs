use std::time::Duration;

#[derive(Clone, Copy)]
#[derive(Debug)]
pub enum DifficultyLevel 
{
    Easy,
    Medium,
    Hard,
    Insane
}

#[derive(Clone, Copy)]
pub struct Difficulty 
{
    move_duration: Duration,
    score_per_bonus:i32,
    bonus_count:i16,
    level:DifficultyLevel,
    next_level_trigger:i32
}

pub fn get_difficulty_data(level: DifficultyLevel) -> Difficulty {
    match  level {
        DifficultyLevel::Easy => Difficulty
        {
            move_duration: Duration::from_millis(400),
            score_per_bonus: 1,
            bonus_count: 4,
            level: DifficultyLevel::Easy,
            next_level_trigger:10
        },
        DifficultyLevel::Medium =>Difficulty
        {
            move_duration: Duration::from_millis(300),
            score_per_bonus: 4,
            bonus_count: 3,
            level: DifficultyLevel::Medium,
            next_level_trigger:50
        },
        DifficultyLevel::Hard => Difficulty
        {
            move_duration: Duration::from_millis(200),
            score_per_bonus: 6,
            bonus_count: 2,
            level: DifficultyLevel::Hard,
            next_level_trigger:100
        },
        DifficultyLevel::Insane => Difficulty
        {
            move_duration: Duration::from_millis(100),
            score_per_bonus: 10,
            bonus_count: 1,
            level: DifficultyLevel::Insane,
            next_level_trigger:9999999
        },
    }
}