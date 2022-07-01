use bluer::gatt::local::{
    Characteristic, CharacteristicNotify, CharacteristicNotifyMethod, CharacteristicRead,
    CharacteristicWrite, CharacteristicWriteMethod, Service,
};
use futures_util::FutureExt;

pub const BRAIN_UUID: &str = "08590f7e-db05-467e-8757-72f6faeb13d5";
const BRAIN_RX_UUID: &str = "08590f7e-db05-467e-8757-72f6faeb13f5";
const BRAIN_TX_UUID: &str = "08590f7e-db05-467e-8757-72f6faeb1306";

pub fn make_brain_service() -> Service {
    return Service {
        uuid: BRAIN_UUID.parse().unwrap(),
        primary: false,
        characteristics: vec![
            Characteristic {
                uuid: BRAIN_RX_UUID.parse().unwrap(),
                read: Some(CharacteristicRead {
                    read: true,
                    fun: Box::new(move |req| {
                        async move {
                            println!("Recived read on BRAIN_RX");
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
                        async move {
                            println!(
                                "Recived write on BRAIN_RX with a value of 0x{}",
                                hex::encode(&new_value)
                            );
                            Ok(())
                        }
                        .boxed()
                    })),
                    ..Default::default()
                }),
                notify: Some(CharacteristicNotify {
                    notify: true,
                    indicate: true,
                    method: CharacteristicNotifyMethod::Fun(Box::new(move |mut notifier| {
                        async move {
                            tokio::spawn(async move {
                                println!(
                                    "Recived notify subscription on BRAIN_RX with confirming={:?}",
                                    notifier.confirming()
                                );
                            });
                        }
                        .boxed()
                    })),
                    ..Default::default()
                }),
                ..Default::default()
            },
            Characteristic {
                uuid: BRAIN_TX_UUID.parse().unwrap(),
                read: Some(CharacteristicRead {
                    read: true,
                    fun: Box::new(move |req| {
                        async move {
                            println!("Recived read on BRAIN_TX");
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
                        async move {
                            println!(
                                "Recived write on BRAIN_TX with a value of 0x{}",
                                hex::encode(&new_value)
                            );
                            Ok(())
                        }
                        .boxed()
                    })),
                    ..Default::default()
                }),
                ..Default::default()
            },
        ],
        ..Default::default()
    };
}
