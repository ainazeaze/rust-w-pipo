# Généricité

> https://doc.rust-lang.org/stable/book/ch10-00-generics.html  

La généricité est très utilisée dans Rust car elle permet le monomorphisme et le polymorphisme. Cela peut s'appliquer aussi bien dans les structs, les enums ou les fonctions.
Deux exemples simples: `Option` et `Result`.

# Monomorphisme

> https://doc.rust-lang.org/book/ch10-01-syntax.html#performance-of-code-using-generics  
> https://oswalt.dev/2021/06/using-generic-types-in-rust/  

Le compilateur transforme les génériques en types concrets pour chaque type qui utilisé dans le type générique. Cela déplace le coût au moment de la compilation plutôt que pendant l'execution.
Cela s'appelle aussi "static dispatch".

Une autre manière de faire est d'utiliser `impl Trait` si on connait un trait qui doit être implémenté par le type qu'on souhaite utiliser.

```rust
fn print_debug(t: impl std::fmt::Debug) {
    println!("{t:?}");
}

fn do_double<T: Double>(t: T) -> T::Output {
    t.double()
}

fn print_double<T>(t: T)
where
    T: std::fmt::Display + Double,
    T::Output: std::fmt::Display,
    <T::Output as Double>::Output: std::fmt::Display,
{
    println!("double of {t} is {}", t.double());
    println!("quad of {t} is {}", t.quad());
}
```

# Polymorphisme

> https://doc.rust-lang.org/stable/book/ch18-02-trait-objects.html  
> https://doc.rust-lang.org/std/keyword.dyn.html  
> https://doc.rust-lang.org/rust-by-example/trait/dyn.html  
> https://oswalt.dev/2021/06/polymorphism-in-rust/  

Il est possible d’utiliser le mot-clé `dyn` pour créer un trait object : cela permet de manipuler à l’exécution des valeurs de types différents tant qu’elles implémentent un même trait. Dans ce cas, il est nécessaire de l'encapsuler car le compilateur ne connait pas la taille du type à l'avance.
Cela s'appelle aussi "dynamique dispatch".

Un exemple ou c'est utile est si on souhaite avoir une collection de type différents  :
```rust
trait Widget {
    fn draw(&self);
}

struct Button { label: String }
struct Label { text: String }
struct Slider { value: f32 }

impl Widget for Button {
    fn draw(&self) { println!("Button: {}", self.label); }
}
impl Widget for Label {
    fn draw(&self) { println!("Label: {}", self.text); }
}
impl Widget for Slider {
    fn draw(&self) { println!("Slider: {}", self.value); }
}

fn main() {
    let widgets: [Box<dyn Widget>; 3] = [
        Box::new(Button { label: "OK".into() }),
        Box::new(Label {
            text: "Hello".into(),
        }),
        Box::new(Slider { value: 0.5 }),
    ];

    for w in widgets.iter() {
        w.draw();
    }
}
```
