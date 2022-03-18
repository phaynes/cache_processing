/// 
/// Process to test CPU processing on a Mac.
///

extern crate sysctl;

use cachelines::cpustats::CPUStats;

fn main() {
    let mut cpu_stats = CPUStats::new();
    cpu_stats.out();
}
