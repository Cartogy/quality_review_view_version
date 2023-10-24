extends FieldEditor
class_name SingleFieldEditor

onready var line_edit:LineEdit = $Panel/MarginContainer/HBoxContainer/PanelContainer/LineEdit

# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.

func prompt_editor(p_data, view: Row):
	data = p_data
	current_row = view;
	
	self.connect("update_view",current_row,"update_row_view")
	
	if p_data != null:
		line_edit.text = data.get_content()
	else:
		line_edit.text = ""
	
	self.popup()

func clear_data():
	data = null
	if self.is_connected("update_view",current_row,"update_row_view"):
		self.disconnect("update_view",current_row,"update_row_view")
	line_edit.text = ""
	
func cancel():
	clear_data()
	self.hide()

func build_dictionary_data() -> Dictionary:
	var d = {}
	d["id"] = data.get_id()
	d["content"] = line_edit.text
	
	return d
	


#########
# Signal functions
#########
func _on_Apply_pressed():
	data.from_dictionary(build_dictionary_data())
	apply_changes()
	clear_data()
	self.hide()


func _on_Cancel_pressed():
	cancel()


func _on_FieldEditor_popup_hide():
	clear_data()
