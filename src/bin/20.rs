use glam::IVec3;

advent_of_code::solution!(20);

#[derive(Debug, Clone)]
struct Particle {
    position: IVec3,
    velocity: IVec3,
    acceleration: IVec3,
}

impl Particle {
    fn new(position: IVec3, velocity: IVec3, acceleration: IVec3) -> Self {
        Self {
            position,
            velocity,
            acceleration,
        }
    }
}

fn parse_ivec3(input: &str) -> Option<IVec3> {
    let pos_elems = input.split(',').collect::<Vec<_>>();
    Some(IVec3::new(
        pos_elems[0].parse().ok()?,
        pos_elems[1].parse().ok()?,
        pos_elems[2].parse().ok()?,
    ))
}

fn parse_particle(line: &str) -> Option<Particle> {
    let particle_data = line.split(", ").collect::<Vec<_>>();

    let position = parse_ivec3(&particle_data[0][3..particle_data[0].len() - 1])?;
    let velocity = parse_ivec3(&particle_data[1][3..particle_data[1].len() - 1])?;
    let acceleration = parse_ivec3(&particle_data[2][3..particle_data[2].len() - 1])?;

    Some(Particle::new(position, velocity, acceleration))
}

fn tick(particles: Vec<Particle>) -> Vec<Particle> {
    let mut particles = particles;

    for particle in particles.iter_mut() {
        particle.velocity += particle.acceleration;
        particle.position += particle.velocity;
    }

    let mut retain = Vec::new();

    for particle in particles.iter() {
        if particles
            .iter()
            .filter(|p| p.position == particle.position)
            .count()
            == 1
        {
            retain.push(particle.clone());
        }
    }

    retain
}

pub fn part_one(input: &str) -> Option<usize> {
    let particles = input.lines().filter_map(parse_particle).collect::<Vec<_>>();

    let (idx, _) = particles.iter().enumerate().min_by(|(_, a), (_, b)| {
        let a_len = a.acceleration.length_squared();
        let b_len = b.acceleration.length_squared();
        a_len.cmp(&b_len)
    })?;

    Some(idx)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut particles = input.lines().filter_map(parse_particle).collect::<Vec<_>>();

    for i in 0.. {
        particles = tick(particles);

        if particles.len() == 1 || i == 40 {
            break;
        }
    }

    Some(particles.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(1));
    }
}
