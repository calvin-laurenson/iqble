mod iq_brain;
mod iq_controller;
use bluer::{
    adv::Advertisement,
    gatt::local::{
        Application, Characteristic, CharacteristicNotify, CharacteristicNotifyMethod, Service,
    },
};
// use gilrs::Gilrs;
use std::time::Duration;
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    time::{interval, sleep},
};

const ADVERT_DATA: &str = "604489-1-5-11-0";

#[tokio::main(flavor = "current_thread")]
async fn main() -> bluer::Result<()> {
    // let mut gilrs = Gilrs::new().unwrap();
    // let gamepads: Vec<gilrs::Gamepad> = gilrs.gamepads().map(|(_id, pad)| pad).collect();
    // if gamepads.len() == 0 {
    //     println!("No gamepads found");
    //     return Ok(());
    // }
    // let gamepad = gamepads[0];
    // println!(
    //     "Using {} which is {:?}",
    //     gamepad.name(),
    //     gamepad.power_info()
    // );

    // let LeftStickXCode = gamepad.axis_code(gilrs::Axis::LeftStickX).unwrap();
    // let LeftStickYCode = gamepad.axis_code(gilrs::Axis::LeftStickY).unwrap();
    // let RightStickXCode = gamepad.axis_code(gilrs::Axis::RightStickX).unwrap();
    // let RightStickYCode = gamepad.axis_code(gilrs::Axis::RightStickY).unwrap();
    // println!(
    //     "LeftStickXCode: {}, LeftStickYCode: {}, RightStickXCode: {}, RightStickYCode: {}",
    //     LeftStickXCode, LeftStickYCode, RightStickXCode, RightStickYCode
    // );
    // // loop {
    // let state = gamepad.state();
    // println!("{:?}", state.buttons().map(|(_, b)| b.is_pressed()).collect::<Vec<bool>>());
    // // }

    // return Ok(());
    // env_logger::init();
    let session = bluer::Session::new().await?;
    let adapter = session.default_adapter().await?;
    adapter.set_powered(true).await?;
    println!(
        "Advertising on Bluetooth adapter {} with address {}",
        adapter.name(),
        adapter.address().await?
    );
    let le_advertisement = Advertisement {
        advertisement_type: bluer::adv::Type::Peripheral,
        service_uuids: vec![
            iq_controller::CONTROLLER_UUID.parse().unwrap(),
            iq_brain::BRAIN_UUID.parse().unwrap(),
        ]
        .into_iter()
        .collect(),
        discoverable: Some(true),
        local_name: Some(ADVERT_DATA.to_string()),
        ..Default::default()
    };
    println!("{:?}", &le_advertisement);
    let adv_handle = adapter.advertise(le_advertisement).await?;

    println!(
        "Serving GATT service on Bluetooth adapter {}",
        adapter.name()
    );
    let app = Application {
        services: vec![
            iq_controller::make_controller_service(),
            iq_brain::make_brain_service(),
        ],
        ..Default::default()
    };

    let app_handle = adapter.serve_gatt_application(app).await?;

    println!("Service ready. Press enter to quit.");
    let stdin = BufReader::new(tokio::io::stdin());
    let mut lines = stdin.lines();
    let _ = lines.next_line().await;

    println!("Removing service and advertisement");
    drop(app_handle);
    drop(adv_handle);
    sleep(Duration::from_secs(1)).await;

    Ok(())
}
