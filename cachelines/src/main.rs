/// 
/// Process to test CPU processing on a mac
///

extern crate sysctl;

use cachelines::cpustats::CPUStats;

fn main() {
    let mut cpu_stats = CPUStats::new();
    cpu_stats.out();
}