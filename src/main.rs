//! هذا تعليق يشرح الوحدة بالكامل
/// هنا نخبر راست ان يبحث عن ملف بهذا الاسم وينفذه
mod modles; 
/// this is the main function
fn main(){
    println!("hello world!");
    println!("I'm a Rustacen!");
//  the place holder in Rust is {} and it gonaa print whatever after the ,
    println!("{} days",31);
    // Positional Arguments :
    // يمكنك تحديد اي متغير يذهب لاي قوس بأستخدام الارقام التي تبدأ من 0
   println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
   // الأسماء (Named Arguments) : نفس لغة بايثون
   // حيث يمكنك اعطاء اسماء للمتغيرات لتسهيل قرائتها
   println!("{subject} {verb}", subject="the fox", verb="jumps");
   //
}
// this is a regular comment in Rust languge...
/*
this is a multi-line comment in Rust
that can include multibule line */
// using the cargo doc -- open will make you happy