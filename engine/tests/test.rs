#[macro_use]
extern crate time_test;
extern crate goscript_engine as engine;

fn run(path: &str, trace: bool) -> usize {
    let cfg = engine::Config {
        work_dir: Some("./".to_string()),
        base_path: Some("./std/".to_string()),
        trace_parser: trace,
        trace_checker: trace,
        trace_vm: true,
    };
    let engine = engine::Engine::new(cfg);
    engine.run(path)
}

#[test]
fn test_g2case0() {
    let err_cnt = run("./tests/group2/case0.gos", true);
    assert!(err_cnt == 0);
}

#[test]
fn test_g2case1() {
    let err_cnt = run("./tests/group2/case1.gos", true);
    assert!(err_cnt == 0);
}

#[test]
fn test_g2case2() {
    let err_cnt = run("./tests/group2/case2.gos", true);
    assert!(err_cnt == 0);
}

#[test]
fn test_g2case3() {
    let err_cnt = run("./tests/group2/case3.gos", true);
    assert!(err_cnt == 0);
}

#[test]
fn test_g2nil() {
    let err_cnt = run("./tests/group2/nil.gos", true);
    assert!(err_cnt == 0);
}

#[test]
fn test_g2display() {
    let err_cnt = run("./tests/group2/display.gos", true);
    assert!(err_cnt == 0);
}

#[test]
fn test_unicode0() {
    time_test!();

    let err_cnt = run("./tests/group2/unicode0.gos", true);
    assert!(err_cnt == 0);
}

#[test]
fn test_sync_mutex() {
    let err_cnt = run("./tests/group2/sync_mutex.gos", true);
    assert!(err_cnt == 0);
}

#[test]
fn test_g1case1() {
    let err_cnt = run("./tests/group1/case1.gos", true);
    assert!(err_cnt == 0);
}

#[test]
fn test_closure1() {
    let err_cnt = run("./tests/group1/closure1.gos", true);
    assert!(err_cnt == 0);
}

#[test]
fn test_closure2() {
    let err_cnt = run("./tests/group1/closure2.gos", true);
    assert!(err_cnt == 0);
}

#[test]
fn test_closure3() {
    let err_cnt = run("./tests/group1/closure3.gos", true);
    assert!(err_cnt == 0);
}

#[test]
fn test_closure4() {
    let err_cnt = run("./tests/group1/closure4.gos", true);
    assert!(err_cnt == 0);
}

#[test]
fn test_func1() {
    let err_cnt = run("./tests/group1/func1.gos", true);
    assert!(err_cnt == 0);
}

#[test]
fn test_blankid() {
    let err_cnt = run("./tests/group1/blankid.gos", true);
    assert!(err_cnt == 0);
}

#[test]
fn test_declare() {
    let err_cnt = run("./tests/group1/declare.gos", true);
    assert!(err_cnt == 0);
}

#[test]
fn test_slice1() {
    let err_cnt = run("./tests/group1/slice1.gos", true);
    assert!(err_cnt == 0);
}

#[test]
fn test_map1() {
    let err_cnt = run("./tests/group1/map1.gos", true);
    assert!(err_cnt == 0);
}

#[test]
fn test_pkg() {
    let err_cnt = run("./tests/group1/pkg.gos", true);
    assert!(err_cnt == 0);
}

#[test]
fn test_type() {
    let err_cnt = run("./tests/group1/type.gos", true);
    assert!(err_cnt == 0);
}

#[test]
fn test_pointer() {
    let err_cnt = run("./tests/group1/pointer.gos", true);
    assert!(err_cnt == 0);
}

#[test]
fn test_operations() {
    let err_cnt = run("./tests/group1/operations.gos", true);
    assert!(err_cnt == 0);
}

#[test]
fn test_if() {
    let err_cnt = run("./tests/group1/if.gos", true);
    assert!(err_cnt == 0);
}

#[test]
fn test_for() {
    let err_cnt = run("./tests/group1/for.gos", true);
    assert!(err_cnt == 0);
}

#[test]
fn test_interface() {
    let err_cnt = run("./tests/group1/interface.gos", true);
    assert!(err_cnt == 0);
}

#[test]
fn test_interface2() {
    let err_cnt = run("./tests/group1/interface2.gos", true);
    assert!(err_cnt == 0);
}

#[test]
fn test_initorder() {
    let err_cnt = run("./tests/group1/initorder.gos", true);
    assert!(err_cnt == 0);
}

#[test]
fn test_switch() {
    let err_cnt = run("./tests/group1/switch.gos", true);
    assert!(err_cnt == 0);
}

#[test]
fn test_typeswitch() {
    let err_cnt = run("./tests/group1/typeswitch.gos", true);
    assert!(err_cnt == 0);
}

#[test]
fn test_basictypes() {
    let err_cnt = run("./tests/group1/basictypes.gos", true);
    assert!(err_cnt == 0);
}

#[test]
fn test_composite() {
    let err_cnt = run("./tests/group1/composite.gos", true);
    assert!(err_cnt == 0);
}

#[test]
fn test_array() {
    let err_cnt = run("./tests/group1/array.gos", true);
    assert!(err_cnt == 0);
}

#[test]
fn test_conversion() {
    let err_cnt = run("./tests/group1/conversion.gos", true);
    assert!(err_cnt == 0);
}

#[test]
fn test_async() {
    let err_cnt = run("./tests/group1/async.gos", true);
    assert!(err_cnt == 0);
}

#[test]
fn test_channel() {
    let err_cnt = run("./tests/group1/channel.gos", true);
    assert!(err_cnt == 0);
}

#[test]
fn test_defer() {
    let err_cnt = run("./tests/group1/defer.gos", true);
    assert!(err_cnt == 0);
}

#[test]
fn test_goto() {
    let err_cnt = run("./tests/group1/goto.gos", true);
    assert!(err_cnt == 0);
}

#[test]
fn test_recover() {
    let err_cnt = run("./tests/group1/recover.gos", true);
    assert!(err_cnt == 0);
}

#[test]
fn test_select() {
    let err_cnt = run("./tests/group1/select.gos", true);
    assert!(err_cnt == 0);
}

#[test]
fn test_linked() {
    let err_cnt = run("./tests/demo/linked.gos", true);
    assert!(err_cnt == 0);
}

#[test]
fn test_leetcode5() {
    time_test!();

    let err_cnt = run("./tests/demo/leetcode5.gos", true);
    assert!(err_cnt == 0);
}

#[test]
fn test_fibonacci() {
    time_test!();

    let err_cnt = run("./tests/demo/fibonacci.gos", true);
    assert!(err_cnt == 0);
}

#[test]
fn test_issue8() {
    time_test!();

    let err_cnt = run("./tests/issues/issue8.gos", true);
    assert!(err_cnt == 0);
}
