struct Solution;

impl Solution {
    pub fn largest_triangle_area(points : Vec<Vec<i32>>) -> f64 {
        let mut max_area = 0.0;
        for i in 0..points.len()-2 {
            for j in i+1..points.len()-1 {
                for k in j+1..points.len() {
                    max_area = f64::max(max_area, Self::area(&points[i],&points[j],&points[k]));
                }
            }
        }
        max_area
    }
    
    fn dis(x: &[i32] , y: &[i32] ) -> f64 {
        let dx = (x[0] - y[0]) as f64;
        let dy = (x[1] - y[1]) as f64;
        (dx * dx + dy * dy).sqrt()
    }

    fn area(point1 : &[i32], point2 : &[i32], point3 : &[i32]) -> f64 {
        let a = Self::dis(point1,point2);
        let b = Self::dis(point3,point1);
        let c = Self::dis(point2,point3);
        let p = (a+b+c)/2.0;
        (p*(p-a)*(p-b)*(p-c)).sqrt()
    }
    
}


fn main() {
    let res = Solution::largest_triangle_area(vec![vec![0,0],vec![0,1],vec![1,0],vec![0,2],vec![2,0]]);
    println!("{}", res);
}