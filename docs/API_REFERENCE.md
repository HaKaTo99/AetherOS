# AetherOS API Reference

**Version:** 1.9  
**Last Updated:** February 1, 2026

This document serves as an overview of the AetherOS API documentation.

---

## Generating Documentation

To generate the full API documentation locally:

```bash
cd kernel
cargo doc --no-deps --document-private-items --open
```

This will:
1. Generate rustdoc HTML for all modules
2. Open the documentation in your default browser
3. Create documentation at `target/doc/aetheros_kernel/index.html`

---

## Key Modules

### Memory Management (`memory::smme`)
**Symbian-Modern Memory Engine** - Production-ready allocator

- `SymbianModernMemoryEngine` - Main allocator struct
- `allocate()` / `deallocate()` - Memory allocation API
- `stats()` - Usage statistics

**Example**:
```rust
use aetheros_kernel::SMME;

let ptr = unsafe { SMME.allocate(4096).unwrap() };
// Use memory...
unsafe { SMME.deallocate(ptr, 4096); }
```

---

### Scheduler (`scheduler::active_objects`)
**Active Object Scheduler** - Priority-based multitasking

- `ActiveObjectScheduler` - Main scheduler
- `ActiveObject` - Task structure
- `schedule()` / `yield_task()` - Scheduling operations

**Example**:
```rust
use aetheros_kernel::scheduler::ActiveObject;

let task = ActiveObject::new(1, 128, 0); // id, priority, process_id
```

---

### Distributed Computing (`distributed`)
**Distributed infrastructure** - Migration, storage, balancing

- `MigrationManager` - Task migration
- `KvStore` - Distributed key-value storage
- `LoadBalancer` - Metrics-based balancing

**Example**:
```rust
use aetheros_kernel::distributed::{MIGRATION_MANAGER, KV_STORE};

unsafe {
    MIGRATION_MANAGER.init();
    KV_STORE.put("key".into(), vec![1, 2, 3]);
}
```

---

### IPC (`ipc::qc`)
**Quantum Channel RPC** - Cross-device communication

- `RpcMethod` - RPC method enumeration
- `RpcMessage` - Message structure
- `serialize()` / `deserialize()` - Protocol handling

**Example**:
```rust
use aetheros_kernel::ipc::qc::{RpcMethod, RpcMessage};

let msg = RpcMessage {
    method: RpcMethod::Ping,
    payload: vec![],
};
```

---

### UI Framework (`ui`)
**User Interface** - Widgets and layout

- `Widget` trait - Base widget interface
- `Label`, `Button`, `Panel` - UI components
- `FlexLayout` - Layout engine

**Example**:
```rust
use aetheros_kernel::ui::{Label, Rect};

let label = Label::new(Rect::new(0, 0, 100, 20), "Hello");
```

---

## Documentation Standards

All public APIs follow these conventions:

1. **Module docs**: Every `mod.rs` has comprehensive module-level documentation
2. **Examples**: All public functions include practical examples
3. **Safety**: `unsafe` functions document safety requirements
4. **Links**: Cross-references to related modules

---

## Online Documentation

*Coming soon: GitHub Pages hosted rustdoc*

For now, generate locally with `cargo doc --open`

---

## Further Reading

- **Developer Guide**: `docs/DEVELOPER_GUIDE.md` - Build and contribute
- **Deployment Guide**: `docs/DEPLOYMENT_GUIDE.md` - Hardware deployment
