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
   //تحويل الأنظمة الرقمية Base Conversion
   let num = 255;
   println!("{}",num); //العشري
   println!("{:b}",num); //الثنائي
   println!("{:o}",num); //الثماني
   println!("{:x}",num); //السداسي عشر
   // المحاذاة والمسافات (Alignment & Padding)
   let user1="Ali";
   let id1 = 1;
   let user2 = "Hussein";
   let id2 = 125;
   println!("User: {:<10} | ID: {:>5}",user1,id1);
   println!("User: {:<10} | ID: {:>5}",user2,id2);
   // التحقق الذكي (Argument Checking)
   //  هذا السطر سيسبب خطأ في المترجم (Compile Error)
   //  {1}لانك وضعت    
   // تتوقع متغيرا جديد لكنك اعطيت المترجم متغير واحد وهو في هذه الحالة حسين 
   // println!("Hello {0}, Wellcome to {1}","Hussein");
   // هذه الجملة صحيحة لاننا حددنا متغيريين اثنين
   println!("Hello {0}, Wellcome to {1}","Hussein","Ali");
   // pedding with zeros 
   let invoice_num = 42;
   // 0>5 تعني: اجعل العرض 5 واملأ الفراغ بالأصفار من اليسار
   println!("invoice_num: {:0>5}", invoice_num);

}
// this is a regular comment in Rust languge...
/*
this is a multi-line comment in Rust
that can include multibule line */
// using the cargo doc -- open will make you happy
 