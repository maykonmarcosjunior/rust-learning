pub fn main() {
    println!("--------- Property File ----------");
    if false {
        println!("--------- Borrowing ----------");
        borrowing_test1();
        borrowing_test2();
    }
    if false {
        println!("--------- Pointers ----------");
        pointers1();
        pointers2();
    }
    if true {
        println!("--------- References ----------");
        mutable_reference();
        imutable_reference();
        lifetime_reference();
        ascii_capitalize(&mut vec!['a', 'b', 'c']);
        // é necessário atribuir a uma variável,
        // porque a função retorna uma referência
        let vec_str = vec!["Hello".to_string(), "World".to_string()];
        let st = first(&vec_str);
        println!("First string is {}", st);
    }
}

fn first(strings: &Vec<String>) -> &String {
    let s_ref = &strings[0]; // s_ref tem permissão F agora
    // se &strings[0] fosse retornado diretamente, o compilador
    // removeria todas as permissões de strings, porque não seria
    // possível saber se strings seria usada de novo
    s_ref
    // Sem s_ref, haveria a possibilidade de strings ser desalocado
    // gerando um dangling pointer
}

fn mutable_reference() {
    println!("\n--> Mutable Reference");
    let mut v: Vec<i32> = vec![1, 2, 3];    // v tem permissões ROW
    println!("Vector is {:?}", v);
    let num: &i32 = &v[2];    
    // v tem permissão R, num tem RO (não tem W porque não é mut)
    // *num tem permissão R, porque num é RO
    // é importante que *num só tenha permissão de leitura,
    // porque significa que, embora _num_ possa mudar seu valor
    // (ou seja, se referir a outro endereço), não pode mudar o valor de v[2].
    println!("Third element (num) is {}", *num);
    // A partir daqui, o compilador percebe que num não será mais usada, 
    // então remove todas as permissões de num e *num
    v.push(4); // aqui, o compilador também remove as permissões de v.
    println!("Vector is now {:?}", v);

}

fn imutable_reference() {
    println!("\n--> Imutable Reference");
    let mut v: Vec<i32> = vec![1, 2, 3];
    println!("Vector is {:?}", v);
    let num: &mut i32 = &mut v[2];
    println!("Third element is {}", *num);
    // agora é uma referência mutável, então num tem permissões RO
    // *num ganha permissão RW e v agora não tem permissão nenhuma
    //  de leitura, já que pode ser alterado por *num
    
    *num += 1; // num não é imutável, não pode apontar para outro endereço
    println!("Third element is now {}", *num);
    let num2: &i32 = &*num;
    // *num perde temporariamente a permissão W, e num a O
    // num2 ganha permissão WO, e *num2 ganha permissão R
    println!("num = {}, num2 = {}", *num, *num2); // ambos podem ser lidos
    println!("Vector is now {:?}", v);      // vai funcionar porque num não será usado de novo
}

fn lifetime_reference() {
    println!("\n--> Reference Lifetime");
    let mut x = 1; // lifetime of x starts here, with ROW permissions
    println!("x = {x}");
    let y = &x; // lifetime of y starts here, and x looses its WO permissions
    println!("y = {y}"); // RO permissions for y and R for *y
    let z = *y; // lifetime of z starts here, and y's ends here
    println!("z = {z}"); // x has its WO permissions back, and y is gone
    x += z; // z is bonded solely to the former value of y, with RO permissions
    println!("x = {x}"); // z is not bonded to x, so it can be modified and read
}

fn ascii_capitalize(v: &mut Vec<char>) {
    println!("\n--> ASCII Capitalize");
    // v has RO permissions, *v has RW permissions
    println!("Vector is {:?}", v); // porque v não pode apontar para outro endereço
    let c = &v[0]; // v perde a permissão O e *v a permissão W
    println!("First element is {}", c); // c tem RO
    println!("First element is {}", *c); // *c tem R
    println!("First element is {}", &c);
    if c.is_ascii_lowercase() { // nesse branch, c e *c perdem todas as permissões
        let up = c.to_ascii_uppercase(); // v recupera a permissão O e *v a W
        v[0] = up; // up tinha permissão RO, mas a perde aqui (não é usada de novo)
    }
    println!("Vector is now {:?}", v);
}

fn pointers1() {
    let mut x: Box<i32> = Box::new(1);
    let a: i32 = *x;         // *x reads the heap value, so a = 1
    *x += 1;                 // *x on the left-side modifies the heap value,
                             //     so x points to the value 2

    let r1: &Box<i32> = &x;  // r1 points to x on the stack
    let b: i32 = **r1;       // two dereferences get us to the heap value

    let r2: &i32 = &*x;      // r2 points to the heap value directly
    let c: i32 = *r2;        // so only one dereference is needed to read it
    println!("a = {a}, b = {b}, c = {c}");

}

fn pointers2() {
    let x: Box<i32> = Box::new(-1);
    let x_abs1 = i32::abs(*x); // explicit dereference
    let x_abs2 = x.abs();          // implicit dereference
    assert_eq!(x_abs1, x_abs2);
    println!("x_abs1 = {x_abs1}, x_abs2 = {x_abs2}");

    let r: &Box<i32> = &x;
    let r_abs1 = i32::abs(**r); // explicit dereference (twice)
    let r_abs2 = r.abs();           // implicit dereference (twice)
    assert_eq!(r_abs1, r_abs2);
    println!("r_abs1 = {r_abs1}, r_abs2 = {r_abs2}");

    let q = Box::new(&x);
    let q_abs1 = i32::abs(***q); // explicit dereference (three times)
    let q_abs2 = q.abs();           // implicit dereference (three times)
    assert_eq!(q_abs1, q_abs2);
    println!("q_abs1 = {q_abs1}, q_abs2 = {q_abs2}");

    let s = String::from("Hello");
    let s_len1 = str::len(&s); // explicit reference
    let s_len2 = s.len();          // implicit reference
    assert_eq!(s_len1, s_len2);
    println!("s_len1 = {s_len1}, s_len2 = {s_len2}");
}

fn borrowing_test1() {
    let first = String::from("Ferris");
    let full = add_suffix(first);
    let new_full = full;
    let bb = false;
    let mut new_new_full: String = "Preview".to_string();
    if bb {
        new_new_full = new_full;
        // mesmo que não acontecesse a atribuição,
        // a propriedade de new_full seria movida para new_new_full
    }
    // println!("{full}");
    println!("new_new_full = {new_new_full}");    
}

fn borrowing_test2() {
    let second: &str = "Ferris";
    let third = second;
    // println!("{first}");
    println!("{second} --> {third}");
}

fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr.");
    name
}
