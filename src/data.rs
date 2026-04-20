// used ChatGPT to generate new product name and descriptions
// images are from the Internet (AI images are scary😱)

use crate::model::Product;
use crate::configuration::Settings;

pub fn fetch_products(_settings: &Settings) -> Vec<Product> {
    vec![
        Product {
            id: 1,
            name: "Anker PowerCore 10000 Portable Charger".to_string(),
            price: 29.99,
            description: "Compact 10,000mAh power bank with fast charging support. Perfect for charging smartphones on the go.".to_string(),
            image: "/placeholder.png".to_string()
        },
        Product {
            id: 2,
            name: "Logitech M185 Wireless Mouse".to_string(),
            price: 19.99,
            description: "Reliable wireless mouse with plug-and-play USB receiver and long battery life.".to_string(),
            image: "/placeholder.png".to_string()
        },
        Product {
            id: 3,
            name: "JBL Go 3 Bluetooth Speaker".to_string(),
            price: 49.99,
            description: "Portable waterproof Bluetooth speaker with surprisingly powerful sound and bold design.".to_string(),
            image: "/placeholder.png".to_string()
        },
        Product {
            id: 4,
            name: "Amazon Fire TV Stick Lite".to_string(),
            price: 39.99,
            description: "Stream your favorite content in HD with this compact and easy-to-use streaming device.".to_string(),
            image: "/placeholder.png".to_string()
        },
        Product {
            id: 5,
            name: "Razer Kraken X Gaming Headset".to_string(),
            price: 59.99,
            description: "Lightweight gaming headset with immersive 7.1 surround sound and comfortable ear cushions.".to_string(),
            image: "/placeholder.png".to_string()
        },
        Product {
            id: 6,
            name: "SanDisk 128GB Ultra USB 3.0 Flash Drive".to_string(),
            price: 24.99,
            description: "High-speed USB flash drive for quickly transferring files, photos, and videos.".to_string(),
            image: "/placeholder.png".to_string()
        },
        Product {
            id: 7,
            name: "TP-Link AC1200 WiFi Router".to_string(),
            price: 69.99,
            description: "Dual-band WiFi router offering fast speeds and stable connectivity for home networks.".to_string(),
            image: "/placeholder.png".to_string()
        },
        Product {
            id: 8,
            name: "Apple EarPods (3.5mm Headphone Jack)".to_string(),
            price: 25.00,
            description: "Classic wired earbuds with built-in remote and microphone for calls and music.".to_string(),
            image: "/placeholder.png".to_string()
        },
        Product {
            id: 9,
            name: "Belkin USB-C to HDMI Adapter".to_string(),
            price: 34.99,
            description: "Connect your USB-C laptop or tablet to an HDMI display for presentations or streaming.".to_string(),
            image: "/placeholder.png".to_string()
        },
        Product {
            id: 10,
            name: "Philips Hue White Smart Bulb (Single Pack)".to_string(),
            price: 29.99,
            description: "Smart LED bulb controllable via app or voice assistant. Great for smart home setups.".to_string(),
            image: "/placeholder.png".to_string()
        }
    ]
}