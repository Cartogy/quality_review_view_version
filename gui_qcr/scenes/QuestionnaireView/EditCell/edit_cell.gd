extends ScrollContainer

signal update_form(form_id, text)

onready var line_edit = $LineEdit
var updateable: bool
var id

# Called when the node enters the scene tree for the first time.
func _ready():
	self.rect_min_size.y = 50
	line_edit.rect_min_size = self.rect_min_size

func set_text(text: String):
	line_edit.text = text

func set_update_questionnaire(status: bool):
	updateable = status


func _on_LineEdit_text_changed(new_text):
	emit_signal("update_form", id, new_text)
