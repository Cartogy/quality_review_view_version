
gd-lib:
	cp gdnative_rust_qcr/target/debug/libquestionnaire_godot.so gui_qcr/bin/

lib:
	cp gdnative_rust_qcr/target/debug/libquestionnaire_godot.so gui_qcr/bin/
	cp gdnative_rust_qcr/csv_database_reader/qcr_database.db gui_qcr/database/qcr_database.db
