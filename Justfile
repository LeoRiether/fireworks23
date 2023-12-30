dev:
    wasm-pack build
    cd www/ && pnpm install && pnpm start

build:
    #!/bin/bash
    wasm-pack build
    cd www
    pnpm install
    pnpm build

deploy VERSION: build
    git add www/dist
    git commit -m "{{VERSION}}"
    git subtree push --prefix www/dist origin gh-pages
