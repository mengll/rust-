### trait 切入的相互的实现
```
trait Shape {
    fn area(&self) ->f64;
    fn jump(&self);
}

trait Round {
    fn get_radius(&self) ->f64;
}

struct Circle {
    radius:f64,
}

impl Round for Circle {
    fn get_radius(&self)->f64{
        self.radius
    }
}

impl  Shape for Round {
    fn area(&self)->f64{
        std::f64::consts::PI * self.get_radius() * self.get_radius()                                                       
    }

    fn jump(&self){
        println!("{}","jumper")
    }
}

fn main(){
    let b = Box::new(Circle{radius:4f64}) as Box<Round>; // 智能指针，数据存放到堆上
    let area  = b.area();
    println!("面积：{}",area);
    b.jump();
}

```
