use anyhow::anyhow;
use std::{env, fs};

pub fn systemd() -> anyhow::Result<()> {
	let home = env::var("HOME")?;

	let service_path = format!("{home}/.config/systemd/user/deployd.service");
	let service = format!(
		"[Unit]
Description=deployd

[Service]
Type=simple
StandardOutput=journal
ExecStart=~/.cargo/bin/deployd

[Install]
WantedBy=default.target
"
	);
	let replace = match fs::metadata(&service_path) {
		Ok(_) => {
			let content = fs::read_to_string(&service_path)?;
			if content == service {
				false
			} else {
				fs::remove_file(&service_path)?;
				true
			}
		}
		Err(_) => true,
	};
	if replace {
		fs::write(&service_path, service)?;
		bash::run("systemctl --user enable deployd.service")?;
		bash::run("systemctl --user start deployd.service")?;
		bash::run("systemctl --user status deployd.service")?;
	}

	Ok(())
}
