use windows::{Win32::System::Com::*, Win32::UI::Shell::*, Win32::UI::WindowsAndMessaging::*};

fn find_desktop_folder_view() -> anyhow::Result<()> {
    unsafe {
        let handle = GetForegroundWindow();

        let windows: IShellWindows = CoCreateInstance(&ShellWindows, None, CLSCTX_ALL)?;
    };

    Ok(())
}
