use bluer::gatt::local::{
    Characteristic, CharacteristicNotify, CharacteristicNotifyMethod, CharacteristicRead,
    CharacteristicWrite, CharacteristicWriteMethod, Service,
};
use futures_util::FutureExt;
use std::time::Duration;
use tokio::time::interval;

pub const CONTROLLER_UUID: &str = "08590f7e-db05-467e-8757-72f6faeb13a5";
const JS_DATA_UUID: &str = "08590f7e-db05-467e-8757-72f6faeb13b5";
const JS_RATE_UUID: &str = "08590f7e-db05-467e-8757-72f6faeb13c5";

const BTN_EU: u8 = 0x08;
const BTN_EL: u8 = 0x02;
const BTN_FU: u8 = 0x04;
const BTN_FL: u8 = 0x01;
const BTN_LU: u8 = 0x20;
const BTN_LL: u8 = 0x10;
const BTN_RU: u8 = 0x80;
const BTN_RL: u8 = 0x40;

#[derive(Debug)]
struct ControllerData {
    j1_y: u8,
    j1_x: u8,
    j2_y: u8,
    j2_x: u8,
    buttons: u8,
    battery: u8,
    main_pwr: u8,
    idle_time: u8,
    pwr_off_delay: u8,
    cont_count: u8,
    unused: u8,
    unused1: u8,
    unused2: u8,
    unused3: u8,
}

impl Default for ControllerData {
    fn default() -> Self {
        Self {
            j1_y: 127,
            j1_x: 127,
            j2_y: 127,
            j2_x: 127,
            buttons: 0,
            battery: 127,
            main_pwr: 255,
            idle_time: 0,
            pwr_off_delay: 0,
            cont_count: 0,
            unused: 0,
            unused1: 0,
            unused2: 0,
            unused3: 0,
        }
    }
}

struct ButtonState {
    left_trigger_upper: bool,
    left_trigger_lower: bool,
    right_trigger_upper: bool,
    right_trigger_lower: bool,
    left_button_upper: bool,
    left_button_lower: bool,
    right_button_upper: bool,
    right_button_lower: bool,
}

impl Default for ButtonState {
    fn default() -> Self {
        Self {
            left_trigger_upper: false,
            left_trigger_lower: false,
            right_trigger_upper: false,
            right_trigger_lower: false,
            left_button_upper: false,
            left_button_lower: false,
            right_button_upper: false,
            right_button_lower: false,
        }
    }
}

fn get_button_value(state: ButtonState) -> u8 {
    let mut value = 0;

    if state.left_trigger_upper {
        value |= BTN_LU;
    }
    if state.left_trigger_lower {
        value |= BTN_LL;
    }
    if state.right_trigger_upper {
        value |= BTN_RU;
    }
    if state.right_trigger_lower {
        value |= BTN_RL;
    }
    if state.left_button_upper {
        value |= BTN_EU;
    }
    if state.left_button_lower {
        value |= BTN_EL;
    }
    if state.right_button_upper {
        value |= BTN_FU;
    }
    if state.right_button_lower {
        value |= BTN_FL;
    }

    return value;
}

fn serialize_controller_data(data: ControllerData) -> Vec<u8> {
    let mut buffer = Vec::new();
    buffer.push(data.j1_y);
    buffer.push(data.j1_x);
    buffer.push(data.j2_y);
    buffer.push(data.j2_x);
    buffer.push(data.buttons);
    buffer.push(data.battery);
    buffer.push(data.main_pwr);
    buffer.push(data.idle_time);
    buffer.push(data.pwr_off_delay);
    buffer.push(data.cont_count);
    buffer.push(data.unused);
    buffer.push(data.unused1);
    buffer.push(data.unused2);
    buffer.push(data.unused3);
    return buffer;
}
pub fn make_controller_service() -> bluer::gatt::local::Service {
    return Service {
        uuid: CONTROLLER_UUID.parse().unwrap(),
        primary: true,
        characteristics: vec![
            Characteristic {
                uuid: JS_DATA_UUID.parse().unwrap(),
                notify: Some(CharacteristicNotify {
                    indicate: true,
                    notify: true,
                    method: CharacteristicNotifyMethod::Fun(Box::new(move |mut notifier| {
                        async move {
                            tokio::spawn(async move {
                                println!(
                                    "Notification session start with confirming={:?}",
                                    notifier.confirming()
                                );
                                let mut interval = interval(Duration::from_millis(2));
                                let mut count = 0;
                                loop {
                                    {
                                        let value = ControllerData {
                                            buttons: get_button_value(ButtonState::default()),
                                            cont_count: count,
                                            j1_y: 255,
                                            ..Default::default()
                                        };
                                        println!("Notifying with value {:x?}", value);
                                        if let Err(err) =
                                            notifier.notify(serialize_controller_data(value)).await
                                        {
                                            println!("Notification error: {}", &err);
                                            break;
                                        }
                                    }
                                    // >= instead of == incase of bit-flip
                                    if count >= 255 {
                                        count = 0;
                                    } else {
                                        count += 1;
                                    }
                                    interval.tick().await;
                                }
                                println!("Notification session stop");
                            });
                        }
                        .boxed()
                    })),

                    ..Default::default()
                }),
                read: Some(CharacteristicRead {
                    read: true,
                    fun: Box::new(move |req| {
                        // let value = value_read.clone();
                        async move {
                            // let value = value.lock().await.clone();
                            println!("Read request {:?}", &req);
                            Ok(vec![])
                        }
                        .boxed()
                    }),
                    ..Default::default()
                }),
                write: Some(CharacteristicWrite {
                    write: true,
                    write_without_response: true,
                    method: CharacteristicWriteMethod::Fun(Box::new(move |new_value, req| {
                        // let value = value_write.clone();
                        async move {
                            println!("Should not print this");
                            // let mut value = value.lock().await;
                            // *value = new_value;
                            Ok(())
                        }
                        .boxed()
                    })),
                    ..Default::default()
                }),
                ..Default::default()
            },
            Characteristic {
                uuid: JS_RATE_UUID.parse().unwrap(),
                write: Some(CharacteristicWrite {
                    method: CharacteristicWriteMethod::Fun(Box::new(move |value, req| {
                        async move {
                            tokio::spawn(async move {
                                println!(
                                    "Write request {:?} with value 0x{}",
                                    &req,
                                    hex::encode(&value)
                                );
                            });
                            Ok(())
                        }
                        .boxed()
                    })),
                    write: true,
                    ..Default::default()
                }),
                ..Default::default()
            },
        ],
        ..Default::default()
    };
}
