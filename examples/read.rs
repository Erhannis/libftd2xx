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
use std::time::{Duration, Instant};
use std::cmp::min;

const RX_BUF_SIZE: usize = 0x10000*3;//*0x200
const MAX_PRINT_SIZE: usize = 0x10;
// const MAX_PRINT_SIZE: usize = 0x1000000;
const ITER: i32 = 0x10;

fn main() -> Result<(), Box<dyn Error>> {
    let mut ft: Ft232h = Ftdi::new()?.try_into()?;
    let mut rx_buf: Vec<u8> = vec![0; RX_BUF_SIZE];

    ft.reset()?;
    ft.purge_all()?;
    //debug_assert_eq!(ft.queue_status()?, 0);
    ft.set_usb_parameters(65536)?;
    ft.set_chars(0, false, 0, false)?;
    ft.set_timeouts(Duration::from_millis(10000), Duration::from_millis(10000))?;
    //ft.set_latency_timer(Duration::from_millis(16))?;
    //ft.set_flow_control_rts_cts()?;
    ft.set_bit_mode(0x00, BitMode::Reset)?;
    ft.set_bit_mode(0x00, BitMode::SyncFifo)?; //CHECK mask? 0xFF instead, like other code?

    // From the application note "Wait for all the USB stuff to complete and work"
    // This does not seem to be necessary though
    // thread::sleep(Duration::from_millis(100));

    let now0 = Instant::now();
    let mut total: u128 = 0;
    for _ in 0..ITER {
        // print!(". ");
        let now = Instant::now();
        ft.read_all(&mut rx_buf)?;
        let t: u128 = now.elapsed().as_micros();
        let z: u128 = (RX_BUF_SIZE * 1000000).try_into().unwrap();
        total += u128::try_from(RX_BUF_SIZE).unwrap();
        // println!("{RX_BUF_SIZE} @ {} = {} B/s", t, z/t);
        // print!("rx: ");
        // let n = min(rx_buf.len(), MAX_PRINT_SIZE);
        // if n < rx_buf.len() {
        //     for i in 0..n {
        //         print!("{:#03},", rx_buf[i]);
        //     }
        //     print!("...");
        //     for i in (rx_buf.len()-n)..rx_buf.len() {
        //         print!("{:#03},", rx_buf[i]);
        //     }
        // } else {
        //     for i in 0..n {
        //         print!("{:#03},", rx_buf[i]);
        //     }
        // }
        // println!();
        // println!();
        //ft.write_all(&rx_buf)?;
    }

    let t: u128 = now0.elapsed().as_micros();
    let z: u128 = (total * 1000000).try_into().unwrap();
    println!("total {total} @ {} = {} B/s", t, z/t);

    println!();

    ft.close()?;

    Ok(())
}



