# Maintainer: Gerhard Schwanzer <geri@sdf.org>
pkgname=niccalc-bin
pkgver=0.9.7
pkgrel=1
pkgdesc="A tool to calculate nicotine amount needed for an e-cigarette liquid"
url="https://github.com/geri1701/niccalc"
license=("MIT")
arch=("x86_64")
provides=("niccalc")
options=("strip")
source=("https://github.com/geri1701/niccalc/releases/download/v$pkgver/niccalc-$pkgver-x86_64.tar.gz")
sha256sums=("bb954f127a977c499e6c468c5d9d1797ef970e439b6e25fc9011c6599151f5f1")

package() {
    install -Dm755 niccalc -t "$pkgdir/usr/bin/"
}
