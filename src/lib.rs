use std::env;
use std::time::SystemTime;
use std::fs::File;
use std::thread;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::collections::HashMap;
use std::time;

use evdev_rs::Device;
use evdev_rs::TimeVal;
use evdev_rs::ReadFlag;
use evdev_rs::GrabMode;
use evdev_rs::InputEvent;
use evdev_rs::enums::EV_KEY;
use evdev_rs::enums::EV_SYN;
use evdev_rs::enums::EventCode;
use evdev_rs::uinput::UInputDevice;

pub type MacroMap = HashMap<EV_KEY, Box<dyn Fn(&InputEvent, &Sender<InputEvent>) -> bool>>;

fn dev_uinput_from_file(file: File) -> Result<(Device, UInputDevice), std::io::Error> {
    let dev = Device::new_from_file(file).unwrap();
    let uinput = UInputDevice::create_from_device(&dev).unwrap();

    return Ok((
        dev,
        uinput,
    ));
}

fn next_event(dev: &mut Device) -> Result<InputEvent, std::io::Error> {
    return dev.next_event(ReadFlag::NORMAL | ReadFlag::BLOCKING).map(|val| val.1);
}

pub fn new_key_event(ec: &EventCode, value: i32) -> InputEvent {
    let since_epoch = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let s = libc::timeval { tv_sec: since_epoch.as_secs() as i64, tv_usec: since_epoch.subsec_micros() as i64 };

    return InputEvent::new(&TimeVal::from_raw(&s), ec, value);
}

pub fn send_key(tx: &Sender<InputEvent>, key: EV_KEY, value: i32) {
    tx.send(crate::new_key_event(&EventCode::EV_KEY(key), value)).unwrap();
}

pub fn new_syn_event() -> InputEvent {
    let since_epoch = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let s = libc::timeval { tv_sec: since_epoch.as_secs() as i64, tv_usec: since_epoch.subsec_micros() as i64 };

    return InputEvent::new(&TimeVal::from_raw(&s), &EventCode::EV_SYN(EV_SYN::SYN_REPORT), 1);
}

pub fn send_syn(tx: &Sender<InputEvent>) {
    let since_epoch = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let s = libc::timeval { tv_sec: since_epoch.as_secs() as i64, tv_usec: since_epoch.subsec_micros() as i64 };

    tx.send(InputEvent::new(&TimeVal::from_raw(&s), &EventCode::EV_SYN(EV_SYN::SYN_REPORT), 1)).unwrap();
}

pub fn sleep(duration: u64)  {
    thread::sleep(time::Duration::from_millis(duration));
}

fn read_loop(dev: &mut Device, tx: Sender<InputEvent>, get_key_map: Box<dyn Fn() -> MacroMap>) {
    dev.grab(GrabMode::Grab).unwrap();

    let mut key_map = get_key_map();

    loop {
        let ev = next_event(dev);
        match ev {
            Ok(ev) => if handle_ev(ev, &mut key_map, &tx) { break; },
            Err(_e) => (),
        }
    }
}

fn handle_ev(ev: InputEvent, key_map: &mut MacroMap, tx: &Sender<InputEvent>) -> bool {
    if ev.is_code(&EventCode::EV_KEY(EV_KEY::KEY_ESC)) {
        println!("EXIT!!!");
        return true;
    }

    if let EventCode::EV_KEY(x) = ev.event_code {
        if let Some(f) = key_map.get_mut(&x) {
            return f(&ev, tx);
        }
    }

    tx.send(ev).unwrap();
    return false;
}

pub fn run(get_key_map: Box<dyn Fn() -> MacroMap>) {
    let file_name = env::var("DEV_PATH").expect("DEV_PATH not found on env variables!");

    let debug = match env::var("DEBUG") {
        Ok(val) => val == "1",
        Err(_) => false,
    };

    let file = File::open(file_name).unwrap();

    let (mut dev, uinput) = dev_uinput_from_file(file).unwrap();

    let (tx, rx): (Sender<InputEvent>, Receiver<InputEvent>) = mpsc::channel();

    let write_loop_thread = thread::spawn(move || {
        while let Ok(ev) = rx.recv() {
            if debug {
                if let EventCode::EV_KEY(k) = ev.event_code {
                    println!("{:?} {} {} {}", k, ev.value, ev.time.tv_sec, ev.time.tv_usec);
                }
            }
            
            uinput.write_event(&ev).ok();
        }
    });

    read_loop(&mut dev, tx, get_key_map);

    write_loop_thread.join().expect("panic!");
}
