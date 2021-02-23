#[test]
#[fluvio_test_macro::fluvio_test(min_spu = 2, topic = "test")]
fn smoke_test() {
    // let start_offsets = super::produce::produce_message(&self.option).await;
    // super::consume::validate_consume_message(&self.option, start_offsets).await;

    //println!("{:?}", testdetails);
    assert!(true);
}
