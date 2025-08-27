use strip_ansi_escapes::strip;

fn main() {
    let raw_string = "\u{1b}[38;2;203;166;247mmini-act\u{1b}[39m";
    let cleaned_string = String::from_utf8_lossy(&strip(raw_string.as_bytes()).unwrap()).to_string();
    eprintln!("Raw: {:?}", raw_string);
    eprintln!("Cleaned: {:?}", cleaned_string);

    let raw_string_with_more_codes = "\u{1b}[2K\u{1b}[1A\u{1b}[2K\u{1b}[1A\u{1b}[2K\u{1b}[1A\u{1b}[2K\u{1b}[1A\u{1b}[2K\u{1b}[1A\u{1b}[2K\u{1b}[G\u{1b}[38;2;203;166;247m✦ \u{1b}[39mI'll create a minimal Rust port of \u{1b}[38;2;203;166;247mact\u{1b}[39m to run our GitHub Actions workflow locally, avoiding Go. First,\r\n  I'll analyze \u{1b}[38;2;203;166;247mact\u{1b}[39m's \u{1b}[38;2;203;166;247mmain.go\u{1b}[39m to understand its core logic. Then, I'll create a new Rust project named\r\n  \u{1b}[38;2;203;166;247mmini-act\u{1b}[39m in the \u{1b}[38;2;203;166;247mcrates\u{1b}[39m directory. I'll implement a YAML parser using \u{1b}[38;2;203;166;247mserde_yaml\u{1b}[39m and a simple runner to\r\n  execute the \u{1b}[38;2;203;166;247mcheck_and_build\u{1b}[39m job's steps sequentially. Docker integration is deferred as it's not\r\n  currently needed. I'll begin by examining \u{1b}[38;2;203;166;247mmain.go\u{1b}[39m.\r\n⠙ \u{1b}[38;2;203;166;247mDissecting `act`'s Architecture\u{1b}[38;2;108;112;134m (esc to cancel, 7s)\u{1b}[39m\r\n\r\n\u{1b}[38;2;108;112;134mUsing: 2 GEMINI.md files\u{1b}[39m\r\n\u{1b}[38;2;137;180;250m~/storage/github/libminizinc\u{1b}[38;2;108;112;134m \u{1b}[39m           \u{1b}[38;2;243;139;168mno sandbox \u{1b}[38;2;108;112;134m(see \u{1b}[39m \u{1b}[38;2;203;166;247m gemini-2.5-pro \u{1b}[38;2;108;112;134m(97% \u{1b}[39m    \u{1b}[38;2;108;112;134m| \u{1b}[38;2;243;139;168m✖ 3 errors \u{1b}[38;2;108;112;134m(ctrl+o \u{1b}[39m\r\n\u{1b}[38;2;137;180;250m(feature/community-docs*)\u{1b}[39m               \u{1b}[38;2;243;139;168m/docs)\u{1b}[39m           \u{1b}[38;2;203;166;247mcontext left)\u{1b}[39m              \u{1b}[38;2;243;139;168mfor details)\u{1b}[39m\r\n";
    let cleaned_string_with_more_codes = String::from_utf8_lossy(&strip(raw_string_with_more_codes.as_bytes()).unwrap()).to_string();
    eprintln!("Raw (more codes): {:?}", raw_string_with_more_codes);
    eprintln!("Cleaned (more codes): {:?}", cleaned_string_with_more_codes);
}
