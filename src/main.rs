use std::io::{self, Read};
use std::process::Command;
use std::thread;
use std::time::Duration;

fn main() {
    let domain: &str = "net.yanue.V2rayU";
    let key: &str = "userRules";

    // see more details from V2rayU repo, open the file of "$project/Preference/PreferencePac.swif", search for some code snippets like below:
    // ....
    // // read userRules from UserDefaults
    // let txt = UserDefaults.get(forKey: .userRules)
    // ...
    // so we use the defaults command of MacOS to read and write the user rules
    let output = Command::new("/usr/bin/defaults")
        .args(["read", domain, key])
        .output()
        .expect("failed to execute read user rules");

    if !output.status.success() {
        println!("failed to read user rules, maybe the key not exists, or the domain not exists");
        return;
    }
    // fetch the rules from the output
    let mut rules = fetch_rules(&output);
    // read the changes of rules from user input
    println!("Input the new rules followed by lines, press Ctrl+D to finish:");
    let mut user_input = String::new();
    io::stdin()
        .read_to_string(&mut user_input)
        .expect("failed to read input");
    let user_input = user_input.trim_end_matches('\n');
    // check if user_input already exists in rules
    if !rules.contains(user_input) {
        // in memory, append the new rules to the existing rules
        rules.push_str(&user_input);
    }
    // update the user rules finally
    Command::new("defaults")
        .args(["write", domain, key, &rules])
        .output()
        .expect("failed to execute write user rules");

    if !output.status.success() {
        println!("failed to write user rules");
        return;
    }
    println!("user rules updated successfully! trying to reload the rules in V2rayU...");
    restart_v2rayu_application();
}

fn fetch_rules(output: &std::process::Output) -> String {
    let mut rules = String::from_utf8_lossy(&output.stdout).to_string();
    let mut cleaned_rules = String::new();
    for line in rules.lines() {
        if line.trim().is_empty() {
            continue;
        }
        cleaned_rules.push_str(line);
        cleaned_rules.push_str("\n");
    }
    rules = cleaned_rules;
    println!("current user rules: {}", rules);
    rules
}

fn restart_v2rayu_application() {
    // close the V2rayU application
    let _ = Command::new("osascript")
        .args(&["-e", "tell application \"V2rayU\" to quit"])
        .output()
        .expect("Failed to execute osascript");

    // wait some time for the V2rayU application to exit
    thread::sleep(Duration::from_secs(2));

    // restart the V2rayU application
    let start_output = Command::new("open")
        .args(&["-a", "/Applications/V2rayU.app"])
        .output()
        .expect("Failed to open V2rayU");
    if start_output.status.success() {
        println!("V2rayU has been restarted.");
    } else {
        println!("Failed to restart V2rayU.");
    }
}
// /**  **/
// fn open_chrome_omega_extenstion_page() {
//     // open the chrome omega extension page, and allow the user to click the disable/enable button
//     // the shell script:
//     // osascript -e 'tell application "Google Chrome" to open location "chrome://extensions/?id=padekgcemlokbadohgkifijomclgjgif"'
//     let _ = Command::new("/opt/local/bin/open_omega_plugin")
//         .output()
//         .expect("Failed to open Omega plugin page in Chrome");

//     println!(
//         "Please click the disable/enable button in the Omega plugin page to reload the rules."
//     );
// }
