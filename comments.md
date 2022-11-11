### Comments

#### Armado del grafo

Al principio planteamos el objeto Node con dos nodos hijo del mismo tipo Node (aun sin pensar en referencias). Obviamente esto es erronero porque Rust no sabe a priori cuánto espacio va a ocupar el objeto Node. En C/C++ esto quizas compilaría y lanzaría un error de ejecución. En Rust, esto ni siquiera compila y el error se llama "infinite size". Es por eso que leyendo un poco la docu vemos que la solución de Rust para esto es usar Box<T>. Este tipo de datos en pocas palabras es un puntero (guarda una referencia al heap) con lo cual no es necesario saber a priori el tamaño que va a terminar teniendo mi objeto (en este caso un Nodo del grafo).

Ver ejemplo de una lista armada con Cons, que es un tipo de dato que guarda nested pairs y viene de Lisp programming (me suena a lo que vimos en clase)
Documentacion: https://doc.rust-lang.org/book/ch15-01-box.html

Algo interesante a destacar es el garbage collector de Rust. En Rust, no necesitamos desalocar manualmente la memoria ya que este tipo de dato implementa el trait "Drop" que desaloca la memoria una vez que el tipo de dato (en este caso Box) sale de scope. (No hay perdida de memoria)

#### Min Heap

Empezamos armando manualmente una lista enlazada que siempre apunte al minimo pero nos dimos cuenta que era menos performante y que Rust ya probee un objeto especial para eso: BinaryHeap<T>. Lo que pasa con este objeto es que por default es un MaxHeap, no un MinHeap como necesitamos. Para que sea MinHeap y además tome a T = Node (objeto custom que creamos nosotros), debemos implementar una serie de Traits. Estos traits le dicen a Rust cómo debe tratar este Binary Heap y cómo es la regla de ordenamiento. En este caso tuvimos que implementar dentro del objeto Node, los traits de Eq, Ord, PartialEq y PartialOrd para que al hacer pop() del heap siempre traiga el nodo con menor frecuencia.

Esto nos sirvio para mostrar un poco cómo funcionan los traits en Rust.