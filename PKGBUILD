# Maintainer: Gerhard Schwanzer <geri@sdf.org>
pkgname=niccalc-bin
pkgver=0.9.3
pkgrel=1
pkgdesc="A tool to calculate nicotine amount needed for an e-cigarette liquid"
url="https://github.com/geri1701/niccalc"
license=("MIT")
arch=("x86_64")
provides=("niccalc")
options=("strip")
source=("https://github.com/geri1701/niccalc/releases/download/v$pkgver/niccalc-$pkgver-x86_64.tar.gz")
sha256sums=("77147e659eaa909ba943967b540aab0377fe18f044e39c34d3094a52ae7d5378")

package() {
    install -Dm755 niccalc -t "$pkgdir/usr/bin/"
}
