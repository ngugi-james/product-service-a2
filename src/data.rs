use crate::model::Product;
use crate::configuration::Settings;

pub fn fetch_products(_settings: &Settings) -> Vec<Product> {
    vec![
        Product {
            id: 1,
            name: "Bose QuietComfort Ultra".to_string(),
            price: 349.99,
            description: "Immerse yourself in sound with the Bose QuietComfort Ultra wireless headphones, offering noise-canceling technology and premium audio quality.".to_string(),
            image: "/BoseQuietComfortUltra.jpg".to_string()
        },
        Product {
            id: 2,
            name: "Samsung Galaxy S24 Ultra".to_string(),
            price: 1199.99,
            description: "Experience the ultimate smartphone with the Samsung Galaxy S24 Ultra, featuring a stunning display, powerful performance, and exceptional camera capabilities.".to_string(),
            image: "/SamsungGalaxyS24Ultra.jpg".to_string()
        },
        Product {
            id: 3,
            name: "LG 32ML600M Monitor".to_string(),
            price: 249.99,
            description: "Enhance your workspace or gaming setup with the LG 32ML600M monitor, offering a Full HD display and excellent color accuracy.".to_string(),
            image: "/LG32ML600MMonitor.jpg".to_string()
        },
        Product {
            id: 4,
            name: "Playstation 5 Slim Console".to_string(),
            price: 499.99,
            description: "Level up your gaming with the Playstation 5 Slim Console, delivering lightning-fast loading times and immersive gaming experiences.".to_string(),
            image: "/PlayStation5SlimConsole.jpg".to_string()
        },
        Product {
            id: 5,
            name: "Apple Watch SE 44mm".to_string(),
            price: 279.99,
            description: "Stay connected and track your fitness goals with the Apple Watch SE 44mm, combining powerful features with a sleek design.".to_string(),
            image: "/AppleWatchSE.jpg".to_string()
        },
        Product {
            id: 6,
            name: "JBL Boombox 2 Portable".to_string(),
            price: 399.99,
            description: "Take the party anywhere with the JBL Boombox 2 Portable Bluetooth Speaker, offering powerful sound and an impressive battery life.".to_string(),
            image: "/JBLBoombox2Portable.jpg".to_string()
        }
    ]
}
