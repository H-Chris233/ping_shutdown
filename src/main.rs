use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
use std::process::Output;
use std::env;

#[cfg(windows)]
const SHUTDOWN_COMMAND: &str = "shutdown /s /t 0";

#[cfg(unix)]
const SHUTDOWN_COMMAND: &str = "poweroff";

fn main() {
    #[cfg(windows)]
    cmd_to_utf8();
    
    println!("Started running...");
    let ip1 = "192.168.3.8";
    let ip2 = "192.168.3.9";
    loop_60sec(ip1 ,ip2);

}

#[allow(dead_code)]
fn get_ip() -> Vec<String> {
    let mut args = vec![];
    for arg in env::args().skip(1) {
        args.push(arg);
            //.expect("Error getting argument");
        if args.len() == 0 {
            println!("Useage:ping_shutdown ip ip ...");
            
            }
        }
        args    
}

fn loop_60sec(ip1:&str ,ip2:&str) {
    let secs = 60;
    println!("Started 60sec loop...");
    for i in 1.. {
        println!("60sec Looped for {} times...", i);
        let status = verify(ip1,ip2,secs);
        if status == false {
            loop_20sec(ip1 ,ip2);
            continue;
        }
        sleep(Duration::from_secs(60));
    }
}

fn get_status(ip:&str) -> String {
    let command = format!("ping {} -n 1" ,ip);
    let message = format!("Started clienting {}..." ,ip);
    let output = match run_command(&command, &message){
    Ok(output) => output,
    Err(_) => error(),
    };
    
    let status = String::from_utf8_lossy(&output.stdout);
    let status =  status.to_string();
    status
}

fn check_status(ip:&str) -> bool {
    let status = get_status(ip);
    println!("Started checking {}..." ,ip);
    if status.contains("TTL") {
        println!("fine.");
        return true;
    }else{
        println!("Request timed out.");
        return false;
    }

}

fn verify(ip1:&str ,ip2:&str ,secs:i32) -> bool {
    let status1 = check_status(ip1);
    let status2 = check_status(ip2);
    println!("{} secs left for the next loop..." ,secs);
    if status1 == false && status2 == false {
        return false;
    }else{
        return true;
    }
}

fn loop_20sec(ip1:&str ,ip2:&str) {
    let secs = 20;
    let mut time_left = 3;
    println!("Warning!!! Connection lost!!!!");
    println!("Checking web connection every 20 seconds!!");
    loop{
        println!("{} times left for shutting down...", time_left);
        let status = verify(ip1,ip2,secs);
        if status == true {
            break;
        }else if time_left == 0 {
            shutdown();
        }
        sleep(Duration::from_secs(20));
        time_left -= 1;
    }
    println!("Reconnected!!!");
    println!("Exiting 20sec loop...");
}

fn shutdown() {
    let _ =run_command(SHUTDOWN_COMMAND, "Started shutting down...");
}

#[cfg(windows)]
fn run_command(command: &str, message: &str) -> Result<Output, std::io::Error> {
    println!("{}", message);
    let output = Command::new("cmd")
        .arg("/C")
        .arg(command)
        .output()?;
    Ok(output)
}

#[cfg(unix)]
fn run_command(command: &str, message: &str) -> Result<Output, std::io::Error> {
    println!("{}", message);
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()?;
    Ok(output)
}

#[cfg(windows)]
fn cmd_to_utf8() {
    let _ = Command::new("cmd")
        .arg("/C")
        .arg("chcp 65001")
        .output()
        .expect("I/O ERROR!!!");
}

fn error() -> ! {
    println!("An error occured,please send an email to h-chris233@qq.com or open a issue to help me improve, tanks!");
    std::process::exit(1);
    
    }