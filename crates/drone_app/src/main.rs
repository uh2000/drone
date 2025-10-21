// ...existing code...
use std::io::{self, ErrorKind};
use std::net::UdpSocket;
use std::time::Duration;

/// Minimal Tello SDK client used by `drone_app`.
pub struct Tello {
    socket: UdpSocket,
}

const TELLO_ADDR: &str = "192.168.10.1:8889";

impl Tello {
    /// Bind local socket and connect to Tello address.
    pub fn new() -> io::Result<Self> {
        let socket = UdpSocket::bind("0.0.0.0:0")?;
        socket.connect(TELLO_ADDR)?;
        socket.set_read_timeout(Some(Duration::from_secs(5)))?;
        socket.set_write_timeout(Some(Duration::from_secs(3)))?;
        Ok(Self { socket })
    }

    /// Enter SDK command mode.
    pub fn connect(&self) -> io::Result<()> {
        self.expect_ok("command")
    }

    pub fn battery(&self) -> io::Result<u8> {
        let s = self.send("battery?")?;
        s.trim().parse::<u8>().map_err(|e| {
            io::Error::new(ErrorKind::Other, format!("parse battery failed: {} (got '{}')", e, s))
        })
    }

    pub fn takeoff(&self) -> io::Result<()> {
        self.expect_ok("takeoff")
    }

    pub fn land(&self) -> io::Result<()> {
        self.expect_ok("land")
    }

    pub fn rotate_clockwise(&self, deg: u16) -> io::Result<()> {
        self.expect_ok(&format!("cw {}", deg))
    }

    pub fn move_backward(&self, cm: u16) -> io::Result<()> {
        self.expect_ok(&format!("back {}", cm))
    }

    pub fn move_right(&self, cm: u16) -> io::Result<()> {
        self.expect_ok(&format!("right {}", cm))
    }

    /// Send raw command and return response string.
    pub fn send(&self, cmd: &str) -> io::Result<String> {
        self.socket.send(cmd.as_bytes())?;
        let mut buf = [0u8; 1024];
        let n = self.socket.recv(&mut buf)?;
        Ok(String::from_utf8_lossy(&buf[..n]).trim().to_string())
    }

    fn expect_ok(&self, cmd: &str) -> io::Result<()> {
        let resp = self.send(cmd)?;
        if resp.eq_ignore_ascii_case("ok") {
            Ok(())
        } else {
            Err(io::Error::new(ErrorKind::Other, format!("'{}' -> '{}'", cmd, resp)))
        }
    }
}

// ...existing code...
// added main entry point
fn main() -> std::io::Result<()> {
    let drone = Tello::new()?;
    drone.connect()?;

    let battery = drone.battery()?;
    println!("Battery: {}%", battery);
    if battery < 15 {
        eprintln!("Battery too low for flight; aborting.");
        return Ok(());
    }

    drone.takeoff()?;
    std::thread::sleep(Duration::from_secs(2));
    drone.rotate_clockwise(90)?;
    drone.move_backward(100)?;
    drone.move_right(100)?;
    drone.land()?;

    Ok(())
}