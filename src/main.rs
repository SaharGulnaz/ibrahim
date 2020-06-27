#[derive(Debug)]
struct book{
  Name : string,
  Author : string,
  price :u16,
  availability : bool,
}


  fn main(){
    let book_1=book{
      name:string::from("book A"),
      Author:string::from("Author A"),
      price:500,
      avaiability:true
    };
  }
  