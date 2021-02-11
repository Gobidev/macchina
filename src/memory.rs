use crate::extra;

/// Read __MemTotal__ from `/proc/meminfo`
pub fn memtotal() -> u64 {
    let mem = String::from(
        extra::get_line_at("/proc/meminfo", 0, "Could not extract used MemTotal!").unwrap(),
    );
    // Collect only the value of MemTotal
    let s_mem_kb: String = mem.chars().filter(|c| c.is_digit(10)).collect();
    s_mem_kb.parse::<u64>().unwrap()
}

/// Read __MemFree__ from `/proc/meminfo`
pub fn memfree() -> u64 {
    let mem =
        String::from(extra::get_line_at("/proc/meminfo", 1, "Could not extract MemFree!").unwrap());
    // Collect only the value of MemFree
    let s_mem_kb: String = mem.chars().filter(|c| c.is_digit(10)).collect();
    s_mem_kb.parse::<u64>().unwrap()
}

/// Read __Buffers__ from `/proc/meminfo`
pub fn buffers() -> u64 {
    let mem =
        String::from(extra::get_line_at("/proc/meminfo", 3, "Could not extract Buffers!").unwrap());
    // Collect only the value of Buffers
    let s_mem_kb: String = mem.chars().filter(|c| c.is_digit(10)).collect();
    s_mem_kb.parse::<u64>().unwrap()
}

/// Read __Cached__ from `/proc/meminfo`
pub fn cached() -> u64 {
    let mem =
        String::from(extra::get_line_at("/proc/meminfo", 4, "Could not extract Cached!").unwrap());
    // Collect only the value of Cached
    let s_mem_kb: String = mem.chars().filter(|c| c.is_digit(10)).collect();
    s_mem_kb.parse::<u64>().unwrap()
}

/// Read __SReclaimable__ from `/proc/meminfo`
pub fn sreclaimable() -> u64 {
    let mem = String::from(
        extra::get_line_at("/proc/meminfo", 23, "Could not extract SReclaimable!").unwrap(),
    );
    // Collect only the value of SReclaimable
    let s_mem_kb: String = mem.chars().filter(|c| c.is_digit(10)).collect();
    s_mem_kb.parse::<u64>().unwrap()
}

/// Calculate memory utilization,
/// used = memtotal - memfree - cached - sreclaimable - buffers
pub fn used() -> u64 {
    memtotal() - memfree() - cached() - sreclaimable() - buffers()
}
