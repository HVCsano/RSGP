default:
    @just --list

set windows-powershell

[windows]
web:
    cd web; deno task dev

[linux]
web:
    cd web && deno task dev

[windows]
mg:
    cd crates/manager; cargo watch -x run

[linux]
mg:
    cd crates/manager && cargo watch -x run