extern crate nalgebra as na;
extern crate ndarray as nd;

    /// Returns true if a value is less than some small threshold
    /// 
    /// # Arguments: 
    /// 
    /// * `value` - a f64 value
    /// 
    /// # Returns:
    /// 
    /// * `bool` - true or false
    pub fn near_zero(value : f64) -> bool {
        return value.abs() < 0.000001;
    }

    // need inverse and normalize matrix from nalgebra

    pub fn vec_to_so3(vec: na::Vector3<f64>) -> na::Matrix3<f64> {
        let so3 = na::Matrix3::new(
            0.0,    -vec[2], vec[1],
            vec[2],  0.0,    -vec[0],
            -vec[1], vec[2], 0.0      
        );

        return so3;
    }

    pub fn so3_to_vec(so3mat : na::Matrix3<f64>) -> na::Vector3<f64> {
        //how the hell does array indexing work? Am reading nalgebra repo and can't find anything
        let vec = vec![so3mat(2, 1), so3mat(0, 2), so3mat(1, 0)];
        return vec;
    }

    pub fn axis_to_angle3(axis : na::Vector3<f64>) -> (na::Vector3<f64>, f64) {
        return(normalize(axis), axis.norm());
    }




///Computes the matrix exponential of a matrix in so(3).
///
/// 
pub fn matrix_to_exp3(so3mat: na::Matrix3<f64>) {
    let omgtheta = so3_to_vec(so3mat);
    if near_zero(omgtheta.norm()) {
        //return the identity matrix of the same size as omgtheta
    }
    else {
        let theta = axis_to_angle3(omgtheta)[2];
        let omgmat = so3mat / theta;
        //return linalg.I + sin(theta) * omgmat + (1 - cos(theta)) * omgmat * omgmat;
    }
}


    #[test]
    fn test_vec_to_so3() {
        let vector = na::Vector3::new(21.0,21.0,21.0);

        //need to figure out how to call from other modules
        let vec_trans = vec_to_so3(vector);
        println!("{}", vec_trans);
    }