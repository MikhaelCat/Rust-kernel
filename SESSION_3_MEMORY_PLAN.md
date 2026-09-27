# Сессия 3: Memory Management (Управление Памятью)

## План Развития Модуля

### Текущее Состояние ✅
- `mm/phys.rs` - Physical Memory Manager готов
- Базовая структура PageFrame
- Allocation/deallocation через free list
- Utilization tracking

### Что Нужно Реализовать 📋

#### 1. Virtual Memory Manager (VMA)
```rust
// Подсистема виртуальной памяти
- VMA (Virtual Memory Area) management
- Page table entries  
- mmap/munmap системные вызовы
- Memory mapping для процессов
```

#### 2. Slab Allocator (kmalloc)
```rust
// slab allocator для маленьких объектов
- Red-black tree for cache management
- Slab classes (48B, 96B, 192B, etc.)
- Per-CPU caches
- Freelist management
```

#### 3. Swap Space Support
```rust
// Обмен страницами на disk
- Swap area management
- Page-out/Page-in algorithms
- Swap cache
- Swappiness control
```

#### 4. Page Fault Handling
```rust
// Обработка page faults
- Major vs Minor faults
- Copy-on-Write support
- Demand paging
- OOM killer integration
```

#### 5. Memory Cgroups
```rust
// Ограничения памяти для cgroups
- Memory limit enforcement
- Accounting per cgroup
- OOM notification
```

---

## Структура Файлов

```
src/mm/
├── mod.rs                    # Модуль exports
├── error.rs                  # Ошибки управления памятью
├── phys.rs                   # ✅ Физическая память менеджер (Готов)
├── vm.rs                     # ⏳ Виртуальная память (Нужно написать)
├── page_table.rs             # ⏳ Таблицы страниц
├── kmalloc.rs                # ❌ Slab allocator (Не начато)
├── swap.rs                   # ❌ Swap support (Не начато)
├── fault.rs                  # ❌ Page fault handling (Нужно дописать)
├── vma.rs                    # ❌ VMA management (Не начато)
├── page_alloc.rs             # ❌ Расширенный page allocator (Нужно)
├── cow.rs                    # ✅ Copy-on-Write skeleton (Минимальный)
├── memcg.rs                  # ✅ Memcgs skeleton (Минимальный)
├── oom.rs                    # ❌ OOM killer (Нужно)
├── cma.rs                    # ✅ Contiguous Memory Allocator (Базово)
├── hugepage.rs               # ❌ Huge pages (Нужно)
├── compaction.rs             # ❌ Memory compaction (Нужно)
└── ... (другие вспомогательные файлы)
```

---

## Приоритеты Разработки

### 🔴 Критически важно (высокий приоритет):
1. **VM Management** - базовые функции mmap/virtual memory
2. **Slab Allocator** - эффективная аллокация мелких объектов
3. **Page Table** - управление таблицами страниц

### 🟡 Важное (средний приоритет):
4. **Swap Support** - обмен страницами
5. **Page Fault Handler** - обработка fault'ов
6. **OOM Killer** - защита от исчерпания памяти

### 🟢 Желательное (низкий приоритет):
7. Huge Pages
8. Memory Compaction
9. KSM (Kernel Same-page Merging)

---

## Примеры Использования

### Выделение памяти:

```rust
// Физическая память
let mut mm = PhysicalMemoryManager::new(16 * 1024 * 1024, 4096)?; // 16MB
let page1 = mm.allocate_page()?;      // Получить 4KB страницу
let page2 = mm.allocate_page()?;      // Еще одну
mm.free_page(page1)?;                  // Освободить обратно

// Виртуальная память
let vma = Vma::new(start_addr, length, flags);
mm.map_page(vma, phys_page)?;          // Связать виртуальную с физической

// kmalloc (slab allocator)
let buffer: Box<[u8; 256]> = kmalloc(size)?;  // Выделить из кэша
kfree(buffer);                             // Освободить
```

---

## Архитектура Memory Subsystem

```
┌─────────────────────────────────────────────────────────┐
│              User Space (mmap, malloc)                  │
└──────────────────────────┬──────────────────────────────┘
                           ↓
┌──────────────────────────▼──────────────────────────────┐
│           Virtual Memory Manager (VMM)                   │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────┐  │
│  │   mmap()    │  │   munmap()  │  │   mprotect()    │  │
│  └─────────────┘  └─────────────┘  └─────────────────┘  │
└──────────────────────────┬──────────────────────────────┘
                           ↓
┌──────────────────────────▼──────────────────────────────┐
│         Page Table Manager & TLB                        │
│  - L1/L2 Page Tables     - PTE management               │
│  - Address Translation   - TLB flush                    │
└──────────────────────────┬──────────────────────────────┘
                           ↓
        ┌──────────────────┴──────────────────┐
        ↓                                     ↓
┌───────────────┐                     ┌───────────────┐
│ Slab Allocator│                     │Physical Memory│
│ (kmalloc/kfree)│                    │  Manager       │
│               │                     │               │
│ Per-size caches│                    │ Free list      │
│ Red-black tree │                    │ Page frames    │
└───────────────┘                     └───────────────┘
```

---

## Следующий Шаг

Сейчас будем реализовывать **VirtuaL Memory Manager** и **Slab Allocator**.

Эти два компонента критичны для работы ядра - без них невозможно создание процессов с изолированной памятью.

---

## Статус Завершенности

| Компонент | Статус | Прогресс |
|-----------|--------|----------|
| Physical Memory | ✅ Готов | 100% |
| Virtual Memory (VMA) | ⏳ Написать | 0% |
| Slab Allocator | ❌ Не начато | 0% |
| Page Tables | ⏳ Нужно | 0% |
| Swap Support | ❌ Не начато | 0% |
| Page Faults | ⏳ Улучшить | 20% |

**Общий прогресс:** ~25% из запланированного
