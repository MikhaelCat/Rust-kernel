# boot

Boot pipeline modules include:
- cmdline parse/sanitize/validate
- memory map validation
- ELF loader + relocation + handoff
- firmware/DTB/ACPI handoff helpers
- boot policy contract + self-check

Contract highlights:
- `console=` must be present in cmdline
- memory map must be valid and at least 1 MiB
- kernel image must be present before final handoff

Main docs: ../../Documentation/boot/overview.md
