# План развития Linux-ядра на Rust - Секция 6/15
## Блочные устройства (Block Device I/O)

### Текущее состояние:
✅ Block device abstraction
✅ Request queue management
✅ Bio (Block I/O) structures
✅ Basic disk I/O scheduler

### Задачи для реализации:
1. Multi-queue block layer ( blk-mq)
2. Elevator algorithms (deadline, noop, cfq)
3. NVMe driver implementation
4. SCSI subsystem
5. RAID controller support
6. IO_uring integration for async I/O

### Приоритеты:
- [ ] High: blk-mq implementation
- [ ] High: NVMe driver
- [ ] Medium: RAID support
- [ ] Low: Disk utilities
