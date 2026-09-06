trait Vehicle: Paint {
    fn park(&self);
}

trait Paint {
    fn paint(&self, color: String) {
        println!("Painting the vehicle with color: {}", color);
    }
}

struct VehicleInfo {
    make: String,
    model: String,
    year: u32,
}

impl Vehicle for Car {
    fn park(&self) {
        println!("Car parked");
    }
}

impl Vehicle for Truck {
    fn park(&self) {
        println!("Truck parked");
    }
}

struct Car {
    info: VehicleInfo,
}

struct Truck {
    info: VehicleInfo,
}

impl Truck {
    fn upload(&self) {
        println!("Truck uploaded");
    }
}

impl Paint for Truck {}

impl Paint for Car {}

struct House {}

impl Paint for House {
    fn paint(&self, color: String) {
        println!("Painting the house with color: {}", color);
    }
}

fn main() {
    let car = Car {
        info: VehicleInfo {
            make: "Toyota".into(),
            model: "Camry".into(),
            year: 2020,
        },
    };
    paint_vehicle(&car);
}

fn paint_red<T: Paint>(object: &T) {
    object.paint("red".into());
}

fn paint_blue<T: Paint>(object: &impl Paint) {
    object.paint("blue".into());
}

fn paint_vehicle<T>(object: &T)
where
    T: Paint,
{
    object.paint("puple".into());
}

fn create_paintable_object(vehicle: bool) -> Box<dyn Paint> {
    //静态调度是编译器知道调用哪些具体方法之一
    if vehicle {
        Box::new(Car {
            info: VehicleInfo {
                make: "Toyota".into(),
                model: "Camry".into(),
                year: 2020,
            },
        })
    } else {
        Box::new(House {})
    }
}
