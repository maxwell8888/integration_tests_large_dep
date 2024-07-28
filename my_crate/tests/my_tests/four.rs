use my_crate::add;
use super::shared::launch_browser;

#[tokio::test]
pub async fn foo() {
    let two = add(1, 1);
    assert_eq!(two, 2);
    launch_browser().await.unwrap();
}
