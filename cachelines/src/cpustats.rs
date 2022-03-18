///
/// Retreive basic CPU statistics.
///

extern crate sysctl;

static ARM_TYPE: &str               = "arm64";
static HW_MACHINE: &str             = "hw.machine";
static HW_MEM_SIZE: &str            = "hw.memsize";
static HW_CACHE_LINE_SIZE: &str     = "hw.cachelinesize";
static HW_PHYS_CPU_COUNT: &str      = "hw.ncpu";
static HW_PHYS_PERF_CPU_COUNT: &str = "hw.perflevel0.physicalcpu";
static HW_L1_CACHE_SIZE: &str       = "hw.perflevel0.l1dcachesize";
static HW_L2_CACHE_SIZE: &str       = "hw.perflevel0.l2cachesize";

// Import the trait
use sysctl::Sysctl;

/// CPU statistics to collect.
pub struct CPUStats {
    pub is_arm          : bool,
    pub mem_size        : i64,
    pub cpu_count       : i32,
    pub perf_core_count : i32,
    pub cache_line_size : i32,
    pub l1_cache_size   : i32,
    pub l2_cache_size   : i32,
}

/// Provides basic CPU statistics 
impl CPUStats {

    pub fn read_long_value(key : &str) -> i64 {
        let ctl = sysctl::Ctl::new(key).unwrap();
        let value_str = ctl.value_string().unwrap();
        let value = value_str.trim().parse::<i64>().unwrap();
        value
    }

    pub fn read_value(key : &str) -> i32 {
        let ctl = sysctl::Ctl::new(key).unwrap();
        let value_str = ctl.value_string().unwrap();
        let value = value_str.trim().parse::<i32>().unwrap();
        value
    }

    pub fn new() -> CPUStats {
        let ctl = sysctl::Ctl::new(HW_MACHINE).unwrap();
        let cpu_type = ctl.value_string().unwrap();

        CPUStats {
            is_arm          : cpu_type.eq_ignore_ascii_case(ARM_TYPE),
            mem_size        : CPUStats::read_long_value(HW_MEM_SIZE),
            cpu_count       : CPUStats::read_value(HW_PHYS_CPU_COUNT),
            perf_core_count : CPUStats::read_value(HW_PHYS_PERF_CPU_COUNT),
            cache_line_size : CPUStats::read_value(HW_CACHE_LINE_SIZE),
            l1_cache_size   : CPUStats::read_value(HW_L1_CACHE_SIZE),
            l2_cache_size   : CPUStats::read_value(HW_L2_CACHE_SIZE)
        }
    }

   pub fn out(&mut self) {
        println!("is_arm: {}", self.is_arm);
        println!("memory_size: {} MB", self.mem_size/(1024*1024));
        println!("cpu_count: {} ", self.cpu_count);
        println!("perf_cores: {} ", self.perf_core_count);
        println!("cache_line_size: {} ", self.cache_line_size);
        println!("l1_cache_size: {}KB ", self.l1_cache_size/1024);
        println!("l2_cache_size: {}MB ", self.l2_cache_size/(1024*1024));
    }
}