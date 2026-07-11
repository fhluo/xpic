mod app 'crates/xpic-app/justfile'
mod cli 'crates/xpic/justfile'
mod spotlight 'crates/spotlight/justfile'

set shell := ["nu", "-c"]

default:
  @just --list

build *args: (cli::build args) (spotlight::build args) (app::build args)

release: (build "--release")
  mkdir bin
  mv target/release/xpic.exe bin/xpic-cli.exe
  mv target/release/xpic-app.exe bin/Xpic.exe

[group: 'package']
[working-directory: 'build']
[windows]
iscc:
    #!nu
    let inno = (
      ["C:\\Program Files\\Inno Setup 7", $"($env.LOCALAPPDATA)\\Programs\\Inno Setup 7"]
      | where {|p| $p | path exists }
      | first
    )

    if ($inno != null) {
      $env.Path = ($env.Path | prepend $inno)

      let lang = ($inno | path join "Languages" "ChineseSimplified.isl")
      if not ($lang | path exists) {
        print $"(ansi light_gray)Downloading Simplified Chinese translation for Inno Setup...(ansi reset)"
        http get "https://raw.githubusercontent.com/kira-96/Inno-Setup-Chinese-Simplified-Translation/main/ChineseSimplified.isl"
        | save $lang
        print $"(ansi green)✓ Saved to ($lang)(ansi reset)"
      }
    }

    ISCC Xpic.iss
