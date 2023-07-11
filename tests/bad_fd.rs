use calloop::{PostAction, Readiness, Token, TokenFactory};
use std::os::unix::io::RawFd;

struct BadFd(RawFd);

impl calloop::EventSource for BadFd {
    type Event = calloop::Readiness;
    type Metadata = ();
    type Ret = ();
    type Error = std::io::Error;

    fn process_events<F>(&mut self, _: Readiness, _: Token, _: F) -> Result<PostAction, std::io::Error> { todo!() }

    fn register(&mut self, _: &mut calloop::Poll, _: &mut TokenFactory) -> Result<(), calloop::Error> { todo!() }

    fn reregister(&mut self, _: &mut calloop::Poll, _: &mut TokenFactory) -> Result<(), calloop::Error> { todo!() }

    fn unregister(&mut self, _: &mut calloop::Poll) -> Result<(), calloop::Error> { todo!() }
}

fn bad_fd() {
    let event_loop = calloop::EventLoop::try_new().unwrap();
}
