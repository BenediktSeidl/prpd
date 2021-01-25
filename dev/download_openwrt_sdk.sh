set -e
wget -O sdk.tar.xz https://downloads.openwrt.org/releases/19.07.6/targets/ath79/generic/openwrt-sdk-19.07.6-ath79-generic_gcc-7.5.0_musl.Linux-x86_64.tar.xz
xz -d sdk.tar.xz
tar -xf sdk.tar
rm sdk.tar
