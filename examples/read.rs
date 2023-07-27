//! This example is adapted from the [MPSSE Basics] application note from FTDI.
//!
//! Get out your logic analyzer or oscilloscope!
//!
//! On the FT232H this will toggle ADBUS0 from high to low.
//!
//! [MPSSE Basics]: https://www.ftdichip.com/Support/Documents/AppNotes/AN_135_MPSSE_Basics.pdf
#![deny(unsafe_code)]
use libftd2xx::{BitMode, Ft232h, Ftdi, FtdiCommon};
use std::error::Error;
//use std::thread;
use std::time::Duration;

const RX_BUF_SIZE: usize = 0x10000;

fn main() -> Result<(), Box<dyn Error>> {
    let mut ft: Ft232h = Ftdi::new()?.try_into()?;
    let mut rx_buf: Vec<u8> = vec![0; RX_BUF_SIZE];

    ft.reset()?;
    ft.purge_all()?;
    debug_assert_eq!(ft.queue_status()?, 0);
    //ft.set_usb_parameters(65536)?;
    ft.set_chars(0, false, 0, false)?;
    ft.set_timeouts(Duration::from_millis(10000), Duration::from_millis(10000))?;
    ft.set_latency_timer(Duration::from_millis(16))?;
    ft.set_flow_control_rts_cts()?;
    ft.set_bit_mode(0x00, BitMode::Reset)?;
    ft.set_bit_mode(0x00, BitMode::SyncFifo)?; //CHECK mask? 0xFF instead, like other code?
    //ft.set_flow_control_rts_cts()?;

    // From the application note "Wait for all the USB stuff to complete and work"
    // This does not seem to be necessary though
    // thread::sleep(Duration::from_millis(100));

    for _ in 0..10 {
        print!(".");
        ft.read_all(&mut rx_buf)?;
        print!("rx: ");
        for i in 0..rx_buf.len() {
            print!("{},", rx_buf[i]);
        }
        println!();
        println!();
        //ft.write_all(&rx_buf)?;
    }

    println!();

    ft.close()?;

    Ok(())
}



