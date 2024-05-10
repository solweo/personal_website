default:
  @just --list

# Generating necessary style files
# 1. Hash each CSS class and consolidate all the individual files into a single one
# 2. Resolve PostCSS features into eventual style file
# Building and serving this site
dev:
  stylance . --watch & \
  postcss ./target/stylance-output/bundle.css --use postcss-mixins --use autoprefixer --use postcss-selector-matches --use postcss-selector-not --use postcss-custom-media --use postcss-media-minmax --use postcss-nesting --use postcss-custom-selectors --no-map --output ./index.css --watch --verbose & \
  trunk serve --open 

build-styles:
  stylance .
  postcss ./target/stylance-output/bundle.css --use postcss-mixins --use autoprefixer --use postcss-selector-matches --use postcss-selector-not --use postcss-custom-media --use postcss-media-minmax --use postcss-nesting --use postcss-custom-selectors --no-map --output ./index.css