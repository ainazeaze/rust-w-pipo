# Traits

> https://doc.rust-lang.org/stable/book/ch10-02-traits.html  
> https://doc.rust-lang.org/stable/std/keyword.trait.html  

Le concept de trait est important car il débloque plein de fonctionnalités. C'est un concept qui ressemble beaucoup aux interfaces dans les langages orienté objets qui permet de définir des méthodes ou types qui pourront être appelés une fois implémentées.
Il est possible d'implementer autant de traits qu'on veut sur un type.

# Règles

> https://doc.rust-lang.org/reference/items/implementations.html#orphan-rules  

On peut implementer :
- un trait interne avec type interne
- un trait externe avec un type interne
- un trait interne avec un type externe

MAIS on ne peut pas implementer un trait externe avec un type externe : c'est la "orphan rule". On verra comment contourner ce problème avec le pattern NewType.

# Méthode par défaut

> https://doc.rust-lang.org/stable/src/core/cmp.rs.html#251  

Il est possible d'implementer le corps de la méthode d'un trait ce qui la rend optionnelle lorsqu'on souhaite l'implementer. Par exemple, le trait `PartialEq` définit la méthode `eq` et `ne` qui est optionnelle car on peut juste renvoyer l'inverse de `eq`.

# Traits dans la librairie standard

## Debug

> https://doc.rust-lang.org/stable/std/fmt/trait.Debug.html  

Permet de formatter avec `{:?}`

## Display

> https://doc.rust-lang.org/stable/std/fmt/trait.Display.html  

Permet de formatter avec `{}`. Implemente `[ToString](https://doc.rust-lang.org/stable/std/string/trait.ToString.html)` automatiquement qui fournit la methode `to_string`.

## Copy

> https://doc.rust-lang.org/stable/std/marker/trait.Copy.html  

Marker qui désactive le move d'une valeur au profit d'une copie.

## Clone

> https://doc.rust-lang.org/stable/std/clone/trait.Clone.html  

Permet de cloner une valeur.

## Default

> https://doc.rust-lang.org/stable/std/default/trait.Default.html  

Fournit une valeur par défaut à un type.

## Drop

> https://doc.rust-lang.org/stable/std/ops/trait.Drop.html  

Spécifie une action lorsque la valeur est droppée.

## PartialEq

> https://doc.rust-lang.org/stable/std/cmp/trait.PartialEq.html  

Spécifie la comparaison entre deux valeurs. `PartialEq` est un trait avec un type générique avec par défaut le type qui l'implémente ce qui permet de pouvoir étendre les comparaisons entre deux types différents. Par exemple implementer `PartialEq<i64>` sur un `i32` permet de tester l'égalité sur un `i32` avec un `i64`.

## Iterator

> https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html  

Rend un type "Iterable". Beaucoup de fonctionnalité tournent autour de ce trait, par exemple si un type implemente `Iterator`, il peut être utilisé dans une boucle `for`.

# Derive

> https://doc.rust-lang.org/stable/book/appendix-03-derivable-traits.html  

Pour faciliter l'implementation de traits sur des types, certains traits peuvent être dérivés car ils sont faciles. Par exemple, on peut dériver `Default` sur un struct tant que tout les types qui la compose implémentent `Default`.

# Trait bounds

> https://doc.rust-lang.org/stable/book/ch10-02-traits.html#using-traits-as-parameters  

# Exemple

```rust
struct Number(i32);

trait Double: Sized {
    type Output: Double;

    fn double(&self) -> Self::Output;
    fn quad(&self) -> <Self::Output as Double>::Output {
        self.double().double()
    }
}

impl Double for Number {
    type Output = Self;

    fn double(&self) -> Self {
        Self(self.0 * 2)
    }
}

struct One;

impl Double for One {
    type Output = Number;

    fn double(&self) -> Number {
        Number(2)
    }
}

impl std::fmt::Debug for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Number").field(&self.0).finish()
    }
}

impl std::fmt::Debug for One {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("One").finish()
    }
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Display for One {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "1")
    }
}

impl std::ops::Add<Self> for Number {
    type Output = Number;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl std::ops::Add<i32> for Number {
    type Output = Number;

    fn add(self, rhs: i32) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl std::ops::Add<One> for Number {
    type Output = Number;

    fn add(self, _rhs: One) -> Self::Output {
        Self(self.0 + 1)
    }
}

impl Clone for Number {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

// impl Copy for Number {}

impl Default for Number {
    fn default() -> Self {
        Self(0)
    }
}

impl Drop for Number {
    fn drop(&mut self) {
        println!("{:?} is dropping", self);
    }
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

fn main() {
    let number = Number(10);
    println!("{:?}", number);
    let double = number.double();
    println!("{}", double);
    let quad = number.quad();
    println!("{}", quad);

    let num = Number(40) + Number(4) + 6;
    println!("{}", num);

    println!("{} {:?}", One, One);
}
```
