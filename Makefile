include $(TOPDIR)/rules.mk

PKG_NAME:=rust_mqtt
PKG_VERSION:=0.1.0
PKG_RELEASE:=1

PKG_SOURCE_PROTO:=git
PKG_SOURCE_URL:=https://github.com/svet-b/openwrt-rust-mqtt.git
PKG_SOURCE_VERSION:=b98880a2f0da0619d0439d5d8dda949cf1c9cd44

PKG_BUILD_PARALLEL:=1
PKG_INSTALL:=1
PKG_BUILD_DEPENDS:=rust/host python3/host

# TARGET_CFLAGS += -D_GNU_SOURCE
# TARGET_CXXFLAGS += -latomic
# TARGET_LDFLAGS += -latomic

include $(INCLUDE_DIR)/package.mk
include $(INCLUDE_DIR)/nls.mk
include $(TOPDIR)/package/feeds/packages/rust/rust_targets.mk

CONFIGURE_VARS += \
	CARGO_HOME=$(CARGO_HOME) \
	ac_cv_path_CARGO="$(CARGO_HOME)/bin/cargo" \
	ac_cv_path_RUSTC="$(CARGO_HOME)/bin/rustc"
#	RUSTFLAGS="-C linker=$(TARGET_CC_NOCACHE) -C ar=$(TARGET_AR)"

CONFIGURE_ARGS += \
	--host=$(RUSTC_TARGET_ARCH) \
	--build=$(RUSTC_HOST_ARCH) \
	--with-gnu-ld

# define Build/Prepare
# 	$(call Build/Prepare/Default)
# 	$(CONFIGURE_VARS) cargo install cbindgen
# 	cd $(PKG_BUILD_DIR) && $(CONFIGURE_VARS) ./autogen.sh
# endef

define Package/rust_mqtt
	SECTION:=examples
	CATEGORY:=Examples
	TITLE:=Rust MQTT
endef

define Package/rust_mqtt/description
  Simple MQTT program in Rust for OpenWRT
endef

define Package/suricata6/install
	$(INSTALL_DIR) $(1)/usr/bin
	$(INSTALL_BIN) $(PKG_BUILD_DIR)/rust_mqtt $(1)/usr/bin
endef

$(eval $(call BuildPackage,rust_mqtt))