extends Control
class_name UnitForm

var id: int
var question: String
var section: String

onready var section_label = $HBoxContainer/CenterContainer/SectionLabel
onready var question_label = $HBoxContainer/MarginContainer/QuestionLabel
onready var status_options = $HBoxContainer/MarginContainer3/OptionButton
onready var notes = $HBoxContainer/MarginContainer2/LineEdit


# Called when the node enters the scene tree for the first time.
func _ready():
	set_options()
	set_section_text(section)
	set_question_text(question)
	pass # Replace with function body.

func set_options():
	status_options.clear()
	status_options.add_item("Ok", 0)
	status_options.add_item("NO", 1)
	status_options.add_item("N/A", 2)
	status_options.select(2)

func set_id(p_id: int):
	id = p_id
	
func set_section_text(p_text: String):
	section_label.text = p_text
	
func set_question_text(p_text: String):
	question_label.text = p_text 
