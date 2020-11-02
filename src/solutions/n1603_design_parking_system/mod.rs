/// https://leetcode.com/problems/design-parking-system/
///
/// https://leetcode-cn.com/problems/design-parking-system/
struct ParkingSystem {
    capacities: Vec<i32>,
}

impl ParkingSystem {
    fn new(big: i32, medium: i32, small: i32) -> Self {
        Self {
            capacities: vec![big, medium, small],
        }
    }

    fn add_car(&mut self, car_type: i32) -> bool {
        if self.capacities[(car_type - 1) as usize] == 0 {
            return false;
        }
        self.capacities[(car_type - 1) as usize] -= 1;
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_design_parking_system() {
        let mut parking_system = ParkingSystem::new(1, 1, 0);
        assert_eq!(parking_system.add_car(1), true);
        assert_eq!(parking_system.add_car(2), true);
        assert_eq!(parking_system.add_car(3), false);
        assert_eq!(parking_system.add_car(1), false);
    }
}
