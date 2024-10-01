default:
  @just --list

dev:
  echo 'Generating necessary style files' & \
  stylance --watch front & \
  stylance --watch ui_kit & \
  postcss ./target/stylance-output/* --use postcss-mixins --use autoprefixer --use postcss-selector-matches --use postcss-selector-not --use postcss-custom-media --use postcss-media-minmax --use postcss-nesting --use postcss-custom-selectors --no-map --dir ./assets/ --watch --verbose & \
  echo 'Building and serving this site in dev mode' & \
  cargo leptos watch 

serve:
  just build-styles & \
  echo 'Building and serving this site in release mode' & \
  cargo leptos serve -r

build-styles:
  echo 'Generating necessary style files' & \
  echo '1. Consolidate all the individual CSS files into a single one per package' & \
  stylance front & \
  stylance ui_kit & \
  echo '2. Resolve PostCSS features into eventual style files' & \
  postcss ./target/stylance-output/* --use postcss-mixins --use autoprefixer --use postcss-selector-matches --use postcss-selector-not --use postcss-custom-media --use postcss-media-minmax --use postcss-nesting --use postcss-custom-selectors --no-map --dir ./assets/ --verbose & \
