extends Control

enum state { EMPTY, EDITING}
var current_state = state.EMPTY

var unit_form_scene = preload("res://scenes/UnitForm/UnitForm.tscn")

onready var vbox = $VBoxContainer

func _ready():
	# Create space between the elements
	vbox.add_constant_override("separation", 60)
	edit_forms([[0,"section text","question text"],[1,"section 1 text","question 2 text"]])
	
func edit_forms(forms: Array):
	if current_state == state.EDITING:
		printerr("Currently editing a form")
	else:
		for form in forms:
			var form_id = form[0]
			var section_text = form[1]
			var question_text = form[2]
			
			var unit_form = build_unit_form(form_id, section_text, question_text)
			vbox.add_child(unit_form)
		
		current_state = state.EDITING
		
func build_unit_form(form_id: int, section_text: String, question_text: String) -> UnitForm:
	var unit_form = unit_form_scene.instance()
	unit_form.id = form_id
	unit_form.question = question_text
	unit_form.section = section_text
	
	return unit_form
