RustHound (rusthound)
=====================

Reads log files and reports lines that match rules or unusual activity. This archive holds the `rusthound` command-line tool, built from the tagged release.

Install
-------
Put the `rusthound` binary (`rusthound.exe` on Windows) in a directory on your PATH,
or use the installer, which does that for you and checks the checksum:

  macOS / Linux:  curl -fsSL https://raw.githubusercontent.com/rustfuture/RustHound/main/install.sh | sh
  Windows:        irm https://raw.githubusercontent.com/rustfuture/RustHound/main/install.ps1 | iex

Then:

  rusthound --version
  rusthound --help

Verify this download
--------------------
Each archive has a matching .sha256 file on the release page:

  sha256sum -c rusthound-<version>-<target>.tar.gz.sha256      (Linux)
  shasum -a 256 -c rusthound-<version>-<target>.tar.gz.sha256  (macOS)

Docs and source: https://github.com/rustfuture/RustHound
License: see LICENSE
