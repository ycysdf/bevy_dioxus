use std::any::Any;
use std::ops::{Deref, DerefMut};

pub type ReturnValue = Box<dyn Any + Send + 'static>;

pub struct ApcMsg<S> {
    fun: Box<dyn FnOnce(S) -> ReturnValue + Send + 'static>,
    return_sender: Option<oneshot::Sender<ReturnValue>>,
}

pub fn call_with_return<S, R: Send + 'static>(
    sender: &ApcSender<S>,
    f: impl FnOnce(S) -> R + Send + 'static,
) -> Box<R> {
    let (return_sender, return_receiver) = oneshot::channel::<ReturnValue>();

    sender
        .send(ApcMsg {
            fun: Box::new(move |s| Box::new(f(s))),
            return_sender: Some(return_sender),
        })
        .unwrap();
    let result = return_receiver.recv().unwrap();
    result.downcast().unwrap()
}

pub fn call<S>(sender: &ApcSender<S>, f: impl FnOnce(S) + Send + 'static) {
    sender
        .send(ApcMsg {
            fun: Box::new(move |s| Box::new(f(s))),
            return_sender: None,
        })
        .unwrap();
}

pub struct ApcSender<S>(pub flume::Sender<ApcMsg<S>>);

impl<S> Clone for ApcSender<S> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<S> Deref for ApcSender<S> {
    type Target = flume::Sender<ApcMsg<S>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<S> DerefMut for ApcSender<S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct ApcReceiver<S>(pub flume::Receiver<ApcMsg<S>>);

impl<S> Clone for ApcReceiver<S> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<S> Deref for ApcReceiver<S> {
    type Target = flume::Receiver<ApcMsg<S>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<S> DerefMut for ApcReceiver<S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<S> ApcReceiver<S> {
    pub fn try_recv_all(&self, s: S) -> bool
    where
        S: Clone,
    {
        let mut r = false;
        while let Ok(msg) = self.try_recv() {
            Self::send_return(msg, s.clone());
            r = true;
        }
        r
    }

    pub fn send_return(msg: ApcMsg<S>, s: S) {
        let return_value = (msg.fun)(s);
        if let Some(return_sender) = msg.return_sender {
            return_sender.send(return_value).unwrap();
        }
    }
}

pub fn channel<S>() -> (ApcSender<S>, ApcReceiver<S>) {
    let (apc_sender, apc_receiver) = flume::unbounded();

    (ApcSender(apc_sender), ApcReceiver(apc_receiver))
}
