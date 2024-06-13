use polyfmt::{self, println, spacer};

// These tests aren't real tests, I just eyeball things to see if they work.
// Maybe I'll write real tests, maybe I wont. Shut-up.
#[test]
fn test_line_wrapping() {
    let options = polyfmt::Options {
        debug: true,
        max_line_length: 40,
        padding: 1,
    };

    let fmt = polyfmt::new(polyfmt::Format::Plain, options);
    polyfmt::set_global_formatter(fmt);

    std::println!("========== 40 chars; no wrapping");
    println!("Fast frogs leap over every lazy dogs to."); // 40 chars
    std::println!("\n");

    std::println!("========== 40 chars; should wrap one word");
    println!("Fast frogs leap over every lazy dogs to food."); // 44 chars; should wrap.
    std::println!("\n");

    // 86 characters should have two wrapped lines and then a little bit at the end.
    std::println!("========== 86 chars; two wrapped lines");
    println!(
        "Every summer, we visit the beautiful shores of Lake Tahoe with our family and friends."
    );
    std::println!("\n");

    // 86 characters should have two wrapped lines and then a little bit at the end.
    std::println!("========== 86 chars; two wrapped lines");
    println!(
        "Every summer, we visit the beautiful shores of Lake Tahoe with our family and friends."
    );
    std::println!("\n");

    std::println!("========== ~500 chars; multiple wrapped lines");
    println!("Throughout the bustling streets of the city, people from all walks of life come together, creating a vibrant tapestry of cultures and experiences. From the aromatic coffee shops that line the narrow cobblestone alleys, where poets and painters find their muse, to the bustling markets filled with colorful produce and exotic spices, the city offers a rich palette of sights, sounds, and flavors. Every corner tells a story, every face a unique narrative of dreams and struggles, all interwoven into the fabric of this urban landscape, making it a place of endless possibilities and enduring charm.");
    std::println!("\n");

    // 86 characters should have two wrapped lines and then a little bit at the end.
    std::println!("========== 86 chars twice; proper spacing can be handled between strings");
    println!(
        "Every summer, we visit the beautiful shores of Lake Tahoe with our family and friends."
    );
    println!("");
    println!();
    println!(
        "Every summer, we visit the beautiful shores of Lake Tahoe with our family and friends."
    );
    std::println!("\n");
}

#[test]
fn test_tree_formatting() {
    let options = polyfmt::Options {
        debug: true,
        max_line_length: 40,
        padding: 1,
    };

    let fmt = polyfmt::new(polyfmt::Format::Tree, options);
    polyfmt::set_global_formatter(fmt);

    println!("Start of the tree.");

    println!("New node in tree should not wrap.");

    println!("Another node in tree should wrap exactly once");

    println!("Another node in the tree, should wrap twice because it's somewhat a long sentence");

    println!("This should be printed and then we'll test out leaving some space in the middle");
    println!();
    println!("");
    println!("Another regular node put here to check that the one above acts correctly.");
    spacer!();
    println!("Checking that spacers work");
}
