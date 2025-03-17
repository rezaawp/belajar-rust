### 1. **Mutable**
   **Mutable** berarti dapat diubah atau dimodifikasi. Dalam konteks Rust, ketika sebuah variabel dideklarasikan dengan kata kunci `mut`, variabel tersebut menjadi mutable dan nilai yang disimpannya dapat diubah setelah deklarasi. Jika sebuah variabel tidak dideklarasikan dengan `mut`, maka variabel tersebut bersifat **immutable** (tidak bisa diubah).

   **Contoh:**
   ```rust
   let mut x = 5;  // x adalah mutable
   x = 10;          // nilai x dapat diubah
   ```

### 2. **Immutable**
   **Immutable** berarti tidak dapat diubah setelah nilai diberikan. Dalam Rust, variabel yang tidak dideklarasikan dengan `mut` bersifat immutable secara default. Artinya, kita tidak bisa mengubah nilai variabel tersebut setelah deklarasi.

   **Contoh:**
   ```rust
   let x = 5;  // x adalah immutable
   x = 10;      // Error: cannot assign twice to immutable variable `x`
   ```

### 3. **Memory di Stack dan Heap**
   Di Rust, memori dibagi menjadi dua area utama: **stack** dan **heap**.

   - **Stack** adalah area memori yang digunakan untuk menyimpan data dengan ukuran tetap dan yang dikelola secara otomatis oleh compiler. Data di stack memiliki masa hidup yang terbatas, sesuai dengan scope (lingkup) variabel.
   
   - **Heap** adalah area memori yang digunakan untuk menyimpan data dengan ukuran yang dinamis, yaitu yang ukurannya tidak diketahui saat kompilasi dan memerlukan alokasi manual atau otomatis melalui pengelolaan memori. Data di heap harus dibebaskan secara eksplisit setelah tidak digunakan lagi.

   **Contoh perbedaan:**
   - Variabel dengan tipe primitif atau referensi ke data kecil biasanya disimpan di stack.
   - Data yang lebih besar atau tidak diketahui ukurannya pada saat kompilasi (seperti vektor atau string panjang) disimpan di heap.

   **Stack vs Heap:**
   - **Stack**: Lebih cepat, memiliki ukuran terbatas, dan data akan otomatis dihapus saat keluar dari scope.
   - **Heap**: Lebih fleksibel, memungkinkan penyimpanan data besar, tetapi membutuhkan pengelolaan memori manual (misalnya, melalui ownership).

### 4. **String Slice dan Bukan String Slice**
   - **String Slice** adalah referensi ke bagian dari string (dalam bentuk `&str`). String slice lebih ringan karena tidak melakukan alokasi memori baru; hanya mengacu pada data string yang sudah ada.
   
     **Contoh String Slice:**
     ```rust
     let s = String::from("Hello, world!");
     let slice = &s[0..5];  // string slice: mengacu pada bagian "Hello"
     ```
   
   - **Bukan String Slice (String)** adalah tipe string yang dikelola sendiri (jenis `String`). Ini adalah tipe string yang dapat dimodifikasi, yang menyimpan data di heap.

     **Contoh String (Bukan String Slice):**
     ```rust
     let mut s = String::from("Hello");
     s.push_str(", world!");  // kita bisa mengubah String
     ```

   Intinya, **String** adalah tipe yang lebih kompleks yang memiliki kepemilikan data (ownership), sementara **&str** adalah referensi (slice) ke bagian dari string yang sudah ada.

### 5. **Ownership**
   **Ownership** adalah konsep yang sangat penting di Rust yang mengatur bagaimana memori dikelola secara otomatis. Setiap nilai di Rust memiliki **pemilik** (owner), dan hanya ada satu pemilik untuk setiap nilai pada satu waktu. Ketika pemilik nilai tersebut keluar dari scope, nilai tersebut akan dihapus secara otomatis untuk mencegah kebocoran memori.

   - **Transfer Ownership**: Saat kita memindahkan nilai (misalnya, memindahkan `String` ke fungsi lain), ownership berpindah dan kita tidak bisa lagi menggunakan nilai tersebut setelah dipindahkan.
   - **Borrowing**: Alih-alih mentransfer ownership, kita bisa **meminjam** nilai tersebut dengan referensi. Referensi bisa bersifat mutable atau immutable, tergantung pada apakah kita ingin mengubah nilai yang dipinjam.

   **Contoh Ownership:**
   ```rust
   fn main() {
       let s = String::from("Hello");
       let t = s; // ownership of s moved to t
       // println!("{}", s); // Error: s is no longer valid here
   }
   ```

   **Contoh Borrowing:**
   ```rust
   fn main() {
       let s = String::from("Hello");
       let t = &s; // borrow s (immutable reference)
       println!("{}", t); // ok, because t is just a reference to s
       // s can still be used here
   }
   ```

---

Semoga penjelasan ini membantu! Kamu bisa menambahkan penjelasan ini di README GitHub untuk memberi pemahaman dasar tentang istilah-istilah ini dalam Rust.