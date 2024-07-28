use my_crate::add;
mod shared;
use shared::launch_browser;

#[tokio::test]
async fn foo() {
    let two = add(1, 1);
    assert_eq!(two, 2);
    launch_browser().await.unwrap();
}
