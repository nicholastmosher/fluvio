use proc_macro::TokenStream;
use proc_macro2::Span;

//use syn::{AttributeArgs, ItemFn, NestedMeta, parse_macro_input, Ident};
use syn::{AttributeArgs, ItemFn, parse_macro_input, Ident};
use quote::quote;

#[proc_macro_attribute]
pub fn fluvio_test(args: TokenStream, input: TokenStream) -> TokenStream {

    println!("args: \"{}\"", args.to_string());
    println!("input: \"{}\"", input.to_string());

    // Read the test input
    let _macro_args = parse_macro_input!(args as AttributeArgs);

    // Validate inputs by creating TestOption
    //let test_details = validate_test_details(macro_args);

    //// Read the user test
    let fn_user_test = parse_macro_input!(input as ItemFn);

    let test_name = &fn_user_test.sig.ident;
    let test_body = &fn_user_test.block;


    let out_fn_iden = Ident::new(&test_name.to_string(), Span::call_site());

    // TODO: Do we want this to be given via attributes, or randomized?
    let namespace: String = "default".to_string(); // May be UUID or pseudo-random unique english name

    let output_fn = quote! {

        fn #out_fn_iden() {

            fn cleanup_cluster() {
                run_block_on(async move {
                    sleep(Duration::from_secs(60)).await;

                    // Delete cluster
                    let uninstaller = ClusterUninstaller::new()
                        .with_namespace(#namespace)
                        .build().unwrap();
                    uninstaller.uninstall().await.expect("");
                });
            }

            // Build a test meta struct

            // Initialize kubernetes
            use fluvio_future::task::run_block_on;
            use fluvio_future::timer::sleep;
            use fluvio_cluster::{ClusterConfig, ClusterConfigBuilder, ClusterError, ClusterInstaller};
            use fluvio::{Fluvio, FluvioConfig};
            use fluvio_cluster::ClusterUninstaller;
            use std::time::Duration;
            //use fluvio_test_framework::TestDetails;


            let namespace: String = #namespace.to_string(); // May be UUID or pseudo-random unique english name
            let cluster_config: ClusterConfig = fluvio_cluster::ClusterConfig::builder("0.7.0-alpha.5")
                .namespace(&namespace)
                // .add_any_other_configurations()
                .build()
                .unwrap();
            let installer = ClusterInstaller::from_config(cluster_config).unwrap();

            run_block_on(async move {

                let status = installer.install_fluvio().await.expect("");
                let fluvio_config = FluvioConfig::new(status.address());
                let fluvio = Fluvio::connect_with_config(&fluvio_config).await.unwrap();

            });


            // TODO: We want the option to skip clean up if the test fails
            std::panic::set_hook(Box::new(|panic_info| {
                cleanup_cluster();
            }));    


            // Write the user's test content
            // (concern: Will rust-analyzer complain about vars that don't exist before compile?
            //  Bc we are going to need to "magic" a fluvio client struct and a test option struct
            //  for test writers to use)

            #test_body

            // Clean up (let this be configurable)
            run_block_on(async move {
                cleanup_cluster();
            });


            //assert!(test_result.is_ok())

        }

    };

    //println!("{:?}", output_fn.to_string());

    output_fn.into()
    //input
}

//fn validate_test_details(args: Vec<NestedMeta>) -> TestDetails {
//
//    TestDetails {}
//}

//#[derive(Debug, Clone, StructOpt)]
//pub struct ProductOption {
//    /// number of produce iteration for a single producer
//    #[structopt(short, long, default_value = "1")]
//    pub produce_iteration: u16,
//
//    // record size
//    #[structopt(long, default_value = "100")]
//    pub record_size: usize,
//
//    // number of parallel producer
//    #[structopt(long, default_value = "1")]
//    pub producer_count: u16,
//}
//
///// cli options
//#[derive(Debug, Clone, StructOpt)]
//#[structopt(name = "fluvio-test-runner", about = "Test fluvio platform")]
//pub struct TestOption {
//    #[structopt(flatten)]
//    pub produce: ProductOption,
//
//    /// don't install and uninstall
//    #[structopt(short, long)]
//    disable_install: bool,
//
//    /// don't produce message
//    #[structopt(long)]
//    disable_produce: bool,
//
//    /// don't test consumer
//    #[structopt(long)]
//    disable_consume: bool,
//
//    #[structopt(short, long)]
//    /// replication count, number of spu will be same as replication count, unless overridden
//    replication: Option<u16>,
//
//    /// base topic name used.  if replication > 1, then topic name will be named: topic<replication>
//    #[structopt(short("t"), long, default_value = "topic")]
//    pub topic_name: String,
//
//    /// if this is turn on, consumer waits for producer to finish before starts consumption
//    /// if iterations are long then consumer may receive large number of batches
//    #[structopt(long)]
//    pub consumer_wait: bool,
//    /// number of spu
//    #[structopt(short, long, default_value = "1")]
//    pub spu: u16,
//
//    /// enable tls
//    #[structopt(long)]
//    tls: bool,
//
//    /// tls user, only used if tls is used
//    #[structopt(long, default_value = "root")]
//    pub tls_user: String,
//
//    /// run local environment
//    #[structopt(long)]
//    local: bool,
//
//    /// run develop image, this is for k8
//    #[structopt(long)]
//    develop: bool,
//
//    // log apply to fluvio client
//    #[structopt(long)]
//    pub client_log: Option<String>,
//
//    // log apply to fluvio
//    #[structopt(long)]
//    pub server_log: Option<String>,
//
//    // log dir
//    #[structopt(long)]
//    pub log_dir: Option<String>,
//
//    /// authorization ConfigMap
//    #[structopt(long)]
//    pub authorization_config_map: Option<String>,
//
//    /// skip pre-install checks
//    #[structopt(long)]
//    skip_checks: bool,
//}

//#[fluvio_cluster::test(
//    namespace = "...",
//    image_tag = "...",
//    chart_location = "...",
//    chart_version = "...",
//)]

// timeout (what units?)