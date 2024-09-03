// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 bytes.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.

pub use order::Order;

mod order {
    pub struct Order {
        product_name: String,
        quantity: u32,
        unit_price: u32,
    }

    impl Order {
        pub fn new(product_name: String, quantity: u32, unit_price: u32) -> Self {
            validation::check_product_name(&product_name);
            validation::check_quantity(quantity);
            validation::check_unit_price(unit_price);
            Self {
                product_name,
                quantity,
                unit_price,
            }
        }

        pub fn total(&self) -> u32 {
            self.quantity * self.unit_price
        }

        pub fn product_name(&self) -> &String {
            &self.product_name
        }

        pub fn quantity(&self) -> &u32 {
            &self.quantity
        }

        pub fn unit_price(&self) -> &u32 {
            &self.unit_price
        }

        pub fn set_product_name(&mut self, product_name: String) -> &mut Self {
            validation::check_product_name(&product_name);
            self.product_name = product_name;
            self
        }

        pub fn set_quantity(&mut self, quantity: u32) -> &mut Self {
            validation::check_quantity(quantity);
            self.quantity = quantity;
            self
        }

        pub fn set_unit_price(&mut self, unit_price: u32) -> &mut Self {
            validation::check_unit_price(unit_price);
            self.unit_price = unit_price;
            self
        }
    }

    mod validation {
        pub fn check_product_name(product_name: &String) {
            if product_name.is_empty() || product_name.len() > 300 {
                panic!("The product name can't be empty and it can't be longer than 300 bytes")
            }
        }
        pub fn check_quantity(quantity: u32) {
            if quantity == 0 {
                panic!("The quantity must be strictly greater than zero")
            }
        }
        pub fn check_unit_price(unit_price: u32) {
            if unit_price == 0 {
                panic!("The unit price is in cents and must be strictly greater than zero")
            }
        }
    }
}
