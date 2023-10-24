extends FieldEditor
class_name SpecificationEditor

onready var line_edit = $Panel/MarginContainer/HBoxContainer/PanelContainer/LineEdit

onready var section_options = $Panel/MarginContainer/HBoxContainer/Sections

var db: DatabaseAPI

func _ready():
	pass

# [SectionData]
func fill_sections(db: DatabaseAPI):
	var sections = db.all_sections()
	for section in sections:
		var id = section.get_id()
		var content = section.get_content()
		
		section_options.add_item(content, id)

func prompt_editor(p_data, view: Row):
	data = p_data
	current_row = view
	
	fill_sections(db)
	
	self.connect("update_view",current_row,"update_row_view")
	
	var spec_data: SpecificationData = data
	if spec_data == null:
		line_edit.text = ""
	else:
		line_edit.text = spec_data.content
		
	popup()

func clear_data():
	self.data = null
	if self.is_connected("update_view",current_row,"update_row_view"):
		self.disconnect("update_view",current_row,"update_row_view")
		
	section_options.clear()
		
	line_edit.text = ""
	
func cancel():
	clear_data()
	self.hide()

func _on_Apply_pressed():
	data.from_dictionary(build_dictionary_data())
	self.apply_changes()
	clear_data()
	self.hide()

func _on_Cancel_pressed():
	self.cancel()

func _on_SpecificationEditor_popup_hide():
	self.clear_data()

func build_dictionary_data() -> Dictionary:
	var section_id = section_options.get_selected_id()
	print_debug("section id" + str(section_id))
	
	# Since option button starts at zero, and database id starts at 1, we have to 
	# subtract 1 to get the appropriate section.
	var section_name = section_options.get_item_text(section_id-1)
	
	var d = {
		"id": data.get_id(),
		"content": line_edit.text,
		"section_id": section_id,
		"section_name": section_name
	}
	
	return d
	
