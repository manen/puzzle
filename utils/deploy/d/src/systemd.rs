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
ExecStart={home}/.cargo/bin/deployd

[Install]
WantedBy=default.target
"
	);
	let (replace, existed) = match fs::metadata(&service_path) {
		Ok(_) => {
			let content = fs::read_to_string(&service_path)?;
			if content == service {
				(false, true)
			} else {
				fs::remove_file(&service_path)?;
				(true, true)
			}
		}
		Err(_) => (true, false),
	};
	if replace {
		fs::write(&service_path, service)?;
	}
	if !existed {
		bash::run("systemctl --user enable deployd.service")?;
		bash::run("systemctl --user start deployd.service")?;
		bash::run("systemctl --user status deployd.service")?;
	} else {
		bash::run("systemctl --user restart deployd.service")?;
	}

	Ok(())
}
