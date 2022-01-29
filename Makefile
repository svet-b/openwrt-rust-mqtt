include $(TOPDIR)/rules.mk

PKG_NAME:=rust_mqtt
PKG_VERSION:=0.1.0
PKG_RELEASE:=1

PKG_SOURCE_PROTO:=git
PKG_SOURCE_URL:=https://github.com/svet-b/openwrt-rust-mqtt.git
PKG_SOURCE_VERSION:=2b771b3fd783a6d791d33e1234510fcbfd0082b3

PKG_BUILD_DEPENDS:=rust/host

include $(INCLUDE_DIR)/package.mk

CARGO_HOME := $(STAGING_DIR_HOST)/.cargo
RUSTFLAGS="-C linker=$(TARGET_CC_NOCACHE) -C ar=$(TARGET_AR)"

CONFIGURE_VARS += \
        CARGO_HOME=$(CARGO_HOME) \
        RUSTFLAGS=$(RUSTFLAGS)

define Build/Compile
        # Setting LD_LIBRARY_PATH appears necessary in order for `rustc` to find the LLVM library
        # if it is run indirectly by custom crate build scripts compilation steps (such as the one for cmake)
        cd $(PKG_BUILD_DIR) && \
          LD_LIBRARY_PATH=$(STAGING_DIR_HOST)/lib \
          $(CONFIGURE_VARS) cargo build --release --target=$(REAL_GNU_TARGET_NAME)
endef

define Package/rust_mqtt
        SECTION:=examples
        CATEGORY:=Examples
        TITLE:=Rust MQTT
endef

define Package/rust_mqtt/description
  Simple MQTT program in Rust for OpenWRT
endef

define Package/rust_mqtt/install
        $(INSTALL_DIR) $(1)/usr/bin
        $(INSTALL_BIN) $(PKG_BUILD_DIR)/target/$(REAL_GNU_TARGET_NAME)/release/rust_mqtt $(1)/usr/bin
endef

$(eval $(call BuildPackage,rust_mqtt))
