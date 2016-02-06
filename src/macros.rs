#[macro_export]
macro_rules! describe {
    ($description:expr, $block:expr) => {
        describe($description, source_location!(), $block)
    };

    ($eg:expr, $description:expr, $block:expr) => {
        $eg.describe($description, source_location!(), $block)
    };
}

#[macro_export]
macro_rules! it {
    ($eg:expr, $description:expr, $block:expr) => {
        $eg.it($description, source_location!(), $block)
    }
}
