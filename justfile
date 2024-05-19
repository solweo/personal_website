default:
  @just --list

# Generating necessary style files
# 1. Consolidate all the individual CSS files into a single one per package
# 2. Resolve PostCSS features into eventual style files
# Building and serving this site
dev:
  stylance --watch front & \
  stylance --watch ui_kit & \
  postcss ./target/stylance-output/* --use postcss-mixins --use autoprefixer --use postcss-selector-matches --use postcss-selector-not --use postcss-custom-media --use postcss-media-minmax --use postcss-nesting --use postcss-custom-selectors --no-map --dir ./assets/ --watch --verbose & \
  cargo leptos watch 

build-styles:
  stylance --watch front
  stylance --watch ui_kit
  postcss ./target/stylance-output/* --use postcss-mixins --use autoprefixer --use postcss-selector-matches --use postcss-selector-not --use postcss-custom-media --use postcss-media-minmax --use postcss-nesting --use postcss-custom-selectors --no-map --dir ./assets/ --watch --verbose