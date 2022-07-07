pub mod test_car {

    use crate::models::*;

    #[test]
    fn test_car_update_works() {
        let mut car = Car::new(100., 100., 60., 80.);
        
        assert_eq!(car.config.speed, 0.);
        
        car.set_forward(true);
        car.update();

        assert!(car.config.speed != 0.);

        car.set_forward(false);
        car.set_reverse(true);

        car.update();
        car.update();

        assert!(car.config.speed != 0.);
    }
}