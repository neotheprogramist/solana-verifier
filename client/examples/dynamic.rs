use utils::BidirectionalStack;
use verifier::state::BidirectionalStackAccount;

// Define animal type tags as constants
const DOG_TAG: u8 = 1;
const CAT_TAG: u8 = 2;

// Define a trait and two implementors
trait Executable: Sized {
    fn execute(&mut self);
    fn type_tag(&self) -> u8;

    /// Cast a slice to an immutable reference of Self
    fn cast(slice: &[u8]) -> &Self {
        assert_eq!(slice.len(), std::mem::size_of::<Self>());
        unsafe { &*(slice.as_ptr() as *const Self) }
    }

    /// Cast a mutable slice to a mutable reference of Self
    fn cast_mut(slice: &mut [u8]) -> &mut Self {
        assert_eq!(slice.len(), std::mem::size_of::<Self>());
        unsafe { &mut *(slice.as_mut_ptr() as *mut Self) }
    }

    fn as_bytes(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                (self as *const Self) as *const u8,
                std::mem::size_of::<Self>(),
            )
        }
    }

    fn as_bytes_mut(&mut self) -> &mut [u8] {
        unsafe {
            std::slice::from_raw_parts_mut(
                (self as *mut Self) as *mut u8,
                std::mem::size_of::<Self>(),
            )
        }
    }
}

#[repr(C)]
struct Dog {
    name: [u8; 32], // Fixed-size array for name
}

#[repr(C)]
struct Cat {
    color: [u8; 32], // Fixed-size array for color
}

impl Dog {
    fn new(name: &str) -> Self {
        let mut name_bytes = [0u8; 32];
        let bytes = name.as_bytes();
        let len = std::cmp::min(bytes.len(), 32);
        name_bytes[..len].copy_from_slice(&bytes[..len]);
        Self { name: name_bytes }
    }

    fn get_name(&self) -> String {
        let null_pos = self
            .name
            .iter()
            .position(|&b| b == 0)
            .unwrap_or(self.name.len());
        String::from_utf8_lossy(&self.name[..null_pos]).to_string()
    }
}

impl Cat {
    fn new(color: &str) -> Self {
        let mut color_bytes = [0u8; 32];
        let bytes = color.as_bytes();
        let len = std::cmp::min(bytes.len(), 32);
        color_bytes[..len].copy_from_slice(&bytes[..len]);
        Self { color: color_bytes }
    }

    fn get_color(&self) -> String {
        let null_pos = self
            .color
            .iter()
            .position(|&b| b == 0)
            .unwrap_or(self.color.len());
        String::from_utf8_lossy(&self.color[..null_pos]).to_string()
    }
}

impl Executable for Dog {
    fn execute(&mut self) {
        println!("Woof! I'm {}.", self.get_name());
    }

    fn type_tag(&self) -> u8 {
        DOG_TAG
    }
}

impl Executable for Cat {
    fn execute(&mut self) {
        println!("Meow! I am {}.", self.get_color());
    }

    fn type_tag(&self) -> u8 {
        CAT_TAG
    }
}

fn execute(stack: &mut BidirectionalStackAccount) {
    let data = stack.borrow_mut_front();
    match data[0] {
        DOG_TAG => {
            let dog = Dog::cast_mut(&mut data[1..]);
            dog.execute();
        }
        CAT_TAG => {
            let cat = Cat::cast_mut(&mut data[1..]);
            cat.execute();
        }
        _ => {
            panic!("Unknown tag: {}", data[0]);
        }
    }
    stack.pop_front();
}

fn main() {
    let dog: Dog = Dog::new("Buddy");
    let cat = Cat::new("Tabby");

    let mut stack = BidirectionalStackAccount::default();

    // Push dog to the stack
    let mut serialized_dog = Vec::new();
    serialized_dog.push(dog.type_tag());
    serialized_dog.extend_from_slice(dog.as_bytes());
    stack.push_front(&serialized_dog).unwrap();

    // Push cat to the stack
    let mut serialized_cat = Vec::new();
    serialized_cat.push(cat.type_tag());
    serialized_cat.extend_from_slice(cat.as_bytes());
    stack.push_front(&serialized_cat).unwrap();

    // Retrieve and execute
    execute(&mut stack);

    execute(&mut stack);
}
