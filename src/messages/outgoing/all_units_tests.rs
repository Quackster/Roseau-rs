use super::*;

#[test]
fn composes_all_units_packet() {
    let mut response = AllUnits::new(
        "127.0.0.1",
        [PublicUnit::new("Lido", 2, 25, 37120, "pool_a", "model_a")],
    )
    .compose();

    assert_eq!(
        response.get(),
        "#ALLUNITS\rLido,2,25,127.0.0.1/127.0.0.1,37120,Lido\tpool_a,2,25,model_a##"
    );
}
