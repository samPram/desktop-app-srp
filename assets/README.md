# Assets Directory

This directory contains static assets for the desktop dyno testing application.

## Directory Structure

```
assets/
├── icons/          # Application icons and UI icons
├── images/         # Images, logos, and graphics
├── fonts/          # Custom fonts (if any)
└── README.md       # This file
```

## Usage Guidelines

### Icons
- Place application icons in `icons/` directory
- Use standard formats: PNG, ICO, SVG
- Follow naming convention: `icon_name_size.format` (e.g., `app_icon_32.png`)
- Include multiple sizes for different display densities

### Images
- Place application images, logos, and graphics in `images/` directory
- Use web-optimized formats: PNG, JPEG, SVG
- Keep file sizes reasonable for desktop application

### Fonts
- Place custom fonts in `fonts/` directory if needed
- egui includes default fonts, so custom fonts are optional
- Use standard formats: TTF, OTF

## Asset Loading

Assets can be loaded in the application using:

```rust
// Example of loading an icon
let icon_bytes = include_bytes!("../assets/icons/app_icon.png");

// Example of loading an image
let image_bytes = include_bytes!("../assets/images/logo.png");
```

## Notes

- Assets are embedded into the binary at compile time using `include_bytes!`
- Keep asset files reasonably sized to avoid bloating the executable
- Consider using SVG for scalable graphics when possible
- Test assets on different display scales and themes