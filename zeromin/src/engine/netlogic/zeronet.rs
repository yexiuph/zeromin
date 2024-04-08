use std::os::raw::c_ulong;

// Define constants for message types
pub const ZERO_MSG_BASE: c_ulong = 888;
pub const ZERO_MSG_LOGIN: c_ulong = ZERO_MSG_BASE + 450 as c_ulong;
pub const ZERO_MSG_LOBBY: c_ulong = ZERO_MSG_BASE + 950  as c_ulong;
pub const ZERO_MSG_LOBBY_MAX: c_ulong = ZERO_MSG_BASE + 1450 as c_ulong;
pub const ZERO_MSG_GAME_CONTROL: c_ulong = ZERO_MSG_BASE + 1900 as c_ulong;
pub const ZERO_MSG_GAME_CONTROL_MAX: c_ulong = ZERO_MSG_BASE + 3900 as c_ulong;

#[repr(C)]
pub struct ZeroMsgGeneric {
    dw_size: c_ulong,
    n_type: ZeroMsg,
}

#[repr(C)] 
pub enum ZeroMsg {
    ZeroMsgBaseZ = 0,

    // Server info message
    ZeroMsgRequestGameServer = Self::zero_msg_size(ZERO_MSG_BASE, 100),
    ZeroMsgSendGameServer =  Self::zero_msg_size(ZERO_MSG_BASE, 110),
}

impl ZeroMsg {
    const fn zero_msg_size(base: c_ulong, value: c_ulong) -> isize {
        (base + value) as isize
    }
}

#[no_mangle]
pub extern "C" fn construct_zero_msg(size: c_ulong, zero_msg_type: ZeroMsg) -> *mut ZeroMsgGeneric {
    let zero_msg = ZeroMsgGeneric {
        dw_size: size,
        n_type: zero_msg_type,
    };
    Box::into_raw(Box::new(zero_msg))
}

#[no_mangle]
pub extern "C" fn deconstruct_zero_msg(ptr: *mut ZeroMsgGeneric) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        Box::from_raw(ptr);
    }
}
