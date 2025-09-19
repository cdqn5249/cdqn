// src/runtime/mod.rs

pub mod persistence;```

4.  Now, inside `src/runtime/`, create the file for our new service: `persistence.rs`.

---

### **Step 2: Declare the New `runtime` Module**

We need to tell our main library (`lib.rs`) about the new `runtime` module.

1.  Navigate to `src/lib.rs` and click "Edit".
2.  Add `pub mod runtime;` to the file. The whole file should now look like this:

```rust
// src/lib.rs

pub mod kernel;
pub mod runtime;
