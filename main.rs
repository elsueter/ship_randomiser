use rand::Rng;

#[derive(Debug)]
enum Weight_Class{
    light(f32),
    med(f32),
    heavy(f32)
}

fn get_weight(light_w: f32, med_w: f32, heavy_w: f32) -> Weight_Class {
    let mut rng = rand::thread_rng();
    match rng.gen_range(0..=2) {
        0 => Weight_Class::light(light_w),
        1 => Weight_Class::med(med_w),
        _ => Weight_Class::heavy(heavy_w),
    }
}

#[derive(Debug)]
enum Hull_Weapons{
    none,
    gattling_only,
    assault_cannons_no_autocannons,
    assault_cannons,
    artillery,
    anything
}

fn get_hull(min: i8, max: i8) -> Hull_Weapons {
    let mut rng = rand::thread_rng();
    match rng.gen_range(min..=max) {
        0 => Hull_Weapons::none,
        1 => Hull_Weapons::gattling_only,
        2 => Hull_Weapons::assault_cannons_no_autocannons,
        3 => Hull_Weapons::assault_cannons,
        4 => Hull_Weapons::artillery,
        _ => Hull_Weapons::anything,
    }
}


#[derive(Debug)]
enum Turret_Weapons{
    none,
    gattling_only,
    assault_cannons_no_autocannons,
    assault_cannons,
    artillery,
    anything
}

fn get_turret(min: i8, max: i8) -> Turret_Weapons {
    let mut rng = rand::thread_rng();
    match rng.gen_range(min..=max) {
        0 => Turret_Weapons::none,
        1 => Turret_Weapons::gattling_only,
        2 => Turret_Weapons::assault_cannons_no_autocannons,
        3 => Turret_Weapons::assault_cannons,
        4 => Turret_Weapons::artillery,
        _ => Turret_Weapons::anything,
    }
}


#[derive(Debug)]
enum Reactor_Restrictions{
    one_small,
    one_large,
    two_large,
    four_large,
    eight_large,
    twelve_large
}

#[derive(Debug)]
struct Ship{
    name: String,
    weight: Weight_Class,
    weapons: Hull_Weapons,
    turrets: Turret_Weapons,
    reactor: Reactor_Restrictions
}

fn generate_corvette() -> Ship{
    Ship{name:String::from("Corvette"), weight: get_weight(0.5, 0.75, 1.0), weapons: get_hull(1, 3), turrets: get_turret(0, 1), reactor: Reactor_Restrictions::one_small}
}

fn generate_frigate() -> Ship{
    Ship{name:String::from("Frigate"), weight: get_weight(1.0, 1.5, 2.0), weapons: get_hull(1, 4), turrets: get_turret(0, 2), reactor: Reactor_Restrictions::one_large}
}

fn generate_destroyer() -> Ship{
    Ship{name:String::from("Destroyer"), weight: get_weight(2.0, 3.5, 5.0), weapons: get_hull(0, 0), turrets: get_turret(2, 3), reactor: Reactor_Restrictions::two_large}
}

fn generate_ship() -> Ship{
    let mut rng = rand::thread_rng();
    match rng.gen_range(0..=2) {
        0 => generate_corvette(),
        1 => generate_frigate(),
        _ => generate_destroyer(),
    }
}


fn main(){
    let temp = generate_ship();
    println!("{:?}", temp);
}