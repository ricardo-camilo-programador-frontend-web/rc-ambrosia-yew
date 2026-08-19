#!/bin/bash
set -e
echo "Building rc-ambrosia-yew..."

# Build WASM
cargo build --target wasm32-unknown-unknown --release

# Find wasm file  
WASM_FILE=$(find target/wasm32-unknown-unknown/release -maxdepth 1 -name "*.wasm" | head -1)
echo "WASM: $(ls -lh $WASM_FILE)"

# Clean dist
rm -rf dist
mkdir -p dist/images

# Run wasm-bindgen directly to dist/
wasm-bindgen "$WASM_FILE" --target web --out-dir dist --out-name rc-ambrosia-yew

# Copy public assets
cp -r public/* dist/ 2>/dev/null || true
cp -r public/images/* dist/images/ 2>/dev/null || true
rm -f dist/ambrosia-*.jpg dist/images/ATTRIBUTION.md 2>/dev/null || true  # Remove from root if copied
cp -r public/images/ATTRIBUTION.md dist/images/ 2>/dev/null || true

# Copy CSS
for css in tokens base editorial components animations responsive tailwind; do
    cp src/styles/${css}.css dist/ 2>/dev/null || true
done

# Generate index.html
JS_FILE="rc-ambrosia-yew.js"
WASM_FILE_FINAL="rc-ambrosia-yew_bg.wasm"

cat > dist/index.html << 'HTMLEOF'
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>Ambrosia — Food of the Gods</title>
    <meta name="description" content="Discover the Ambrosia apple — a chance seedling from British Columbia, named after the food of the gods in Greek mythology." />
    <meta name="theme-color" content="#C0392B" />
    <meta property="og:type" content="article" />
    <meta property="og:title" content="Ambrosia — Food of the Gods" />
    <meta property="og:description" content="A chance seedling from British Columbia, named after the mythical food that granted immortality." />
    <meta property="og:image" content="/og-image.png" />
    <meta name="twitter:card" content="summary_large_image" />
    <meta name="twitter:title" content="Ambrosia — Food of the Gods" />
    <meta name="twitter:description" content="An interactive editorial article about the Ambrosia apple variety." />
    <script type="application/ld+json">
    {"@context":"https://schema.org","@type":"Article","headline":"Ambrosia — Food of the Gods","description":"An interactive editorial about the Ambrosia apple variety.","author":{"@type":"Person","name":"Ricardo Camilo"},"publisher":{"@type":"Person","name":"Ricardo Camilo"},"about":"Ambrosia Apple (Malus domestica)"}
    </script>
    <script src="https://cdn.counter.dev/script.js" data-id="f30df6f3-776d-4154-959d-0210ac8a8325" data-utcoffset="-3"></script>
    <link rel="preconnect" href="https://fonts.googleapis.com" />
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin />
    <link href="https://fonts.googleapis.com/css2?family=Cormorant+Garamond:wght@400;500;600;700&display=swap" rel="stylesheet" />
    <link rel="stylesheet" href="tokens.css" />
    <link rel="stylesheet" href="base.css" />
    <link rel="stylesheet" href="editorial.css" />
    <link rel="stylesheet" href="components.css" />
    <link rel="stylesheet" href="animations.css" />
    <link rel="stylesheet" href="responsive.css" />
    <link rel="stylesheet" href="tailwind.css" />
    <link rel="manifest" href="manifest.json" />
    <link rel="icon" type="image/svg+xml" href="favicon.svg" />
    <link rel="apple-touch-icon" href="icon-192.png" />
    <script type="module" src="rc-ambrosia-yew.js"></script>
</head>
<body></body>
</html>
HTMLEOF

echo "✅ Build complete!"
echo "Dist files:"
find dist -type f -exec ls -lh {} \; | sort -k9
