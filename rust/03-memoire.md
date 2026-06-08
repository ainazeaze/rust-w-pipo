# Stack vs. Heap

> https://doc.rust-lang.org/stable/book/ch04-01-what-is-ownership.html#the-stack-and-the-heap  
> https://doc.rust-lang.org/rust-by-example/std/box.html#box-stack-and-heap  
> https://martinlwx.github.io/en/what-is-the-heap-and-stack/  
> https://en.wikibooks.org/wiki/X86_Assembly/X86_Architecture  
> https://godbolt.org/z/qYWrda93f  

**Stack** (Pile) est un espace mémoire organisé où les valeurs sont stocké par défaut. Les valeurs doivent avoir une taille fixe connue par le compilateur.

**Heap** (Tas) est un espace mémoire moins organisé que la stack et plus lent qui permet d'allouer des espaces mémoires de manière dynamique.

# Problèmes mémoires

> https://en.wikipedia.org/wiki/Memory_safety  

Exemple de segfault en C :
```c
#include <stdio.h>
int main() {
  int *arr = NULL;
  printf("%d\n", arr[0]);
  
  return 0;
}
```

Exemple de fuite mémoire en C (vérifiable avec `valgrind`):
```c
#include <stdlib.h>

void leak() {
  int *arr = malloc(1024);
  // do something without freeing at the end
  // free(arr);
}

int main() {
  for (int i=0; i < 10; i++) {
    leak();
  }
  
  return 0;
}
```

Exemple de buffer overflow en C :
```c
#include <stdio.h>
int main() {
  int arr[] = {10, 20, 30, 40};
  for (int i = 0; i < 10; i++) {
    int *p = arr + i;
    printf("%d\n", *p);
  }
  return 0;
}
```

Exemple de dangling pointer en C :
```c
#include <stdio.h>
int main() {
  int *arr = malloc(4);
  arr[0] = 10;
  arr[1] = 20;
  arr[2] = 30;
  arr[3] = 40;
  free(arr);

  for (int i = 0; i < 10; i++) {
    int *p = arr + i;
    printf("%d\n", *p);
  }
  return 0;
}
```

Exemple de buffer overflow en Rust :
```rust
fn main() {
    let arr = [10, 20, 30, 40];
    let p = &arr as *const i32;
    unsafe {
        for i in 0..10 {
            let p2 = p.add(i);
            println!("{:?}", *p2);
        }
    }
}
```
