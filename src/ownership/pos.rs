use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Message{
    to: u64,
    connect: String
}

#[derive(Debug)]
struct MailBox {
    messages:  Vec<Message>
}

impl MailBox {
    fn post(&mut self, msg: Message) {
        self.messages.push(msg)
    }
    fn deliver(&mut self, recipient: &CubeSat) -> Option<Message>{
        for i in 0..self.messages.len(){
            if self.messages[i].to == recipient.id {
                let msg = self.messages.remove(i);
                return Some(msg)
            }
        }
        None
    }
}
#[derive(Debug)]
struct CubeSat {
    id: u64,
    mailbox: MailBox
}

#[derive(Debug)]
pub enum StatusMessage{
    Ok
}
#[derive(Debug)]
struct GroundStation {
    radio_freq: f64
}

// impl GroundStation {
//     pub fn send(&self, mailbox: &mut MailBox, msg: Message) {
//         mailbox.post(msg)
//     }
//     pub fn connect(&self, sat_id: u64)->CubeSat{
//         CubeSat {
//             id: sat_id,
//             mailbox: MailBox { messages: vec![]},
//         }
//     }
// }

impl CubeSat{
    pub fn recv(&mut self,mailbox: &mut MailBox) -> Option<Message> {
        mailbox.deliver(&self)
    }
}

fn check_status(sat_id: CubeSat)->CubeSat{
    println!("{:?} {:?}", sat_id, StatusMessage::Ok);
    sat_id
}

fn fetch_sat_ids() -> Vec<u64>{
    vec![1,2,3]
}

pub fn pos(){
    // let base = GroundStation;
    // let ids = fetch_sat_ids();
    // for id in ids {
    //     let mut sat = base.connect(id);
    //     base.send(&mut sat, Message::from("hello there"));
    // }
    let mut mail = MailBox {
        messages: vec![],
    };
    let base: Rc<RefCell<GroundStation>> = Rc::new(RefCell::new(GroundStation{
        radio_freq: 87.65
    }));
    println!("base {:?}", base);
    {
        let mut base_2 = base.borrow_mut();
        base_2.radio_freq -=12.34;
        println!("base 2 {:?}", base_2);
    }
    // let ids = fetch_sat_ids();
    // for id in ids{
    //     let msg = Message {
    //         to: id,
    //         connect: String::from("hello"),
    //     };
    //     base.send(&mut mail, msg);
    // }
    // let ids = fetch_sat_ids();
    // for id in ids{
    //     let mut sat = base.connect(id);
    //     let msg = sat.recv(&mut mail);
    //     println!("{:?} {:?}", sat, msg);
    // }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_pos(){
        pos();
    }
}