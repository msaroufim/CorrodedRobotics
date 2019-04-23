extern crate nalgebra as na;
extern crate ndarray as nd;
 
//Helper functions

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


// Chapter 3: Rigid body motions

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
        //this function def has a bug
        let vec = vec![so3mat(2, 1), so3mat(0, 2), so3mat(1, 0)];
        return vec;
    }

    pub fn axis_to_angle3(axis : na::Vector3<f64>) -> (na::Vector3<f64>, f64) {
        return(axis.normalize(), axis.norm());
    }

    pub fn rot_inverse(rot: na::Matrix3<f64>) {
        rot.try_inverse();
    }



///Computes the matrix exponential of a matrix in so(3).
///
/// 
pub fn matrix_to_exp3(so3mat: na::Matrix3<f64>) -> na::Matrix3<f64> {
    let omgtheta = so3_to_vec(so3mat);
    if near_zero(omgtheta.norm()) {
        
        //do I need to specify the size explicly?
        return na::Matrix<f64>::identity();
    }
    else {
        let theta = axis_to_angle3(omgtheta)[2];
        let omgmat = so3mat / theta;

        //not calling identity correctly
        return na::Matrix<f64>::identity() + theta.sin() * omgmat + (1 - theta.cos()) * omgmat * omgmat;
    }
}

pub fn rot_matrix_to_log3(rotmat: na::Matrix3<f64>) -> na::Matrix3<f64> { 
    unimplemented!();
}

pub fn rot_and_pos_to_homogeneous(rot : na::Matrix3<f64>, pos: na::Vector3<f64>) -> na::Matrix4<f64>  {
    //nalgebra doesn't support vcat or hcat, I may need to do this myself
    //vcat(hcat(R, p), [0 0 0 1])
    unimplemented!();

}

pub fn homogeneous_to_rot_and_post(hom: na::Matrix4<f64>) -> (na::Matrix3<f64>, na::Vector3<f64>) {
    unimplemented!();
}

pub fn invert_homogeneous(rot: na::Matrix3<f64>) -> na::Matrix3<f64> {
    unimplemented!();
}

pub fn vec_to_se3(vec: na::Vector3<f64>) -> na::Matrix4<f64> {
    unimplemented!();

}

pub fn se3_to_vec(se3: na::Matrix4<f64>) -> na::Vector3<f64>  {
    unimplemented!();

}

pub fn adjoint() {
    //already implemented in nalgebra just call it
        unimplemented!();

}

pub fn screw_to_axis() {
    unimplemented!();
}

//TODO: MatrixExp6, MatrixLog6, 




    #[test]
    fn test_vec_to_so3() {
        let vector = na::Vector3::new(21.0,21.0,21.0);

        //need to figure out how to call from other modules
        let vec_trans = vec_to_so3(vector);
        println!("{}", vec_trans);
    }

    fn main() {
        println!("Am inside main function");
    }