tell application "Finder"
    if (count of Finder windows) > 0 then
        try
            get POSIX path of (target of front Finder window as text)
        on error
            return "Error: No valid target in front Finder window."
        end try
    else
        get POSIX path of (desktop as alias)
    end if
end tell
