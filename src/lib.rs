pub mod client; // this bascically tell rust to look for the file client.rs in project dir

pub mod network;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    // use super::client;

    #[test]
    fn it_works() {
        // you can do this like this
       // super::client::connect(); 
        
        // or like this, but you need user super::client; 
        // client::connect(); 
       
        // or like this, think of double colons as going one dir up .. in folder structures in
        // terminal 
        crate::client::connect();
    }
}

// Privacy example
mod outermost {
    pub fn middle_function() {

    }
    pub fn middle_secrect_function() {

    }
    pub mod inside {
        pub fn inner_function(){
            crate::outermost::middle_secrect_function()
        }
        fn secret_function() {

        }
    }
}

fn try_me() {
    outermost::middle_function();
    outermost::middle_secrect_function();
    outermost::inside::inner_function();
    // outermost::inside::secret_function();
}
