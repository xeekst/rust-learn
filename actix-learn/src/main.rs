// use actix::prelude::*;

// // this is our Message
// // we have to define the response type (rtype)
// #[derive(Message)]
// #[rtype(usize)]
// struct Sum(usize, usize);

// // Actor definition
// struct Calculator;

// impl Actor for Calculator {
//     type Context = Context<Self>;

//     fn started(&mut self, _ctx: &mut Self::Context) {
//         println!("I am alive!");
//     }
// }

// // now we need to implement `Handler` on `Calculator` for the `Sum` message.
// impl Handler<Sum> for Calculator {
//     type Result = usize; // <- Message response type

//     fn handle(&mut self, msg: Sum, _ctx: &mut Context<Self>) -> Self::Result {
//         msg.0 + msg.1
//     }
// }

// #[actix::main] // <- starts the system and block until future resolves
// async fn main() {
//     let addr = Calculator.start();
//     let res = addr.send(Sum(10, 5)).await; // <- send message and get future for result

//     match res {
//         Ok(result) => println!("SUM: {}", result),
//         _ => println!("Communication to the actor has failed"),
//     }
// }


// 2

use actix::prelude::*;
use std::time::Duration;

#[derive(Message)]
#[rtype(result = "()")]
struct Ping {
    pub id: usize,
}

// Actor definition
struct Game {
    counter: usize,
    name: String,
    recipient: Recipient<Ping>,
}

impl Actor for Game {
    type Context = Context<Game>;
}

// simple message handler for Ping message
impl Handler<Ping> for Game {
    type Result = ();

    fn handle(&mut self, msg: Ping, ctx: &mut Context<Self>) {
        self.counter += 1;

        if self.counter > 10 {
            System::current().stop();
        } else {
            println!("[{0}] Ping received {1}", self.name, msg.id);

            // wait 100 nanoseconds
            ctx.run_later(Duration::new(0, 100), move |act, _| {
                act.recipient.do_send(Ping { id: msg.id + 1 });
            });
        }
    }
}

fn main() {
    let system = System::new();

    system.block_on(async {
        // To create a cyclic game link, we need to use a different constructor
        // method to get access to its recipient before it starts.
        let _game = Game::create(|ctx| {
            // now we can get an address of the first actor and create the second actor
            let addr = ctx.address();

            let addr2 = Game {
                counter: 0,
                name: String::from("Game 2"),
                recipient: addr.recipient(),
            }
            .start();

            // let's start pings
            addr2.do_send(Ping { id: 10 });

            // now we can finally create first actor
            Game {
                counter: 0,
                name: String::from("Game 1"),
                recipient: addr2.recipient(),
            }
        });
    });

    // let the actors all run until they've shut themselves down
    system.run().unwrap();
}