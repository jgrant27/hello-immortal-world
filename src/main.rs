#[fort::root(redundancy = 64)]
async fn main(bctx: bastion::context::BastionContext) -> Result<(), ()> {
    loop {
        println!("Hello Immortal World from {:?} !\n", &bctx.current().id());
        panic!("Why won't you die ?!")
    }
}
