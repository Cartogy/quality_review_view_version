extends PopupPanel
class_name FieldEditor

# Interface class to deal with editable fields in DatabaseView
var current_row: Row

signal update_database(p_raw_data)
signal update_view(p_data)

var data: QcrData

func get_data() -> QcrData:
	return data

func apply_changes():
	emit_signal("update_database", data)
	emit_signal("update_view",data)
	
func close_prompt():
	clear_data()
	self.hide()

#######
# Abstract functions

func clear_data():
	pass
	
func prompt_editor(p_data, view: Row):
	# 1. pop-up panel
	# 2. Set current information
	pass
	
func cancel():
	pass

func build_dictionary_data() -> Dictionary:
	return {}
	
