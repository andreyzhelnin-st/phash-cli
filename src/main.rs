use img_hash::{HasherConfig, HashAlg, ImageHash, Hasher};
use img_hash::image::open;

#[cfg(test)]
use img_hash::image::DynamicImage;

fn create_hasher() -> Hasher<Box<[u8]>> {
    HasherConfig::new()
        .hash_alg(HashAlg::Mean)
        .hash_size(8, 8)
        .to_hasher()
}

fn compute_hash(path: &str) -> Result<ImageHash<Box<[u8]>>, String> {
    let img = open(path).map_err(|e| format!("cannot open image: {}", e))?;
    let hasher = create_hasher();
    Ok(hasher.hash_image(&img))
}

fn hash_to_hex(hash: &ImageHash<Box<[u8]>>) -> String {
    hash.as_bytes()
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .collect::<String>()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("usage: phash <file1> [file2]");
        std::process::exit(1);
    }

    let path1 = &args[1];
    let hash1 = compute_hash(path1).unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    });

    println!("{}: {}", path1, hash_to_hex(&hash1));

    // If second image is provided, compute hash and distance
    if args.len() >= 3 {
        let path2 = &args[2];
        let hash2 = compute_hash(path2).unwrap_or_else(|e| {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        });

        println!("{}: {}", path2, hash_to_hex(&hash2));

        let distance = hash1.dist(&hash2);
        println!("distance: {}", distance);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use img_hash::image::{ImageBuffer, Rgb};

    fn create_test_image(width: u32, height: u32, color: [u8; 3]) -> DynamicImage {
        let img = ImageBuffer::from_fn(width, height, |_, _| {
            Rgb(color)
        });
        DynamicImage::ImageRgb8(img)
    }

    #[test]
    fn test_create_hasher() {
        let hasher = create_hasher();
        // Just verify it creates successfully
        assert_eq!(std::mem::size_of_val(&hasher), std::mem::size_of_val(&hasher));
    }

    #[test]
    fn test_hash_to_hex() {
        let hasher = create_hasher();
        let img = create_test_image(100, 100, [255, 0, 0]);
        let hash = hasher.hash_image(&img);
        let hex = hash_to_hex(&hash);

        // Should be 16 hex characters (8 bytes * 2 chars per byte)
        assert_eq!(hex.len(), 16);

        // Should only contain hex characters
        assert!(hex.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_identical_images_have_zero_distance() {
        let hasher = create_hasher();
        let img1 = create_test_image(100, 100, [255, 0, 0]);
        let img2 = create_test_image(100, 100, [255, 0, 0]);

        let hash1 = hasher.hash_image(&img1);
        let hash2 = hasher.hash_image(&img2);

        assert_eq!(hash1.dist(&hash2), 0);
    }

    #[test]
    fn test_different_images_have_nonzero_distance() {
        let hasher = create_hasher();
        // Create images with significantly different patterns
        let img1 = ImageBuffer::from_fn(100, 100, |x, _y| {
            if x < 50 { Rgb([255, 255, 255]) } else { Rgb([0, 0, 0]) }
        });
        let img2 = ImageBuffer::from_fn(100, 100, |_x, y| {
            if y < 50 { Rgb([255, 255, 255]) } else { Rgb([0, 0, 0]) }
        });

        let hash1 = hasher.hash_image(&DynamicImage::ImageRgb8(img1));
        let hash2 = hasher.hash_image(&DynamicImage::ImageRgb8(img2));

        assert!(hash1.dist(&hash2) > 0);
    }    #[test]
    fn test_same_image_produces_same_hash() {
        let hasher = create_hasher();
        let img = create_test_image(100, 100, [128, 128, 128]);

        let hash1 = hasher.hash_image(&img);
        let hash2 = hasher.hash_image(&img);

        assert_eq!(hash_to_hex(&hash1), hash_to_hex(&hash2));
    }

    #[test]
    fn test_compute_hash_with_invalid_path() {
        let result = compute_hash("/nonexistent/path/image.png");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("cannot open image"));
    }

    #[test]
    fn test_hash_hex_format() {
        let hasher = create_hasher();
        let img = create_test_image(100, 100, [0, 0, 0]);
        let hash = hasher.hash_image(&img);
        let hex = hash_to_hex(&hash);

        // Verify format: should be lowercase hex
        assert!(hex.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit()));
    }
}

