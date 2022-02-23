#![cfg(target_os = "linux")]

use regex::Regex;
use std::path::Path;
use std::process::Command;

use clap::ArgMatches;

use crate::resolve::resolve_versions;
use crate::rversion::*;

use crate::download::*;
use crate::escalate::*;
use crate::utils::*;

pub fn sc_add(args: &ArgMatches) {
    escalate();
    let linux = detect_linux();
    let mut version = get_resolve(args);
    let ver = version.version.to_owned();
    let verstr = match ver {
        Some(ref x) => x,
        None => "???"
    };

    let url: String = match &version.url {
        Some(s) => s.to_string(),
        None => panic!("Cannot find a download url for R version {}", verstr),
    };

    let filename = basename(&url).unwrap();
    let tmp_dir = std::env::temp_dir().join("rim");
    let target = tmp_dir.join(&filename);
    let target_str;
    if target.exists() {
        target_str = target.into_os_string().into_string().unwrap();
        println!("{} is cached at\n    {}", filename, target_str);
    } else {
        target_str = target.into_os_string().into_string().unwrap();
        println!("Downloading {} ->\n    {}", url, target_str);
        let client = &reqwest::Client::new();
        download_file(client, url, &target_str);
    }

    if linux.distro == "ubuntu" || linux.distro == "debian" {
	add_deb(target_str);
    }

    // system_create_lib(vec![ver]);
    // sc_system_make_links();
}

fn add_deb(path: String) {
    let status = Command::new("apt-get")
	.args(["update"])
	.spawn()
	.expect("Failed to run apt-get update")
	.wait()
	.expect("Failed to run apt-get update");

   if !status.success() {
       panic!("apt-get install exited with status {}", status.to_string());
   }

    let status = Command::new("apt-get")
	.args(["install", "-y", "gdebi-core"])
	.spawn()
	.expect("Failed to install gdebi-core")
	.wait()
	.expect("Failed to install gdebi-core");

    if !status.success() {
        panic!("apt-get exited with status {}", status.to_string());
    }

    let status = Command::new("gdebi")
	.args(["-n", &path])
	.spawn()
	.expect("Failed to run gdebi")
	.wait()
	.expect("Failed to run gdebi");

   if !status.success() {
       panic!("gdebi exited with status {}", status.to_string());
   }
}

pub fn sc_rm(args: &ArgMatches) {
    unimplemented!();
}

pub fn sc_system_add_pak(args: &ArgMatches) {
    unimplemented!();
}

pub fn system_create_lib(vers: Option<Vec<String>>) {
    unimplemented!();
}

pub fn sc_system_make_links() {
    unimplemented!();
}

pub fn get_resolve(args: &ArgMatches) -> Rversion {
    let str = args.value_of("str").unwrap().to_string();

    let eps = vec![str];
    let me = detect_linux();
    let version = resolve_versions(eps, "linux".to_string(), "default".to_string(), Some(me));
    version[0].to_owned()
}

pub fn sc_get_list() -> Vec<String> {
    unimplemented!();
}

pub fn sc_set_default(ver: String) {
    unimplemented!();
}

pub fn sc_show_default() {
    unimplemented!();
}

pub fn sc_system_make_orthogonal(_args: &ArgMatches) {
    // Nothing to do on Windows
}

pub fn sc_system_fix_permissions(args: &ArgMatches) {
    // Nothing to do on Windows
}

pub fn sc_system_forget() {
    // Nothing to do on Windows
}

fn detect_linux() -> LinuxVersion {
    let release_file = Path::new("/etc/os-release");
    let lines = match read_lines(release_file) {
        Ok(x) => { x },
        Err(err) => { panic!("Unknown Linux, no /etc/os-release"); }
    };

    let re_id = Regex::new("^ID=").unwrap();
    let wid_line = grep_lines(&re_id, &lines);
    if wid_line.len() == 0 {
        panic!("Unknown Linux distribution");
    }
    let id_line = &lines[wid_line[0]];
    let id = re_id.replace(&id_line, "").to_string();
    let id = unquote(&id);

    let re_ver = Regex::new("^VERSION_ID=").unwrap();
    let wver_line = grep_lines(&re_ver, &lines);
    if wver_line.len() == 0 {
        panic!("Unknown {} Linux version", id);
    }
    let ver_line = &lines[wver_line[0]];
    let ver = re_ver.replace(&ver_line, "").to_string();
    let ver = unquote(&ver);

    let mut mine = LinuxVersion { distro: id.to_owned(),
				  version: ver.to_owned(),
				  url: "".to_string() };

    let supported = list_supported_distros();

    let mut good = false;
    for dis in supported {
	if dis.distro == mine.distro && dis.version == mine.version {
	    mine.url = dis.url;
	    good = true;
	}
    }

    if ! good {
	panic!(
	    "Unsupported distro: {} {}, see rim list-supported",
	    &id,
	    &ver
	);
    }

    mine
}

fn list_supported_distros() -> Vec<LinuxVersion> {
    vec![
	LinuxVersion { distro: "ubuntu".to_string(),
		       version: "18.04".to_string(),
		       url: "https://cdn.rstudio.com/r/ubuntu-1804/pkgs/r-{}_1_amd64.deb".to_string() },
	LinuxVersion { distro: "ubuntu".to_string(),
		       version: "20.04".to_string(),
		       url: "https://cdn.rstudio.com/r/ubuntu-2004/pkgs/r-{}_1_amd64.deb".to_string() },
	LinuxVersion { distro: "ubuntu".to_string(),
		       version: "22.04".to_string(),
		       url: "https://cdn.rstudio.com/r/ubuntu-2204/pkgs/r-{}_1_amd64.deb".to_string() },
	LinuxVersion { distro: "debian".to_string(),
		       version: "9".to_string(),
		       url: "https://cdn.rstudio.com/r/debian-9/pkgs/r-${}_1_amd64.deb".to_string() },
	LinuxVersion { distro: "debian".to_string(),
		       version: "10".to_string(),
		       url: "https://cdn.rstudio.com/r/debian-9/pkgs/r-${}_1_amd64.deb".to_string() },
    ]
}

pub fn sc_clean_registry() {
    // Nothing to do on Linux
}
