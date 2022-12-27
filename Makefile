build:
	@cargo build --target wasm32-unknown-unknown
	@rm -rf "/home/sarah/.steam/steam/steamapps/common/The Stanley Parable Ultra Deluxe/Ferrex/Mods/ferrex_plugin.wasm"
	@mv target/wasm32-unknown-unknown/debug/ferrex_plugin.wasm "/home/sarah/.steam/steam/steamapps/common/The Stanley Parable Ultra Deluxe/Ferrex/Mods"
	