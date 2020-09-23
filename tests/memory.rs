// Copyright (c) 2020 And Group
//
// SPDX-License-Identifier: Apache-2.0 or MIT
//

//! Integration tests about the hugetlb subsystem
use cgroups::memory::{MemController, SetMemory};
use cgroups::Controller;
use cgroups::{Cgroup, MaxValue};

#[test]
fn test_disable_oom_killer() {
    let h = cgroups::hierarchies::auto();
    let h = Box::new(&*h);
    let cg = Cgroup::new(h, String::from("test_disable_oom_killer"));
    {
        let mem_controller: &MemController = cg.controller_of().unwrap();

        // before disable
        let m = mem_controller.memory_stat();
        assert_eq!(m.oom_control.oom_kill_disable, false);

        // now only v1
        if !mem_controller.v2() {
            // disable oom killer
            let r = mem_controller.disable_oom_killer();
            assert_eq!(r.is_err(), false);

            // after disable
            let m = mem_controller.memory_stat();
            assert_eq!(m.oom_control.oom_kill_disable, true);
        }
    }
    cg.delete();
}

#[test]
fn set_mem_v2() {
    let h = cgroups::hierarchies::auto();
    if !h.v2() {
        return;
    }

    let h = Box::new(&*h);
    let cg = Cgroup::new(h, String::from("set_mem_v2"));
    {
        let mem_controller: &MemController = cg.controller_of().unwrap();

        // before disable
        let m = mem_controller.get_mem().unwrap();
        // case 1: get default value
        assert_eq!(m.low, Some(MaxValue::Value(0)));
        assert_eq!(m.min, Some(MaxValue::Value(0)));
        assert_eq!(m.high, Some(MaxValue::Max));
        assert_eq!(m.max, Some(MaxValue::Max));

        // case 2: set parts
        let m = SetMemory {
            low: Some(MaxValue::Value(1024 * 1024 * 2)),
            high: Some(MaxValue::Value(1024 * 1024 * 1024 * 2)),
            min: Some(MaxValue::Value(1024 * 1024 * 3)),
            max: None,
        };
        let r = mem_controller.set_mem(m);
        assert_eq!(true, r.is_ok());

        let m = mem_controller.get_mem().unwrap();
        // get
        assert_eq!(m.low, Some(MaxValue::Value(1024 * 1024 * 2)));
        assert_eq!(m.min, Some(MaxValue::Value(1024 * 1024 * 3)));
        assert_eq!(m.high, Some(MaxValue::Value(1024 * 1024 * 1024 * 2)));
        assert_eq!(m.max, Some(MaxValue::Max));

        // case 3: set parts
        let m = SetMemory {
            max: Some(MaxValue::Value(1024 * 1024 * 1024 * 2)),
            min: Some(MaxValue::Value(1024 * 1024 * 4)),
            high: Some(MaxValue::Max),
            low: None,
        };
        let r = mem_controller.set_mem(m);
        assert_eq!(true, r.is_ok());

        let m = mem_controller.get_mem().unwrap();
        // get
        assert_eq!(m.low, Some(MaxValue::Value(1024 * 1024 * 2)));
        assert_eq!(m.min, Some(MaxValue::Value(1024 * 1024 * 4)));
        assert_eq!(m.max, Some(MaxValue::Value(1024 * 1024 * 1024 * 2)));
        assert_eq!(m.high, Some(MaxValue::Max));
    }

    cg.delete();
}