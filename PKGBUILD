# Maintainer: CallMeAlphabet
pkgname=fastannoy
pkgver=1
pkgrel=1
pkgdesc="typos that are actually annoying"
arch=('x86_64')
url="https://github.com/CallMeAlphabet/fastannoy"
license=('Apache-2.0')
depends=('gcc-libs')
makedepends=('cargo')
provides=('gti' 'gerp' 'sudp' 'cst' 'vom')
conflicts=('gti' 'gerp' 'sudp' 'cst' 'vom')
source=("fastannoy-$pkgver.tar.gz::https://github.com/CallMeAlphabet/fastannoy/archive/refs/tags/latest.tar.gz")
sha256sums=('SKIP')

# Common command typos this package intentionally shadows in /usr/bin.
# 'sl' is deliberately excluded: it's already an established, well-known
# joke package (steam locomotive) and claiming it would conflict.
typos=('gti' 'gerp' 'sudp' 'cst' 'vom')

prepare() {
    rm -rf "$srcdir/build"
    mkdir -p "$srcdir/build"
    tar -xzf "$srcdir/fastannoy-$pkgver.tar.gz" --strip-components=1 -C "$srcdir/build"
    cd "$srcdir/build"
    cargo fetch --locked --target x86_64-unknown-linux-gnu
}

build() {
    cd "$srcdir/build"
    export RUSTUP_TOOLCHAIN=stable
    export CARGO_TARGET_DIR=target
    cargo build --frozen --release
}

package() {
    cd "$srcdir/build"
    install -Dm755 "target/release/fastannoy" "$pkgdir/usr/bin/fastannoy"
    for typo in "${typos[@]}"; do
        install -Dm755 "target/release/fastannoy" "$pkgdir/usr/bin/$typo"
    done
    install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
    install -Dm644 README.md "$pkgdir/usr/share/doc/$pkgname/README.md"
}
