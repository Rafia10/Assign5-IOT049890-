// use actix_web::{web, App, HttpResponse, HttpServer, Responder};

// async fn index() -> impl Responder {
//     HttpResponse::Ok().body("Hello world welcome to quarter 3!")
// }

// async fn index2() -> impl Responder {
//     HttpResponse::Ok().body("Hello world again!")
// }
// #[actix_rt::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .route("/", web::get().to(index))
//             .route("/again", web::get().to(index2))
//     })
//     .bind("127.0.0.1:8088")?
//     .run()
//     .await
// }
extern crate actix;
extern crate futures;
use futures::{future, Future};
use actix::*;

struct Sum(usize, usize);


impl ResponseType for Sum {
    type Item = usize;
    type Error = ();
}

//  definition
struct Addit;

impl Actor for Addit {
    type Context = Context<Self>;
}

// now we need to define `MessageHandler` for `Sum` message.
impl Handler<Sum> for Addit {

    fn handle(&mut self, msg: Sum, ctx /*Define context type*/: &mut Context<Self>) -> Response<Self, Sum> {
        Self::reply(msg.0 + msg.1)
    }
}

fn main() {
    let system = System::new("ADDIT APPLICATION");

    let addr: Address<_> = Addit.start();
    let res = addr.call_fut(Sum(25, 5));//here we are calling future
    
    system.handle().spawn(res.then(|res| {
        match res {
            Ok(Ok(result)) => println!("SUM OF GIVEN NUMBERS IS: {}", result),
            _ => println!("Something wrong"),
        }
        
        Arbiter::system().send(msgs::SystemExit(0)); //Arbiter controls event loop in it's thread. Each arbiter runs in separate thread.
        future::result(Ok(()))
    }));

    system.run();
}