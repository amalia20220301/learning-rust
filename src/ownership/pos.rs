
type Message=String;

#[derive(Debug)]
struct MailBox {
    messages:  Vec<Message>
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
struct GroundStation;

impl GroundStation {
    pub fn send(&self, to: &mut CubeSat, msg: Message) {
        to.mailbox.messages.push(msg)
    }
}

impl CubeSat{
    pub fn recv(&mut self) -> Option<Message> {
        self.mailbox.messages.pop()
    }
}
fn check_status(sat_id: CubeSat)->CubeSat{
    println!("{:?} {:?}", sat_id, StatusMessage::Ok);
    sat_id
}
pub fn pos(){
    let base = GroundStation;
    let mut sat_a=CubeSat{
        id: 0,
        mailbox: MailBox{
            messages: vec![]
        }
    };
    println!("t0 {:?}", sat_a);
    base.send(&mut sat_a, Message::from("hello, there"));
    println!("t1 {:?}", sat_a);
    let msg = sat_a.recv();
    println!("t1 {:?}", sat_a);
    println!("message {:?}", msg);
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_pos(){
        pos();
    }
}