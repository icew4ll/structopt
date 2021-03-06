// imports {{{
#[macro_use]
extern crate structopt;
#[macro_use]
extern crate serde_derive;
// #[macro_use]
// extern crate lazy_static;
extern crate csv;
extern crate duct;
extern crate regex;
use duct::cmd;
// use regex::RegexSet;
use std::error::Error;
use std::fs::File;
use std::process;
use structopt::StructOpt;
// macro to create vector of strings
macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}
// }}}
// structs and enums {{{
#[derive(Debug, Deserialize)]
struct Csv {
    alias: String,
    ip: String,
    user: String,
    pass: String,
}
// structopt argument parsing
#[derive(Debug, StructOpt)]
#[structopt(name = "foo")]
struct Opt {
    file: Option<String>,
    #[structopt(subcommand)]
    cmd: Option<Command>,
}
#[derive(Debug, StructOpt)]
enum Command {
    Audit { ips: Vec<String> },
    Qua { test: String },
    Rex { test: String },
}
// }}}
// main {{{
fn main() {
    // connections {{{
    // init vecs
    let mut connections = vec![];
    // read file
    if let Err(err) = read(&mut connections) {
        println!("error running example: {}", err);
        process::exit(1);
    }
    // println!("{:?}", connections);
    //}}}
    let opt = Opt::from_args();
    // println!("{:?}", opt);
    // println!("{:?}", opt.file);
    // argument index 1 parsing {{{
    if let Some(arg1) = opt.file {
        let t = &connections
            .into_iter()
            .filter(|i| i.0 == arg1)
            .collect::<Vec<_>>();
        // println!("{:?}", t);
        // get vars
        let ip = t[0].1.to_string();
        let user = t[0].2.to_string();
        let pass = t[0].3.to_string();
        // build ssh expect:
        if let Err(err) = ssh(ip, user, pass, "interact") {
            println!("{}", err);
            process::exit(1);
        }
    }
    // }}}
    // match subcommands {{{
    match opt.cmd {
        Some(Command::Audit { ips }) => {
            if let Err(err) = audit(ips) {
                println!("{}", err);
                process::exit(1);
            }
        }
        _ => (),
    }
    // }}}
}
// }}}
// ssh {{{
fn ssh(ip: String, user: String, pass: String, route: &str) -> Result<(), Box<Error>> {
    println!("{} {} {} {}", ip, user, pass, route);
    // command pieces
    let slice = &ip[0..2];
    let setup = vec_of_strings!["set prompt {[#|%|>|$] $}\n"];
    let blan = vec_of_strings![
        format!("spawn ssh $env({})@$env({})", "UI", "AMBK10"),
        "expect $prompt",
        format!("send ssh \"$env({})@{}\n\"", user, ip),
        "expect \"assword\"",
        format!("send \"$env({})\n\"", pass),
        "expect $prompt"
    ];
    let lan = vec_of_strings![
        format!("spawn ssh $env({})@{}", user, ip),
        "expect \"assword\"",
        format!("send \"$env({})\n\"", pass),
        "expect $prompt"
    ];
    let sudo = vec_of_strings![
        "send \"sudo su -\n\"",
        "expect \"assword\"",
        format!("send \"$env({})\n\"", pass),
        "expect $prompt"
    ];
    let interact = vec_of_strings!["interact"];
    let brdesktop = format!(
        "ssh -f -N -D9050 $UI@$AMBK10; proxychains rdesktop -g 1300x708 -5 -K -r clipboard:CLIPBOARD -u ${} -p ${} {}",
        user, pass, ip
    );
    let rdesktop = format!(
        "rdesktop -g 1300x708 -5 -K -r clipboard:CLIPBOARD -u ${} -p ${} {}",
        user, pass, ip
    );
    // command assemble
    match slice.as_ref() {
        "10" => match user.as_ref() {
            "UR" => {
                let cmds = [&setup[..], &blan[..], &interact[..]].concat();
                println!("{}", &cmds.join(";"));
                let args = &["-c", &cmds.join(";")];
                cmd("expect", args).run().unwrap();
            }
            "UP" | "UI" => {
                let cmds = [&setup[..], &blan[..], &sudo[..], &interact[..]].concat();
                println!("{}", &cmds.join(";"));
                let args = &["-c", &cmds.join(";")];
                cmd("expect", args).run().unwrap();
            }
            "UA" | "US" | "UL" => {
                println!("{}", &brdesktop);
                let args = &["-c", &brdesktop];
                cmd("bash", args).run().unwrap();
            }
            _ => (),
        },
        _ => match user.as_ref() {
            "UR" => {
                let cmds = [&setup[..], &lan[..], &interact[..]].concat();
                println!("{}", &cmds.join(";"));
                let args = &["-c", &cmds.join(";")];
                cmd("expect", args).run().unwrap();
            }
            "UP" | "UI" => {
                let cmds = [&setup[..], &lan[..], &sudo[..], &interact[..]].concat();
                println!("{}", &cmds.join(";"));
                let args = &["-c", &cmds.join(";")];
                cmd("expect", args).run().unwrap();
            }
            "UA" | "US" | "UL" => {
                println!("{}", &rdesktop);
                let args = &["-c", &rdesktop];
                cmd("bash", args).run().unwrap();
            }
            _ => (),
        },
    }
    Ok(())
}
// }}}
// audit {{{
fn audit(ips: Vec<String>) -> Result<(), Box<Error>> {
    println!("IPS: {:?}", ips);
    for ip in ips {
        let test = vec_of_strings![
            "set prompt {[#|%|>|$] $}\n",
            format!("spawn ssh $env({})@{}", "UI", ip),
            "expect \"assword\"",
            format!("send \"$env({})\n\"", "PG"),
            "expect $prompt",
            "send \"uname -n\n\"",
            "expect $prompt",
            // "send \"python -c 'import platform;print(platform.platform())'\n\"",
            "send \"cat /etc/*release | grep -m 1 -E '(Red Hat|Cent|Debian|PRETTY_NAME|Fedora)' | sed -e 's/PRETTY_NAME=//'\n\"",
            "expect $prompt",
            "send \"php -v\n\"",
            "expect $prompt",
            "send \"openssl version\n\"",
            "expect $prompt",
            // new
            "send \"grep -m 1 -E 'model name' /proc/cpuinfo | awk '{ print substr(\\$0, index(\\$0,\\$4)) }'\n\"",
            "expect $prompt",
            "send \"httpd -v | awk '/Apache/ {print \\$3}'\n\"",
            "expect $prompt",
            "send \"perl -v | head -2\n\"",
            "expect $prompt",
            "send \"mysql -V\n\"",
            "expect $prompt",
            "send \"iscsiadm -m session -P 3 | egrep '(iscsiadm|Attached scsi)' | sed -e 's/\\[\\\\t\\]*//'\n\"",
            "expect $prompt",
            // exit
            "send \"exit\n\"",
            "expect eof",
            "exit"
        ];
        // println!("{}", test.join(";"));
        let args = &["-c", &test.join(";")];
        let stdout = cmd("expect", args).read().unwrap();
        // println!("{:?}", stdout);
        // split
        let vec: Vec<&str> = stdout.split("\r\n").collect();
        // strip out command lines
        let strip = vec
            .into_iter()
            .filter(|x| !x.contains("$"))
            .filter(|x| !x.contains("#"))
            .filter(|x| !x.contains("Zend"))
            .filter(|x| !x.contains("Copyright"))
            .filter(|x| !x.contains("perl -v"))
            .filter(|&x| x != "")
            .filter(|&x| x != "exit")
            .filter(|&x| x != "mysql -V")
            .filter(|&x| x != "openssl version")
            .filter(|&x| x != "php -v")
            .collect::<Vec<_>>();
        // print strip
        let select = strip
            .iter()
            .position(|&x| x.contains("Last login")).unwrap();
        // strip.iter().for_each(|x| println!("{:?}", x));
        let finish = &strip[select+1..strip.len()-2].join("%");
        println!("{}", finish);
        // contains
        // let mysql = &strip.iter().filter(|x| x.contains("mysql")).collect::<Vec<_>>();
        // let iscsiadm = &strip
        //     .iter()
        //     .filter(|x| x.contains("iscsiadm"))
        //     .collect::<Vec<_>>();
        // let intel = &strip
        //     .iter()
        //     .filter(|x| x.contains("Intel"))
        //     .collect::<Vec<_>>();
        // let openssl = &strip
        //     .iter()
        //     .filter(|x| x.contains("OpenSSL"))
        //     .collect::<Vec<_>>();
        // let php = &strip
        //     .iter()
        //     .filter(|x| x.contains("php"))
        //     .collect::<Vec<_>>();
        // let perl = &strip
        //     .iter()
        //     .filter(|x| x.contains("perl"))
        //     .collect::<Vec<_>>();
        // let linux = &strip
        //     .iter()
        //     .filter(|x| x.contains("Linux-"))
        //     .collect::<Vec<_>>();
        // let httpd = &strip
        //     .iter()
        //     .filter(|x| x.contains("httpd"))
        //     .collect::<Vec<_>>();
        // println!(
        //     "{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?}",
        //     mysql, iscsiadm, intel, openssl, php, perl, linux, httpd
        // );
    }
    Ok(())
}
// }}}
// read file {{{
fn read(connections: &mut Vec<((String, String, String, String))>) -> Result<(), Box<Error>> {
    let file = File::open("list")?;
    let mut rdr = csv::ReaderBuilder::new().flexible(true).from_reader(file);
    for result in rdr.deserialize() {
        let record: Csv = result?;
        connections.push((record.alias, record.ip, record.user, record.pass))
    }
    Ok(())
}
// }}}
