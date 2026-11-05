use sigmaos_gluebuddy::*;

#[test]
fn component_is_constructible_and_runnable() {
    // Arrange
    let c = sigmaos_gluebuddy::Gluebuddy::new("sigmaos-gluebuddy").expect("valid name");
    // Act
    let out = c.run().expect("run succeeds");
    // Assert
    assert!(out.contains("sigmaos-gluebuddy"));
}
