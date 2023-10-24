extends Control

signal update_questionnaire_notes(form_id, text)
signal update_questionnaire_status(form_id, status)


var edit_cell_packed = preload("res://scenes/QuestionnaireView/EditCell/EditCell.tscn")
var option_status_button = preload("res://scenes/QuestionnaireView/OptionStatusButton/OptionStatusButton.tscn")

onready var grid = $PanelContainer/ScrollContainer/GridContainer
onready var exit_button = $VSplitContainer/Button

func _ready():
	grid.columns = 4

func build_questionnaire(forms: Array):
	for form in forms:
		var section_name = form[1]
		var specification_content = form[2]
		var form_id = form[0]
		
		add_unit_form_view(form_id, section_name, specification_content, 2)

# Adds the unit form row.
func add_unit_form_view(form_id: int, section_name: String, specification_content: String, status_id: int):
	var section_cell = edit_cell_packed.instance()
	var specification_cell = edit_cell_packed.instance()
	
	grid.add_child(section_cell)
	grid.add_child(specification_cell)
	
	section_cell.set_text(section_name)
	specification_cell.set_text(specification_content)
	
	var notes = edit_cell_packed.instance()
	notes.set_update_questionnaire(true)
	notes.id = form_id
	grid.add_child(notes)
	
	notes.connect("update_form", self,"update_form_notes")
	
	var options: OptionStatusButton = build_options(form_id)
	grid.add_child(options)
	options.selected = 2
	options.connect("update_form_status", self, "update_form_status")
	
	
func update_form_notes(form_id: int, text: String):
	emit_signal("update_questionnaire_notes", form_id, text)
	
func update_form_status(form_id: int, status: int):
	emit_signal("update_questionnaire_status", form_id, status)
	

func build_options(id: int) -> OptionStatusButton:
	var option_button = option_status_button.instance()
	
	option_button.add_item("Ok", 0)
	option_button.add_item("NO", 1)
	option_button.add_item("N/A", 2)
	option_button.unit_form_id = id
	
	return option_button


