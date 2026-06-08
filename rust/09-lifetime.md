# Lifetime

> https://doc.rust-lang.org/stable/book/ch10-03-lifetime-syntax.html  
> https://github.com/cordx56/rustowl  

Désigne la durée de validité des references qui servent à s'assurer que les references ne dépassent pas le scope de leurs données.
Toutes les references ont une lifetime.
On utilise l'annotation de lifetime pour specifier la relation entre plusieurs lifetime.
Le but est d'éviter d'avoir des "dangling reference" (reference qui pointe vers une valeur libérée).

# Borrow Checker

> https://doc.rust-lang.org/stable/book/ch10-03-lifetime-syntax.html#the-borrow-checker  
> https://doc.rust-lang.org/rust-by-example/scope/lifetime.html#lifetimes  

# Elision

> https://doc.rust-lang.org/rust-by-example/scope/lifetime/elision.html  

Concept permettant de définir une lifetime implicite sur une lifetime.
Chaque paramètre de référence reçoit sa propre lifetime implicite.
Si il y a une seule lifetime en paramètre, la lifetime de retour (si il y en a une) est celle de cette référence.
Par default, dans les méthodes (avec self), la lifetime de retour est celle de self.
