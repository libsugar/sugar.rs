# batch_oper
batch_oper provides some batch operation macro for some operations
## Usage
- **Basic**  
  - batch `||`  
    ```rust  
    bop!(|| 4; == 2, > 3);
    ```
    equivalent to
    ```rust
    4 == 2 || 4 > 3
    ```
  - batch `&&`  
    ```rust  
    bop!(&& 4; == 2, > 3);
    ```
    equivalent to
    ```rust
    4 == 2 && 4 > 3
    ```
  - `!`
    ```rust
    bop!(|| a; == 1;!, == 2);
    ```
    equivalent to
    ```rust
    1 == a || a == 2
    ```
- **Set**
  ```rust
  let mut a = 1;
  bop!(= a; + 1, - 2;!, * 3);
  ```
  equivalent to
  ```rust
  let mut a = 1;
  a = a + 1;
  a = 2 - a;
  a = a * 3;
  ```
- **Let**
  ```rust
  bop! { let a|u8 = 1, b = 2 }
  ```
  equivalent to
  ```rust
  let a: u8 = 1;
  let b = 2;
  ```