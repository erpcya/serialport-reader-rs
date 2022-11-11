use std::{time::Duration, thread, io::{Read, self}};

use serialport::{Parity, FlowControl, StopBits, DataBits, SerialPort};



fn main() {
    let mut _handler = serialport::new("/dev/ttyUSB0", 9600)
    .timeout(Duration::from_millis(10))
    .parity(Parity::None)
    .flow_control(FlowControl::Hardware)
    .stop_bits(StopBits::One)
    .data_bits(DataBits::Eight)
    .open_native().expect("Failed to open port");
    let mut clone = _handler.try_clone().expect("Failed to clone");
    thread::spawn(move || 
        // {
        //     let _complete_command: &[u8] = &_complete_command_to_copy;
        //     println!("complete command {:?}", _complete_command);
        //     clone
        //     .write_all(_complete_command)
        //     .expect("Failed to write to serial port")
        // }
        //loop 
        {
            thread::sleep(Duration::from_millis(1000));
            // X
            // &[2, 27, 57, 28, 88, 28, 84, 3, 48, 49, 51, 68]
            //  2, 29, 57, 28, 88, 28, 84, 3, 48, 49, 51, 70
            // Status
            // 2, 122, 56, 28, 78, 3, 48, 49, 50, 49
            // 2, 78, 56, 28, 78, 3, 48, 48, 70, 53
            let data = [2, 27, 57, 28, 88, 28, 84, 3, 48, 49, 51, 68];
            clone
            .write(&data)
            .expect("Failed to write to serial port");
            // clone
            //     .write_all(&[5, 6, 7, 8])
            //     .expect("Failed to write to serial port");
            println!("Epale {:?}", &data);
            // thread::sleep(Duration::from_millis(100000));
        }
    );
    loop {
        let mut _result: Vec<u8> = vec![0; 32];
        // thread::sleep(Duration::from_millis(10));
        // println!(
        //     "Bytes available to read: {}",
        //     _handler.bytes_to_read().expect("Error calling bytes_to_read")
        // );
        match _handler.read(_result.as_mut_slice()) {
            Ok(t) => {
                // println!("Hola -- : {:?}", _result);
                // _handler.write_all(&_result[..t]).unwrap();
                println!("Hola: {:?}", &_result[..t]);
                // println!("{:?}", _result[..t].iter().map(|&c| c as char).collect::<String>());
                // break;
            },
            // Ok(_bytes) => {
            //     // if bytes == 1 {
            //     //     // print!("{:?}, ", buffer);
            //     //     _result.push(buffer[0]);
            //     // }
            //     // println!("Epale {:?}, ", buffer);
                // println!("Epale {:?}, ", _result);
            //     break;
            // }
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
            Err(e) => eprintln!("{:?}", e),
        }
        // match time.elapsed() {
        //     t if t.as_millis() > 5000 => {
        //         println!("timeout");
        //         break;
        //     },
        //     _ => {
        //         // println!("{:x?}", _result);
        //         // println!("{:?}", _result.iter().map(|&c| c as char).collect::<String>());
        //     }
        // }
    }
}