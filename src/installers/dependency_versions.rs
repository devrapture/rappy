use std::collections::HashMap;

pub fn get_dependency_version_map() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    // TailwindCSS
    map.insert("tailwindcss", "^3.4.3");
    map.insert("postcss", "^8.4.39");
    map.insert("prettier", "^3.3.2");
    map.insert("prettier-plugin-tailwindcss", "^0.6.5");

    map
}
