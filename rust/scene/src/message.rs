use std::collections::VecDeque;

pub struct MessageManager {
    pub queue: VecDeque<String>,
}

static mut G_MANAGER: Option<Box<MessageManager>> = None;

pub fn get() -> Option<&'static mut Box<MessageManager>> {
    unsafe {
        if G_MANAGER.is_none() {
            G_MANAGER = Some(Box::new(MessageManager {
                queue: VecDeque::new(),
            }))
        }
    }

    if let Some(message_manager) = unsafe { &mut G_MANAGER } {
        Some(message_manager)
    } else {
        None
    }
}
